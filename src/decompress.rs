use crate::{decode, instruction_length, DecodingError, Instruction, Register};
use byteorder::{ByteOrder, LittleEndian};

type DecompressionResult = Result<u32, DecodingError>;

enum CrInstr {
    Sub,
}

enum CiInstr {
    Addi,
}
fn build_rtype(instruction_type: CrInstr, rd: u16, rs1: u16, rs2: u16) -> u32 {
    let mold = |funct7: u32, rs2: u16, rs1: u16, funct3: u32, rd: u16, opcode: u32| -> u32 {
        let rd: u32 = rd.into();
        let rs1: u32 = rs1.into();
        let rs2: u32 = rs2.into();

        (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
    };

    match instruction_type {
        CrInstr::Sub => mold(0b0100000, rs2, rs1, 0b000, rd, 0b0110011),
    }
}

fn build_itype(instruction_type: CiInstr, rd: u16, rs1: u16, imm: u16) -> u32 {
    let mold = |imm: u16, rs1: u16, funct3: u32, rd: u16, opcode: u32| -> u32 {
        let rd: u32 = rd.into();
        let rs1: u32 = rs1.into();
        let imm: u32 = imm.into();

        (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
    };

    match instruction_type {
        CiInstr::Addi => mold(imm, rs1, 0b000, rd, 0b0010011),
    }
}
pub fn decompress_q0(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 => Err(DecodingError::Illegal),
        0b001 => Err(DecodingError::Unimplemented),
        0b010 => Err(DecodingError::Unimplemented),
        0b011 => Err(DecodingError::Unimplemented),
        0b100 => Err(DecodingError::Unimplemented),
        0b101 => Err(DecodingError::Unimplemented),
        0b110 => Err(DecodingError::Unimplemented),
        0b111 => Err(DecodingError::Unimplemented),
        _ => unreachable!(),
    }
}

pub fn decompress_q1(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 => Err(DecodingError::Unimplemented),
        0b001 => Err(DecodingError::Unimplemented),
        0b010 /* C.LI */ => {
            let rd = (i >> 7) & 0b11111;
            let imm = ((i >> 6) & 0b100000) | (i >> 2) & 0b11111;

            assert!(rd != 0, "rd == 0 is reserved!");

            Ok(build_itype(CiInstr::Addi, rd, 0, imm))
        }
        0b011 /* C.LUI/C.ADDI16SP */ => Err(DecodingError::Unimplemented),
        0b100 /* MISC-ALU */ => match (i >> 10) & 0b11 {
            0b00 => Err(DecodingError::Unimplemented),
            0b01 => Err(DecodingError::Unimplemented),
            0b10 => Err(DecodingError::Unimplemented),
            0b11 => {
                let rs1_rd = 8 + ((i >> 7) & 0b111);
                let rs2 = 8 + ((i >> 2) & 0b111);

                match ((i >> 12) & 0b1, (i >> 5) & 0b11) {
                    (0, 0b00) => Ok(build_rtype(CrInstr::Sub, rs1_rd, rs1_rd, rs2)),
                    (1, 0b10) => Err(DecodingError::Reserved),
                    (1, 0b11) => Err(DecodingError::Reserved),
                    _ => unreachable!(),
                }
            }
            _ => Err(DecodingError::Unimplemented),
        },
        0b101 => Err(DecodingError::Unimplemented),
        0b110 => Err(DecodingError::Unimplemented),
        0b111 => Err(DecodingError::Unimplemented),
        _ => unreachable!(),
    }
}

pub fn decompress_q2(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 => Err(DecodingError::Unimplemented),
        0b001 => Err(DecodingError::Unimplemented),
        0b010 => Err(DecodingError::Unimplemented),
        0b011 => Err(DecodingError::Unimplemented),
        0b100 => Err(DecodingError::Unimplemented),
        0b101 => Err(DecodingError::Unimplemented),
        0b110 => Err(DecodingError::Unimplemented),
        0b111 => Err(DecodingError::Unimplemented),
        _ => unreachable!(),
    }
}

/// An iterator for all PC values where an instruction begins.
pub struct LocationIter<'a> {
    memory_view: &'a [u8],
    current_index: u64,
    address: u64,
}

impl LocationIter<'_> {
    pub fn new(memory_view: &[u8], address: u64) -> LocationIter<'_> {
        LocationIter {
            memory_view,
            current_index: 0,
            address,
        }
    }

    fn current_word(&self) -> u16 {
        let idx: usize = self.current_index as usize;
        let begin = &self.memory_view[idx..idx + 2];
        LittleEndian::read_u16(begin)
    }
}

impl Iterator for LocationIter<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match instruction_length(self.current_word()) {
            2 => {
                self.current_index += 2;
                Some(self.address + self.current_index - 2)
            }
            4 => {
                self.current_index += 4;
                Some(self.address + self.current_index - 4)
            }
            l => panic!("Unimplemented instruction length: {}", l),
        }
    }
}

/// An iterator for all instructions in the program.
pub struct InstructionIter<'a> {
    memory_view: &'a [u8],
    current_index: u64,
}

impl InstructionIter<'_> {
    pub fn new(memory_view: &[u8]) -> InstructionIter<'_> {
        InstructionIter {
            memory_view,
            current_index: 0,
        }
    }

    fn current_word(&self) -> u16 {
        let idx: usize = self.current_index as usize;
        let begin = &self.memory_view[idx..idx + 2];

        LittleEndian::read_u16(begin)
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.current_word();

        self.current_index += 2;

        word
    }

    fn fetch_dword(&mut self) -> u32 {
        let idx: usize = self.current_index as usize;
        let begin = &self.memory_view[idx..idx + 4];

        self.current_index += 4;

        LittleEndian::read_u32(begin)
    }
}

impl Iterator for InstructionIter<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.memory_view.len() as u64 {
            return None;
        }

        Some(
            decode(match instruction_length(self.current_word()) {
                2 => self.fetch_word().into(),
                4 => self.fetch_dword(),
                l => panic!("Unimplemented instruction length: {}", l),
            })
            .expect("valid instruction"),
        )
    }
}
