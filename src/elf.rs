//! # Load RISC-U ELF64 files

use crate::{decode, DecodingError, Instruction};
use byteorder::{ByteOrder, LittleEndian};
use goblin::elf::{program_header::PT_LOAD, section_header::SHT_PROGBITS, Elf};
use log::debug;
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

impl Program {
    pub fn decode(&self) -> Result<DecodedProgram, RiscuError> {
        copy_and_decode(self)
    }
}

#[derive(Clone, Debug)]
pub struct DecodedProgram {
    pub code: ProgramSegment<Instruction>,
    pub data: ProgramSegment<u64>,
}

#[derive(Error, Debug)]
pub enum RiscuError {
    #[error("Error while reading file: {0}")]
    CouldNotReadFile(std::io::Error),

    #[error("Error while parsing ELF: {0}")]
    InvalidElf(goblin::error::Error),

    #[error("ELF is not a valid RISC-U ELF file: {0}")]
    InvalidRiscu(&'static str),

    #[error("Failure during decode: {0:?}")]
    DecodingError(DecodingError),
}

pub fn load_object_file<P>(object_file: P) -> Result<Program, RiscuError>
where
    P: AsRef<Path>,
{
    fs::read(object_file)
        .map_err(RiscuError::CouldNotReadFile)
        .and_then(|buffer| {
            Elf::parse(&buffer)
                .map_err(RiscuError::InvalidElf)
                .and_then(|elf| extract_program(&buffer, &elf))
        })
}

fn extract_program(raw: &[u8], elf: &Elf) -> Result<Program, RiscuError> {
    if elf.is_lib || !elf.is_64 || !elf.little_endian {
        return Err(RiscuError::InvalidRiscu(
            "has to be an executable, 64bit, static, little endian binary",
        ));
    }

    let mut ph_iter = elf
        .program_headers
        .as_slice()
        .iter()
        .filter(|ph| ph.p_type == PT_LOAD);

    let sh_iter = elf
        .section_headers
        .as_slice()
        .iter()
        .filter(|sh| sh.sh_type == SHT_PROGBITS);

    // println!("{:#?}", elf.program_headers.as_slice().last().file_range());

    // for ph in elf.program_headers.as_slice().iter() {
    //   print!("{:?} {:#010x?}\n", ph, ph.file_range());
    // }

    debug!(
        "Binary has {} segments according to program header table (e_phnum)",
        elf.header.e_phnum
    );

    if elf.header.e_phnum < 2
        || ph_iter.clone().count() < 2
        || usize::from(elf.header.e_phnum) < ph_iter.clone().count()
    {
        return Err(RiscuError::InvalidRiscu(
            "must have at least 2 program segments",
        ));
    }

    // println!("{:#?}", ph_iter
    //   .clone()
    //   .find(|ph| !ph.is_write() && ph.is_read() && ph.is_executable())
    // );

    let code_segment_header = match ph_iter
        .clone()
        .find(|ph| !ph.is_write() && ph.is_read() && ph.is_executable())
    {
        Some(segment) => segment,
        None => {
            return Err(RiscuError::InvalidRiscu(
                "code segment (readable and executable) is missing",
            ))
        }
    };

    // println!("{:#?}", ph_iter
    //   .clone()
    //   .find(|ph| ph.is_write() && ph.is_read() && !ph.is_executable())
    // );

    let data_segment_header =
        match ph_iter.find(|ph| ph.is_write() && ph.is_read() && !ph.is_executable()) {
            Some(segment) => segment,
            None => {
                return Err(RiscuError::InvalidRiscu(
                    "data segment (readable and writable) is missing",
                ))
            }
        };

    let code_start;
    let code_segment;
    let code_padding;

    if code_segment_header.p_offset == 0 {
        debug!("p_offset in program header not set (i.e., no Selfie-generated RISC-U executable). Falling back to section header (i.e., assuming a gcc-generated RISC-V executable).");

        let code_section_header = match sh_iter
            .clone()
            .find(|sh| !sh.is_writable() && sh.is_executable())
        {
            Some(segment) => segment,
            None => {
                return Err(RiscuError::InvalidRiscu(
                    "code section (executable) is missing",
                ))
            }
        };

        code_start = code_section_header.sh_addr;
        code_segment = &raw[code_section_header.file_range()];
        code_padding = 0;

        debug!(
            "File range of code segment: {:#010x?}",
            code_section_header.file_range()
        );
    } else {
        debug!(
            "p_offset in program header set (i.e., assuming a Selfie-generated RISC-U executable)."
        );

        code_start = code_segment_header.p_vaddr;
        code_segment = &raw[code_segment_header.file_range()];
        code_padding = (code_segment_header.p_memsz - code_segment_header.p_filesz) as usize;

        debug!(
            "File range of code segment: {:#010x?}",
            code_segment_header.file_range()
        );
    }

    let data_start = data_segment_header.p_vaddr;
    let data_segment = &raw[data_segment_header.file_range()];
    let data_padding = (data_segment_header.p_memsz - data_segment_header.p_filesz) as usize;

    debug!("Code start: {:#010x}", code_start);
    debug!("Data start: {:#010x}", data_start);

    Ok(Program {
        code: ProgramSegment {
            address: code_start,
            content: [code_segment.to_vec(), vec![0; code_padding]].concat(),
        },
        data: ProgramSegment {
            address: data_start,
            content: [data_segment.to_vec(), vec![0; data_padding]].concat(),
        },
    })
}

fn copy_and_decode(program: &Program) -> Result<DecodedProgram, RiscuError> {
    let code = ProgramSegment {
        address: program.code.address,
        content: program
            .code
            .content
            .chunks_exact(size_of::<u32>())
            .map(LittleEndian::read_u32)
            .map(|raw| decode(raw).map_err(RiscuError::DecodingError))
            .collect::<Result<Vec<_>, _>>()?,
    };

    let data = ProgramSegment {
        address: program.data.address,
        content: program
            .data
            .content
            .chunks_exact(size_of::<u64>())
            .map(LittleEndian::read_u64)
            .collect::<Vec<_>>(),
    };

    Ok(DecodedProgram { code, data })
}
