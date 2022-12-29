mod detail;
mod util;

use crate::DecodingError;
use detail::*;

type DecompressionResult = Result<u32, DecodingError>;

/// Decompress compressed instructions from quadrant zero to the corresponding 32-bit instruction.
pub fn decompress_q0(i: u16) -> DecompressionResult {
    if i == 0 {
        return Err(DecodingError::Illegal);
    }

    match (i >> 13) & 0b111 {
        0b000 => decompress_addi4spn(i),
        0b001 => Err(DecodingError::Unimplemented), // C.LFD
        0b010 => decompress_load(i, CiInstr::Lw),
        0b011 => decompress_load(i, CiInstr::Ld),
        0b100 => Err(DecodingError::Reserved),
        0b101 => Err(DecodingError::Unimplemented), // C.FSD
        0b110 => decompress_store(i, CsInstr::Sw),
        0b111 => decompress_store(i, CsInstr::Sd),
        _ => unreachable!(),
    }
}

/// Decompress compressed instructions from quadrant one to the corresponding 32-bit instruction.
pub fn decompress_q1(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 => decompress_addi(i, CiInstr::Addi),
        0b001 => decompress_addi(i, CiInstr::Addiw),
        0b010 => decompress_li(i),
        0b011 => decompress_lui_addi16sp(i),
        0b100 => decompress_misc_alu(i),
        0b101 => decompress_jump(i),
        0b110 => decompress_branch(i, CbInstr::Beq),
        0b111 => decompress_branch(i, CbInstr::Bne),
        _ => unreachable!(),
    }
}

/// Decompress compressed instructions from quadrant two to the corresponding 32-bit instruction.
pub fn decompress_q2(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 => decompress_slli(i),
        0b001 => Err(DecodingError::Unimplemented), // C.FLSDP
        0b010 => decompress_load_sp(i, CiInstr::Lw),
        0b011 => decompress_load_sp(i, CiInstr::Ld),
        0b100 => decompress_jr_mv_add(i),
        0b101 => Err(DecodingError::Unimplemented), // C.FSDSP
        0b110 => Err(DecodingError::Unimplemented), // C.SWSP
        0b111 => decompress_store_sp(i, CsInstr::Sd),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, types::*, Instruction::*};

    // The bulk of the instructions were obtained by compiling selfie with
    // an rv64imc enabled gcc compiler. Examples for case were created by hand.

    #[test]
    fn test_quadrant0() {
        // C.ADDI4SPN
        assert_eq!(decode(0x002c).unwrap(), Addi(IType(0x00810593))); // addi a1, sp, 8

        // C.LFD unimplemented

        // C.LW
        assert_eq!(decode(0x4298).unwrap(), Lw(IType(0x0006a703))); // lw a4, 0(a3)
        assert_eq!(decode(0x483c).unwrap(), Lw(IType(0x05042783))); // lw a5, 80(s0)

        // C.LW: check that no sign extension happens (smoke test)
        assert_eq!(decode(0x5ffc).unwrap(), Lw(IType(0x07c7a783))); // lw a5, 124(a5)

        // C.LD
        assert_eq!(decode(0x6398).unwrap(), Ld(IType(0x0007b703))); // ld a4, 0(a5)
        assert_eq!(decode(0x6b9c).unwrap(), Ld(IType(0x0107b783))); // ld a5, 16(a5)

        // C.LD: check that no sign extension happens (smoke test)
        assert_eq!(decode(0x7ffc).unwrap(), Ld(IType(0x0f87b783))); // ld a5, 248(a5)

        // C.FSD unimplemented

        // C.SW
        assert_eq!(decode(0xc298).unwrap(), Sw(SType(0x00e6a023))); // sw a4, 0(a3)
        assert_eq!(decode(0xd01c).unwrap(), Sw(SType(0x02f42023))); // sw a5, 32(s0)

        // C.SW: check that no sign extension happens (smoke test)
        assert_eq!(decode(0xdffc).unwrap(), Sw(SType(0x06f7ae23))); // sw a5, 124(a5)

        // C.SD
        assert_eq!(decode(0xe398).unwrap(), Sd(SType(0x00e7b023))); // sd a4, 0(a5)
        assert_eq!(decode(0xee98).unwrap(), Sd(SType(0x00e6bc23))); // sd a4, 24(a3)

        // C.SD: check that no sign extension happens (smoke test)
        assert_eq!(decode(0xfffc).unwrap(), Sd(SType(0x0ef7bc23))); // sd a5, 248(a5)
    }

    #[test]
    fn test_quadrant1() {
        // C.NOP

        // C.ADDI
        assert_eq!(decode(0x17e1).unwrap(), Addi(IType(0xff878793))); // addi a5, a5, -8
        assert_eq!(decode(0x0785).unwrap(), Addi(IType(0x00178793))); // addi a5, a5, 1

        // C.ADDIW
        assert_eq!(decode(0x37fd).unwrap(), Addiw(IType(0xfff7879b))); // addiw a5, a5, -1
        assert_eq!(decode(0x2705).unwrap(), Addiw(IType(0x0017071b))); // addiw a4, a4, 1

        // C.LI
        assert_eq!(decode(0x4581).unwrap(), Addi(IType(0x00000593))); // li a1, 0
        assert_eq!(decode(0x577d).unwrap(), Addi(IType(0xfff00713))); // li a4, -1

        // C.ADDI16SP
        assert_eq!(decode(0x002c).unwrap(), Addi(IType(0x00810593))); // addi a1, sp, 8
        assert_eq!(decode(0x1141).unwrap(), Addi(IType(0xff010113))); // addi sp, sp, -16

        // C.LUI
        assert_eq!(decode(0x6785).unwrap(), Lui(UType(0x000017b7))); // lui a5, 0x1
        assert_eq!(decode(0x77fd).unwrap(), Lui(UType(0xfffff7b7))); // lui a5, 0xfffff

        // ALU has a separate test-suite

        // C.J
        assert_eq!(decode(0xb761).unwrap(), Jal(JType(0xf89ff06f))); // j 0x1fff88 (or just j -120)
        assert_eq!(decode(0xa035).unwrap(), Jal(JType(0x02c0006f))); // j 0x2c
        assert_eq!(decode(0xa809).unwrap(), Jal(JType(0x0120006f))); // j 0x12

        // C.BEQZ
        assert_eq!(decode(0xc781).unwrap(), Beq(BType(0x00078463))); // beqz a5, 0x8
        assert_eq!(decode(0xdff5).unwrap(), Beq(BType(0xfe078ee3))); // beqz a5, 0xfd (or just beqz a5, -4)

        // C.BNEZ
        assert_eq!(decode(0xe38d).unwrap(), Bne(BType(0x02079163))); // bnez a5, 0x22
        assert_eq!(decode(0xfff5).unwrap(), Bne(BType(0xfe079ee3))); // bnez a5, 0xfd (or just bnez a5, -4)
    }

    #[test]
    fn test_quadrant1_alu() {
        // C.SRLI unimplemented

        // C.SRAI
        assert_eq!(decode(0x840d).unwrap(), Srai(IType(0x40345413))); // srai s0, s0, 0x3
        assert_eq!(decode(0x947d).unwrap(), Srai(IType(0x43f45413))); // srai s0, s0, 0x3f

        // C.ANDI unimplemented

        // C.SUB
        assert_eq!(decode(0x8e09).unwrap(), Sub(RType(0x40a60633))); // sub a2, a2, a0

        // C.XOR unimplemented

        // C.OR
        assert_eq!(decode(0x8f5d).unwrap(), Or(RType(0x00f76733))); // or a4, a4, a5

        // C.AND
        assert_eq!(decode(0x8ff9).unwrap(), And(RType(0x00e7f7b3))); // and a5, a5, a4

        // C.SUBW unimplemented

        // C.ADDW unimplemented
    }

    #[test]
    fn test_quadrant2() {
        // C.SLLI
        assert_eq!(decode(0x078e).unwrap(), Slli(IType(0x00379793))); // slli a5, a5, 0x3

        // C.FLDSP unimplemented

        // C.LWSP
        assert_eq!(decode(0x4502).unwrap(), Lw(IType(0x00012503))); // lw a0, 0(sp)
        assert_eq!(decode(0x483c).unwrap(), Lw(IType(0x05042783))); // lw a5, 80(s0)

        // C.LDSP
        assert_eq!(decode(0x60e2).unwrap(), Ld(IType(0x01813083))); // ld ra, 24(sp)
        assert_eq!(decode(0x6442).unwrap(), Ld(IType(0x01013403))); // ld s0, 16(sp)

        // C.JR, C.MV, C.ADD
        assert_eq!(decode(0x8782).unwrap(), Jalr(IType(0x00078067))); // jr a5
        assert_eq!(decode(0x853e).unwrap(), Add(RType(0x00f00533))); // mv a0, a5
        assert_eq!(decode(0x9782).unwrap(), Jalr(IType(0x000780e7))); // jalr a5
        assert_eq!(decode(0x97ba).unwrap(), Add(RType(0x00e787b3))); // add a5, a5, a4

        // C.FSDSP unimplemented

        // C.SWSP unimplemented

        // C.SDSP
        assert_eq!(decode(0xe022).unwrap(), Sd(SType(0x00813023))); // sd s0, 0(sp)
        assert_eq!(decode(0xec06).unwrap(), Sd(SType(0x00113c23))); // sd ra, 24(sp)
    }
}
