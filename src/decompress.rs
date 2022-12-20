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

/// Decompress compressed instructions from quadrant one to the corresponding 32-bit instruction.
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

    #[test]
    fn test_quadrant0() {
        // C.ADDI4SPN
        assert_eq!(decode(0x002c).unwrap(), Addi(IType(0x00810593))); // addi a1, sp, 8

        // C.LFD unimplemented

        // C.LW
        assert_eq!(decode(0x4298).unwrap(), Lw(IType(0x0006a703))); // lw a4, 0(a3)
        assert_eq!(decode(0x483c).unwrap(), Lw(IType(0x05042783))); // lw a5, 80(s0)

        // C.LD
        assert_eq!(decode(0x6398).unwrap(), Ld(IType(0x0007b703))); // ld a4, 0(a5)
        assert_eq!(decode(0x6b9c).unwrap(), Ld(IType(0x0107b783))); // ld a5, 16(a5)

        // Reserved

        // C.FSD unimplemented

        // C.SW
        assert_eq!(decode(0xc298).unwrap(), Sw(SType(0x00e6a023))); // sw a4, 0(a3)
        assert_eq!(decode(0xd01c).unwrap(), Sw(SType(0x02f42023))); // sw a5, 32(s0)

        // C.SD
        assert_eq!(decode(0xe398).unwrap(), Sd(SType(0x00e7b023))); // sd a4, 0(a5)
        assert_eq!(decode(0xee98).unwrap(), Sd(SType(0x00e6bc23))); // sd a4, 24(a3)
    }
}
