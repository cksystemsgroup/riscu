pub(super) use super::util::{CbInstr, CiInstr, CsInstr, InstrFormat};

use super::util::*;
use crate::{decompress::DecompressionResult, DecodingError, Register};

// Decompression helpers for quadrant 0 {{{
pub(super) fn decompress_addi4spn(i: u16) -> DecompressionResult {
    let imm = get_imm(i, InstrFormat::Ciw).inv_permute(&[5, 4, 9, 8, 7, 6, 2, 3]);
    let rd = 8 + ((i >> 2) & 0b111);

    assert!(imm != 0, "imm == 0 is reserved");

    Ok(build_itype(CiInstr::Addi, rd, Register::Sp as u16, imm))
}

pub(super) fn decompress_load(i: u16, instruction_type: CiInstr) -> DecompressionResult {
    let imm = get_imm(i, InstrFormat::Cl);
    let rd = 8 + ((i >> 2) & 0b111);
    let rs1 = 8 + ((i >> 7) & 0b111);

    Ok(match instruction_type {
        CiInstr::Lw => build_itype(CiInstr::Lw, rd, rs1, imm.inv_permute(&[5, 4, 3, 2, 6])),
        CiInstr::Ld => build_itype(CiInstr::Ld, rd, rs1, imm.inv_permute(&[5, 4, 3, 7, 6])),
        _ => unreachable!(),
    })
}

pub(super) fn decompress_store(i: u16, instruction_type: CsInstr) -> DecompressionResult {
    let imm = get_imm(i, InstrFormat::Cs);
    let rs2 = 8 + ((i >> 2) & 0b111);
    let rs1 = 8 + ((i >> 7) & 0b111);

    Ok(match instruction_type {
        CsInstr::Sw => build_stype(CsInstr::Sw, rs1, rs2, imm.inv_permute(&[5, 4, 3, 2, 6])),
        CsInstr::Sd => build_stype(CsInstr::Sd, rs1, rs2, imm.inv_permute(&[5, 4, 3, 7, 6])),
    })
}
// }}}

// Decompression helpers for quadrant 1 {{{
pub(super) fn decompress_addi(i: u16, instruction_type: CiInstr) -> DecompressionResult {
    let imm = sign_extend16(get_imm(i, InstrFormat::Ci), 6);
    let dest = (i >> 7) & 0b1_1111;

    assert!(dest != 0, "dest == 0 is reserved!");
    if matches!(instruction_type, CiInstr::Addi) {
        assert!(imm != 0, "imm == 0 is a HINT!");
    }

    Ok(match instruction_type {
        CiInstr::Addi => build_itype(CiInstr::Addi, dest, dest, imm),
        CiInstr::Addiw => build_itype(CiInstr::Addiw, dest, dest, imm),
        _ => unreachable!(),
    })
}

pub(super) fn decompress_li(i: u16) -> DecompressionResult {
    let rd = (i >> 7) & 0b11111;
    let imm = sign_extend16(get_imm(i, InstrFormat::Ci), 6);

    assert!(rd != 0, "rd == 0 is reserved!");

    Ok(build_itype(CiInstr::Addi, rd, Register::Zero as u16, imm))
}

pub(super) fn decompress_lui_addi16sp(i: u16) -> DecompressionResult {
    let rd = (i >> 7) & 0b1_1111;
    let imm = get_imm(i, InstrFormat::Ci);

    assert!(rd != 0, "rd = 0 is reserved!");

    if rd == 2 {
        /* C.ADDI16SP */
        assert!(imm != 0, "imm = 0 is reserved!");

        let imm = imm.inv_permute(&[9, 4, 6, 8, 7, 6, 5]);

        Ok(build_itype(CiInstr::Addi, rd, rd, imm))
    } else {
        let imm = (imm as u32).inv_permute(&[17, 16, 15, 14, 13, 12]);

        Ok(build_utype(CuInstr::Lui, rd, sign_extend32(imm, 18)))
    }
}

pub(super) fn decompress_jump(i: u16) -> DecompressionResult {
    let imm = get_imm(i, InstrFormat::Cj).inv_permute(&[11, 4, 9, 8, 10, 6, 7, 3, 2, 1, 5]);
    Ok(build_jtype(imm))
}

pub(super) fn decompress_misc_alu(i: u16) -> DecompressionResult {
    match (i >> 10) & 0b11 {
        0b00 => Err(DecodingError::Unimplemented), // C.SRLI
        0b01 => {
            let shamt = get_imm(i, InstrFormat::Ci);
            let rd_rs1 = 8 + ((i >> 7) & 0b111);

            assert!(shamt != 0, "shamt == 0 is reserved!");

            Ok(build_itype(CiInstr::Srai, rd_rs1, rd_rs1, shamt))
        }
        0b10 => Err(DecodingError::Unimplemented), // C.ANDI
        0b11 => {
            let rs1_rd = 8 + ((i >> 7) & 0b111);
            let rs2 = 8 + ((i >> 2) & 0b111);

            match ((i >> 12) & 0b1, (i >> 5) & 0b11) {
                (0, 0b00) => Ok(build_rtype(CrInstr::Sub, rs1_rd, rs1_rd, rs2)),
                (0, 0b10) => Ok(build_rtype(CrInstr::Or, rs1_rd, rs1_rd, rs2)),
                (0, 0b11) => Ok(build_rtype(CrInstr::And, rs1_rd, rs1_rd, rs2)),
                (1, 0b10) => Err(DecodingError::Reserved),
                (1, 0b11) => Err(DecodingError::Reserved),
                _ => unreachable!(),
            }
        }
        _ => Err(DecodingError::Unimplemented),
    }
}

pub(super) fn decompress_branch(i: u16, instruction_type: CbInstr) -> DecompressionResult {
    let rs1 = 8 + ((i >> 7) & 0b111);
    let offset = get_imm(i, InstrFormat::Cb).inv_permute(&[8, 4, 3, 7, 6, 2, 1, 5]);

    Ok(build_btype(instruction_type, rs1, sign_extend16(offset, 9)))
}
// }}}

// Decompression helpers for quadrant 2 {{{
pub(super) fn decompress_slli(i: u16) -> DecompressionResult {
    let shamt = get_imm(i, InstrFormat::Ci);
    let rd_rs1 = (i >> 7) & 0b1_1111;

    assert!(rd_rs1 != 0, "rd_rs1 == 0 is reserved!");
    assert!(shamt != 0, "shamt == 0 is a HINT!");

    Ok(build_itype(CiInstr::Slli, rd_rs1, rd_rs1, shamt))
}

pub(super) fn decompress_load_sp(i: u16, instruction_type: CiInstr) -> DecompressionResult {
    let imm = get_imm(i, InstrFormat::Ci);
    let rs1 = Register::Sp as u16;
    let rd = (i >> 7) & 0b1_1111;

    assert!(rd != 0, "rd == 0 is reserved!");

    match instruction_type {
        CiInstr::Lw => {
            let imm = imm.inv_permute(&[5, 4, 3, 2, 7, 6]);
            Ok(build_itype(CiInstr::Lw, rd, rs1, imm))
        }
        CiInstr::Ld => {
            let imm = imm.inv_permute(&[5, 4, 3, 8, 7, 6]);
            Ok(build_itype(CiInstr::Ld, rd, rs1, imm))
        }
        _ => unreachable!(),
    }
}

pub(super) fn decompress_jr_mv_add(i: u16) -> DecompressionResult {
    match ((i >> 12) & 0b1, (i >> 7) & 0b1_1111, (i >> 2) & 0b1_1111) {
        (0, rs1, 0) /* C.JR */ => {
            assert!(rs1 != 0, "rs1 == 0 is reserved!");

            Ok(build_itype(CiInstr::Jalr, Register::Zero as u16, rs1, 0))
        },
        (0, rd, rs2) /* C.MV */ => {
            assert!(rd != 0, "rd == 0 is reserved!");

            Ok(build_rtype(CrInstr::Add, rd, Register::Zero as u16, rs2))
        },
        (1, 0, 0) /* C.EBREAK */ => Err(DecodingError::Unimplemented),
        (1, rs1, 0) => Ok(build_itype(CiInstr::Jalr, Register::Ra as u16, rs1, 0)),
        (1, rd, rs2) => {
            assert!(!(rd == 0 && rs2 != 0), "rd == 0 && rs2 != 0 is a HINT!");

            Ok(build_rtype(CrInstr::Add, rd, rd, rs2))
        },
        (_, _, _) => Err(DecodingError::Unimplemented),
    }
}

pub(super) fn decompress_store_sp(i: u16, instruction_type: CsInstr) -> DecompressionResult {
    let imm = get_imm(i, InstrFormat::Css);
    let rs1 = Register::Sp as u16;
    let rs2 = (i >> 2) & 0b1_1111;

    match instruction_type {
        CsInstr::Sw => Err(DecodingError::Unimplemented),
        CsInstr::Sd => {
            let imm = imm.inv_permute(&[5, 4, 3, 8, 7, 6]);

            Ok(build_stype(CsInstr::Sd, rs1, rs2, imm))
        }
    }
}
// }}}
