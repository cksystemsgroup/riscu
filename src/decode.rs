// Copyright 2019 Jonathan Behrens
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This module was modified by the Selfie authors.

use crate::{types::*, Instruction};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecodingError {
    /// Instruction's opcode is reserved for custom extentions and thus can't be decoded further.
    Custom,
    /// Instruction's opcode is reserved for future standard extentions.
    Reserved,
    /// Instruction bit pattern not defined in current specification.
    Unknown,
    /// More bits from the instruction are required to fully decode it.
    Truncated,
    /// Instruction type is well defined but is part of some extension this library doesn't support
    /// decoding yet.
    Unimplemented,
}

type DecodingResult = Result<Instruction, DecodingError>;

/// Return the length (in bytes) of an instruction given the low 16 bits of it.
///
/// The current spec reserves a bit pattern for instructions of length >= 192 bits, but for
/// simplicity this function just returns 24 in that case. The largest instructions currently
/// defined are 4 bytes so it will likely be a long time until this diffence matters.
pub fn instruction_length(i: u16) -> usize {
    if i & 0b11 != 0b11 {
        2
    } else if i & 0b11100 != 0b11100 {
        4
    } else if i & 0b111111 == 0b011111 {
        6
    } else if i & 0b1111111 == 0b011111 {
        8
    } else {
        10 + 2 * ((i >> 12) & 0b111) as usize
    }
}

/// Decode the given instruction.
pub fn decode(i: u32) -> DecodingResult {
    match i & 0b11 {
        0b11 => match (i >> 2) & 0b11111 {
            0b00000 => decode_load(i),
            0b00001 => Err(DecodingError::Unimplemented), // Load-FP
            0b00010 => Err(DecodingError::Custom),
            0b00011 => Err(DecodingError::Unknown), // misc mem instruction
            0b00100 => decode_op_imm(i),
            0b00101 => Err(DecodingError::Unknown), // aupic instruction
            0b00110 => Err(DecodingError::Unknown), // op imm32 instruction
            0b00111 => Err(DecodingError::Reserved), // 48bit instruction

            0b01000 => decode_store(i),
            0b01001 => Err(DecodingError::Unimplemented), // Store-FP
            0b01010 => Err(DecodingError::Custom),
            0b01011 => Err(DecodingError::Unimplemented), // AMO
            0b01100 => decode_op(i),
            0b01101 => Ok(Instruction::Lui(UType(i))),
            0b01110 => Err(DecodingError::Unknown), // op32 instruction
            0b01111 => Err(DecodingError::Reserved), // 64bit instruction

            0b10000 => Err(DecodingError::Unimplemented), // MADD
            0b10001 => Err(DecodingError::Unimplemented), // MSUB
            0b10010 => Err(DecodingError::Unimplemented), // NMSUB
            0b10011 => Err(DecodingError::Unimplemented), // NMADD
            0b10100 => Err(DecodingError::Unimplemented), // OP-FP
            0b10101 => Err(DecodingError::Reserved),
            0b10110 => Err(DecodingError::Custom),
            0b10111 => Err(DecodingError::Reserved), // 48bit instruction

            0b11000 => decode_branch(i),
            0b11001 => Ok(Instruction::Jalr(IType(i))),
            0b11010 => Err(DecodingError::Reserved),
            0b11011 => Ok(Instruction::Jal(JType(i))),
            0b11100 => decode_system(i),
            0b11101 => Err(DecodingError::Reserved),
            0b11110 => Err(DecodingError::Custom),
            0b11111 => Err(DecodingError::Reserved), // >= 80bit instruction
            _ => unreachable!(),
        },
        _ => Err(DecodingError::Unknown), // compressed instructions
    }
}

#[inline(always)]
fn decode_load(i: u32) -> DecodingResult {
    match (i >> 12) & 0b111 {
        0b011 => Ok(Instruction::Ld(IType(i))),
        0b111 => Err(DecodingError::Reserved),
        _ => Err(DecodingError::Unknown),
    }
}

#[inline(always)]
fn decode_op_imm(i: u32) -> DecodingResult {
    match (i >> 12) & 0b111 {
        0b000 => Ok(Instruction::Addi(IType(i))),
        _ => Err(DecodingError::Unknown),
    }
}

#[inline(always)]
fn decode_store(i: u32) -> DecodingResult {
    match (i >> 12) & 0b111 {
        0b011 => Ok(Instruction::Sd(SType(i))),
        _ => Err(DecodingError::Unknown),
    }
}

#[inline(always)]
fn decode_op(i: u32) -> DecodingResult {
    match (i >> 25, (i >> 12) & 0b111) {
        (0b0000000, 0b000) => Ok(Instruction::Add(RType(i))),
        (0b0100000, 0b000) => Ok(Instruction::Sub(RType(i))),
        (0b0000000, 0b011) => Ok(Instruction::Sltu(RType(i))),
        (0b0000001, 0b000) => Ok(Instruction::Mul(RType(i))),
        (0b0000001, 0b101) => Ok(Instruction::Divu(RType(i))),
        (0b0000001, 0b111) => Ok(Instruction::Remu(RType(i))),
        _ => Err(DecodingError::Unknown),
    }
}

fn decode_branch(i: u32) -> DecodingResult {
    match (i >> 12) & 0b111 {
        0b000 => Ok(Instruction::Beq(BType(i))),
        _ => Err(DecodingError::Unknown),
    }
}

fn decode_system(i: u32) -> DecodingResult {
    match i {
        // Environment Call and Breakpoint
        0b0000_0000_0000_0000_0000_0000_0111_0011 => Ok(Instruction::Ecall(IType(i))),
        _ => Err(DecodingError::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;

    // Nearly all tests are derived from the output of
    // [riscv-tests](https://github.com/riscv/riscv-tests)
    //
    // Examples of individual instructions were extracted with a simple bash command (see below),
    // and then post-processed with emacs macros.
    //
    // $ rg "\tbne\t" | sort -R | tail -n 3 | xclip -selection c

    #[test]
    fn decoding() {
        assert_eq!(decode(0x00001a37).unwrap(), Lui(UType(0x00001a37))); // lui x20,0x1
        assert_eq!(decode(0x800002b7).unwrap(), Lui(UType(0x800002b7))); // lui x5,0x80000
        assert_eq!(decode(0x212120b7).unwrap(), Lui(UType(0x212120b7))); // lui x1,0x21212
        assert_eq!(decode(0xfe1ff06f).unwrap(), Jal(JType(0xfe1ff06f))); // jal x0,800029ec
        assert_eq!(decode(0x0000006f).unwrap(), Jal(JType(0x0000006f))); // jal x0,80002258
        assert_eq!(decode(0xf89ff06f).unwrap(), Jal(JType(0xf89ff06f))); // jal x0,800027ac
        assert_eq!(decode(0x00008067).unwrap(), Jalr(IType(0x00008067))); // jalr x0,0(x1)
        assert_eq!(decode(0x00008067).unwrap(), Jalr(IType(0x00008067))); // jalr x0,0(x1)
        assert_eq!(decode(0x000f0067).unwrap(), Jalr(IType(0x000f0067))); // jalr x0,0(x30)
    }

    #[test]
    fn load() {
        assert_eq!(decode(0x01853683).unwrap(), Ld(IType(0x01853683))); // Ld x13,24(x10)
        assert_eq!(decode(0x02013c03).unwrap(), Ld(IType(0x02013c03))); // Ld x24,32(x2)
        assert_eq!(decode(0x0007b703).unwrap(), Ld(IType(0x0007b703))); // Ld x14,0(x15)
    }

    #[test]
    fn op_imm() {
        assert_eq!(decode(0x00200793).unwrap(), Addi(IType(0x00200793))); // addi x15,x0,2
        assert_eq!(decode(0x00000013).unwrap(), Addi(IType(0x00000013))); // addi x0,x0,0
        assert_eq!(decode(0x00000013).unwrap(), Addi(IType(0x00000013))); // addi x0,x0,0
    }

    #[test]
    fn store() {
        assert_eq!(decode(0x0b613823).unwrap(), Sd(SType(0x0b613823))); // sd x22,176(x2)
        assert_eq!(decode(0x09213823).unwrap(), Sd(SType(0x09213823))); // sd x18,144(x2)
        assert_eq!(decode(0x00f6b423).unwrap(), Sd(SType(0x00f6b423))); // sd x15,8(x13)
    }

    #[test]
    fn op() {
        assert_eq!(decode(0x00c58633).unwrap(), Add(RType(0x00c58633))); // add x12,x11,x12
        assert_eq!(decode(0x00d506b3).unwrap(), Add(RType(0x00d506b3))); // add x13,x10,x13
        assert_eq!(decode(0x00a70533).unwrap(), Add(RType(0x00a70533))); // add x10,x14,x10
        assert_eq!(decode(0x40b50533).unwrap(), Sub(RType(0x40b50533))); // sub x10,x10,x11
        assert_eq!(decode(0x40e78533).unwrap(), Sub(RType(0x40e78533))); // sub x10,x15,x14
        assert_eq!(decode(0x41060633).unwrap(), Sub(RType(0x41060633))); // sub x12,x12,x16
        assert_eq!(decode(0x0020bf33).unwrap(), Sltu(RType(0x0020bf33))); // sltu x30,x1,x2
        assert_eq!(decode(0x0020bf33).unwrap(), Sltu(RType(0x0020bf33))); // sltu x30,x1,x2
        assert_eq!(decode(0x000030b3).unwrap(), Sltu(RType(0x000030b3))); // sltu x1,x0,x0
        assert_eq!(decode(0x021080b3).unwrap(), Mul(RType(0x021080b3))); // mul x1,x1,x1
        assert_eq!(decode(0x02208f33).unwrap(), Mul(RType(0x02208f33))); // mul x30,x1,x2
        assert_eq!(decode(0x02208133).unwrap(), Mul(RType(0x02208133))); // mul x2,x1,x2
        assert_eq!(decode(0x0220df33).unwrap(), Divu(RType(0x0220df33))); // divu x30,x1,x2
        assert_eq!(decode(0x0220df33).unwrap(), Divu(RType(0x0220df33))); // divu x30,x1,x2
        assert_eq!(decode(0x0220df33).unwrap(), Divu(RType(0x0220df33))); // divu x30,x1,x2
        assert_eq!(decode(0x0220ff33).unwrap(), Remu(RType(0x0220ff33))); // remu x30,x1,x2
        assert_eq!(decode(0x0220ff33).unwrap(), Remu(RType(0x0220ff33))); // remu x30,x1,x2
        assert_eq!(decode(0x0220ff33).unwrap(), Remu(RType(0x0220ff33))); // remu x30,x1,x2
    }

    #[test]
    fn branch() {
        assert_eq!(decode(0x10e78463).unwrap(), Beq(BType(0x10e78463))); // beq x15,x14,800024b8
        assert_eq!(decode(0x00050a63).unwrap(), Beq(BType(0x00050a63))); // beq x10,x0,80002538
        assert_eq!(decode(0x1b5a0463).unwrap(), Beq(BType(0x1b5a0463))); // beq x20,x21,80002a10
    }

    #[test]
    fn system() {
        assert_eq!(decode(0x00000073).unwrap(), Instruction::new_ecall()); // ecall
    }
}
