//! # Load RISC-U ELF64 files

use crate::{decode, DecodingError, Instruction};
use byteorder::{ByteOrder, LittleEndian};
use goblin::elf::{program_header::PT_LOAD, Elf};
use std::{fs, mem::size_of, path::Path};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct ProgramSegment<T> {
    pub address: u64,
    pub content: Vec<T>,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub code: ProgramSegment<u8>,
    pub data: ProgramSegment<u8>,
}

#[derive(Clone, Debug)]
pub struct DecodedProgram {
    pub code: ProgramSegment<Instruction>,
    pub data: ProgramSegment<u64>,
}

#[derive(Error, Debug)]
pub enum ElfLoaderError {
    #[error("Error while reading file: {0}")]
    CouldNotReadFile(std::io::Error),

    #[error("Error while parsing ELF: {0}")]
    InvalidElf(goblin::error::Error),

    #[error("ELF is not a valid RISC-U ELF file: {0}")]
    InvalidRiscu(&'static str),

    #[error("Failure during decode: {0:?}")]
    DecodingError(DecodingError),
}

pub fn load_object_file<P>(object_file: P) -> Result<Program, ElfLoaderError>
where
    P: AsRef<Path>,
{
    load_elf_file(object_file, |p| Ok(copy_segments(p)))
}

pub fn load_and_decode_object_file<P>(object_file: P) -> Result<DecodedProgram, ElfLoaderError>
where
    P: AsRef<Path>,
{
    load_elf_file(object_file, copy_and_decode_segments)
}

fn load_elf_file<P, F, R>(object_file: P, collect: F) -> Result<R, ElfLoaderError>
where
    P: AsRef<Path>,
    F: Fn([(u64, &[u8]); 2]) -> Result<R, ElfLoaderError>,
    R: Sized,
{
    fs::read(object_file)
        .map_err(ElfLoaderError::CouldNotReadFile)
        .and_then(|buffer| {
            Elf::parse(&buffer)
                .map_err(ElfLoaderError::InvalidElf)
                .and_then(|elf| extract_program_info(&buffer, &elf).and_then(collect))
        })
}

fn extract_program_info<'a>(
    raw: &'a [u8],
    elf: &Elf,
) -> Result<[(u64, &'a [u8]); 2], ElfLoaderError> {
    if elf.is_lib || !elf.is_64 || !elf.little_endian {
        return Err(ElfLoaderError::InvalidRiscu(
            "has to be an executable, 64bit, static, little endian binary",
        ));
    }

    let mut ph_iter = elf
        .program_headers
        .as_slice()
        .iter()
        .filter(|ph| ph.p_type == PT_LOAD);

    if elf.header.e_phnum != 2 || ph_iter.clone().count() != 2 {
        return Err(ElfLoaderError::InvalidRiscu("must have 2 program segments"));
    }

    let code_segment_header = match ph_iter
        .clone()
        .find(|ph| !ph.is_write() && !ph.is_read() && ph.is_executable())
    {
        Some(segment) => segment,
        None => {
            return Err(ElfLoaderError::InvalidRiscu(
                "code segment (must be executable only) is missing",
            ))
        }
    };

    let data_segment_header =
        match ph_iter.find(|ph| ph.is_write() && ph.is_read() && !ph.is_executable()) {
            Some(segment) => segment,
            None => {
                return Err(ElfLoaderError::InvalidRiscu(
                    "data segment (must be readable and writable only) is missing",
                ))
            }
        };

    let code_start = code_segment_header.p_vaddr;
    let code_segment = &raw[code_segment_header.file_range()];

    let data_start = code_segment_header.p_vaddr;
    let data_segment = &raw[data_segment_header.file_range()];

    Ok([(code_start, code_segment), (data_start, data_segment)])
}

fn copy_segments(segments: [(u64, &[u8]); 2]) -> Program {
    Program {
        code: ProgramSegment {
            address: segments[0].0,
            content: Vec::from(segments[0].1),
        },
        data: ProgramSegment {
            address: segments[1].0,
            content: Vec::from(segments[1].1),
        },
    }
}

fn copy_and_decode_segments(segments: [(u64, &[u8]); 2]) -> Result<DecodedProgram, ElfLoaderError> {
    let code = ProgramSegment {
        address: segments[0].0,
        content: segments[0]
            .1
            .chunks_exact(size_of::<u32>())
            .map(LittleEndian::read_u32)
            .map(|raw| decode(raw).map_err(ElfLoaderError::DecodingError))
            .collect::<Result<Vec<_>, _>>()?,
    };

    let data = ProgramSegment {
        address: segments[1].0,
        content: segments[1]
            .1
            .chunks_exact(size_of::<u64>())
            .map(LittleEndian::read_u64)
            .collect::<Vec<_>>(),
    };

    Ok(DecodedProgram { code, data })
}
