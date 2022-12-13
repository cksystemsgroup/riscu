use crate::{DecodingError, Register};

type DecompressionResult = Result<u32, DecodingError>;

enum CrInstr {
    Sub,
    Add,
    Or,
}

enum CiInstr {
    Addi,
    Lw,
    Ld,
    Jalr,
    Slli,
    Srai,
}

enum CbInstr {
    Beq,
    Bne,
}

enum CsInstr {
    Sw,
    Sd,
}

enum CuInstr {
    Lui,
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
        CrInstr::Add => mold(0b0000000, rs2, rs1, 0b000, rd, 0b0110011),
        CrInstr::Or => mold(0b0000000, rs2, rs1, 0b110, rd, 0b0110011),
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
        CiInstr::Lw => mold(imm, rs1, 0b010, rd, 0b0000011),
        CiInstr::Ld => mold(imm, rs1, 0b011, rd, 0b0000011),
        CiInstr::Jalr => mold(imm, rs1, 0b000, rd, 0b1100111),
        CiInstr::Slli => mold(imm, rs1, 0b001, rd, 0b0010011),
        CiInstr::Srai => mold((0b0100000 << 5) | imm, rs1, 0b101, rd, 0b0010011),
    }
}

fn build_jtype(imm: u16) -> u32 {
    let mold = |imm: u16, rd: u16, opcode: u32| -> u32 {
        let rd: u32 = rd.into();

        // perform sign extension
        let sign: u32 = (imm >> 11).into();
        let imm: u32 = (0xff_ff_f0_00 * sign) | (imm as u32);

        let imm = imm.permute(&[
            20, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 19, 18, 17, 16, 15, 14, 13, 12,
        ]);

        (imm << 12) | (rd << 7) | opcode
    };

    mold(imm, Register::Zero as u16, 0b1101111)
}

fn build_stype(instruction_type: CsInstr, rs1: u16, rs2: u16, imm: u16) -> u32 {
    let mold = |rs2: u16, rs1: u16, funct3: u32, imm: u16, opcode: u32| -> u32 {
        let immh: u32 = (imm >> 5).into();
        let imml: u32 = (imm & 0b1_1111).into();
        let rs1: u32 = rs1.into();
        let rs2: u32 = rs2.into();

        (immh << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (imml << 7) | opcode
    };

    match instruction_type {
        CsInstr::Sw => mold(rs2, rs1, 0b010, imm, 0b0100011),
        CsInstr::Sd => mold(rs2, rs1, 0b011, imm, 0b0100011),
    }
}

fn build_btype(instruction_type: CbInstr, rs1: u16, imm: u16) -> u32 {
    let mold = |imm: u16, rs1: u16, rs2: u16, funct3: u32, opcode: u32| -> u32 {
        let rs1: u32 = rs1.into();
        let rs2: u32 = rs2.into();

        let imm: u32 = imm.permute(&[12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11]).into();

        ((imm >> 5) << 25)
            | (rs2 << 20)
            | (rs1 << 15)
            | (funct3 << 12)
            | ((imm & 0b11111) << 7)
            | opcode
    };

    match instruction_type {
        CbInstr::Beq => mold(imm, rs1, Register::Zero as u16, 0b000, 0b1100011),
        CbInstr::Bne => mold(imm, rs1, Register::Zero as u16, 0b001, 0b1100011),
    }
}

fn build_utype(instruction_type: CuInstr, rd: u16, imm: u32) -> u32 {
    let mold = |imm: u32, rd: u16, opcode: u32| -> u32 {
        let rd: u32 = rd.into();
        let imm: u32 = imm >> 12;

        (imm << 12) | (rd << 7) | opcode
    };

    match instruction_type {
        CuInstr::Lui => mold(imm, rd, 0b0110111),
    }
}

pub fn decompress_q0(i: u16) -> DecompressionResult {
    if i == 0 {
        return Err(DecodingError::Illegal);
    }

    match (i >> 13) & 0b111 {
        0b000 /* C.ADDI4SPN */ => {
            let imm = get_imm(i, InstrFormat::Ciw).inv_permute(&[5, 4, 9, 8, 7, 6, 2, 3]);
            let rd = 8 + ((i >> 2) & 0b111);

            assert!(imm != 0, "imm == 0 is reserved");

            Ok(build_itype(CiInstr::Addi, rd, Register::Sp as u16, imm))
        }
        0b001 /* C.FLD */ => Err(DecodingError::Unimplemented),
        0b010 /* C.LW */ => {
            let imm = get_imm(i, InstrFormat::Cl).inv_permute(&[5, 3, 2, 6]);
            let rd = 8 + ((i >> 2) & 0b111);
            let rs1 = 8 + ((i >> 7) & 0b111);

            Ok(build_itype(CiInstr::Lw, rd, rs1, imm))
        },
        0b011 /* C.LD */ => {
            let imm = get_imm(i, InstrFormat::Cl).inv_permute(&[5, 4, 3, 7, 6]);
            let rd = 8 + ((i >> 2) & 0b111);
            let rs1 = 8 + ((i >> 7) & 0b111);

            Ok(build_itype(CiInstr::Ld, rd, rs1, imm))
        },
        0b100 => Err(DecodingError::Reserved),
        0b101 /* C.FSD */ => Err(DecodingError::Unimplemented),
        0b110 /* C.SW */ => {
            let imm = get_imm(i, InstrFormat::Cl).inv_permute(&[5, 4, 3, 7, 6]);
            let rs2 = 8 + ((i >> 2) & 0b111);
            let rs1 = 8 + ((i >> 7) & 0b111);

            Ok(build_stype(CsInstr::Sw, rs1, rs2, imm))
        },
        0b111 /* C.SD */ => {
            let imm = get_imm(i, InstrFormat::Cs).inv_permute(&[5, 4, 3, 7, 6]);
            let rs2 = 8 + ((i >> 2) & 0b111);
            let rs1 = 8 + ((i >> 7) & 0b111);

            Ok(build_stype(CsInstr::Sd, rs1, rs2, imm))
        },
        _ => unreachable!(),
    }
}

pub fn decompress_q1(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 /* C.ADDI */ => {
            let imm = sign_extend(get_imm(i, InstrFormat::Ci), 6);
            let dest = (i >> 7) & 0b1_1111;

            assert!(dest != 0, "dest == 0 is reserved!");
            assert!(imm != 0, "imm == 0 is a HINT!");

            Ok(build_itype(CiInstr::Addi, dest, dest, imm))
        },
        0b001 /* C.ADDIW */ => Err(DecodingError::Unimplemented),
        0b010 /* C.LI */ => {
            let rd = (i >> 7) & 0b11111;
            let imm = sign_extend(get_imm(i, InstrFormat::Ci), 6);

            assert!(rd != 0, "rd == 0 is reserved!");

            Ok(build_itype(CiInstr::Addi, rd, Register::Zero as u16, imm))
        }
        0b011 /* C.LUI/C.ADDI16SP */ => {
            let rd = (i >> 7) & 0b1_1111;
            let imm = get_imm(i, InstrFormat::Ci);

            assert!(rd != 0, "rd = 0 is reserved!");

            if rd == 2 /* C.ADDI16SP */ {
                let imm = imm.inv_permute(&[9, 4, 6, 8, 7, 6, 5]);

                assert!(imm != 0, "imm = 0 is reserved!");

                Ok(build_itype(CiInstr::Addi, rd, rd, imm))
            } else /* C.LUI */{
                let imm = (imm as u32).inv_permute(&[17, 16, 15, 14, 13, 12]);

                Ok(build_utype(CuInstr::Lui, rd, sign_extend32(imm, 18)))
            }
        },
        0b100 /* MISC-ALU */ => match (i >> 10) & 0b11 {
            0b00 => Err(DecodingError::Unimplemented),
            0b01 /* C.SRAI */ => {
                let shamt = get_imm(i, InstrFormat::Ci);
                let rd_rs1 = 8 + ((i >> 7) & 0b111);

                assert!(shamt != 0, "shamt == 0 is reserved!");

                Ok(build_itype(CiInstr::Srai, rd_rs1, rd_rs1, shamt))
            },
            0b10 => Err(DecodingError::Unimplemented),
            0b11 => {
                let rs1_rd = 8 + ((i >> 7) & 0b111);
                let rs2 = 8 + ((i >> 2) & 0b111);

                match ((i >> 12) & 0b1, (i >> 5) & 0b11) {
                    (0, 0b00) => Ok(build_rtype(CrInstr::Sub, rs1_rd, rs1_rd, rs2)),
                    (0, 0b10) => Ok(build_rtype(CrInstr::Or, rs1_rd, rs1_rd, rs2)),
                    (1, 0b10) => Err(DecodingError::Reserved),
                    (1, 0b11) => Err(DecodingError::Reserved),
                    _ => unreachable!(),
                }
            }
            _ => Err(DecodingError::Unimplemented),
        },
        0b101 /* C.J */ => {
            let imm = get_imm(i, InstrFormat::Cj).inv_permute(&[11, 4, 9, 8, 10, 6, 7, 3, 2, 1, 5]);

            Ok(build_jtype(imm))
        },
        0b110 /* C.BEQZ */ => {
            let rs1 = 8 + ((i >> 7) & 0b111);
            let offset = get_imm(i, InstrFormat::Cb).inv_permute(&[8, 4, 3, 7, 6, 2, 1, 5]);

            Ok(build_btype(CbInstr::Beq, rs1, sign_extend(offset, 9)))
        },
        0b111 /* C.BNEZ */ => {
            let rs1 = 8 + ((i >> 7) & 0b111);
            let offset = get_imm(i, InstrFormat::Cb).inv_permute(&[8, 4, 3, 7, 6, 2, 1, 5]);

            Ok(build_btype(CbInstr::Bne, rs1, sign_extend(offset, 9)))
        },
        _ => unreachable!(),
    }
}

pub fn decompress_q2(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 /* C.SLLI */ => {
            let shamt = get_imm(i, InstrFormat::Ci);
            let rd_rs1 = (i >> 7) & 0b1_1111;

            assert!(rd_rs1 != 0, "rd_rs1 == 0 is reserved!");
            assert!(shamt != 0, "shamt == 0 is a HINT!");

            Ok(build_itype(CiInstr::Slli, rd_rs1, rd_rs1, shamt))
        },
        0b001 /* C.FLDSP */ => Err(DecodingError::Unimplemented),
        0b010 /* C.LWSP */ => {
            let rd = (i >> 7) & 0b11111;
            let imm = get_imm(i, InstrFormat::Ci).inv_permute(&[5, 4, 3, 2, 7, 6]);

            assert!(rd != 0, "rd == 0 is reserved!");

            Ok(build_itype(CiInstr::Lw, rd, 0, imm))
        }
        0b011 /* C.LDSP */ => {
            let imm = get_imm(i, InstrFormat::Ci).inv_permute(&[5,4,3,8,7,6]);
            let rd = (i >> 7) & 0b1_1111;

            assert!(rd != 0, "rd == 0 is reserved!");

            Ok(build_itype(CiInstr::Ld, rd, Register::Sp as u16, imm))
        },
        0b100 /* C.{JR,MV,EBREAK,JALR,ADD} */ => {
            match ((i >> 12) & 0b1, (i >> 7) & 0b1_1111, (i >> 2) & 0b1_1111) {
                (0, rs1, 0) /* C.JR */ => {
                    assert!(rs1 != 0, "rs1 == 0 is reserved!");

                    Ok(build_itype(CiInstr::Jalr, Register::Zero as u16, rs1, 0))
                },
                (0, rd, rs2) /* C.MV */ => {
                    assert!(rd != 0, "rs1 == 0 is reserved!");

                    Ok(build_rtype(CrInstr::Add, rd, Register::Zero as u16, rs2))
                },
                (1, 0, 0) /* C.EBREAK */ => Err(DecodingError::Unimplemented),
                (1, rs1, 0) /* C.JALR */ => Ok(build_itype(CiInstr::Jalr, Register::Ra as u16, rs1, 0)),
                (1, rd, rs2) /* C.ADD */ => {
                    assert!(!(rd == 0 && rs2 != 0), "rd == 0 && rs2 == 0 is a HINT!");

                    Ok(build_rtype(CrInstr::Add, rd, rd, rs2))
                },
                (_, _, _) => Err(DecodingError::Unimplemented),
            }
        },
        0b101 /* C.FSDSP */ => Err(DecodingError::Unimplemented),
        0b110 /* C.SWSP */ => Err(DecodingError::Unimplemented),
        0b111 /* C.SDSP */ => {
            let imm = get_imm(i, InstrFormat::Css).inv_permute(&[5,4,3,8,7,6]);
            let rs2 = (i >> 2) & 0b1_1111;

            Ok(build_stype(CsInstr::Sd, Register::Sp as u16, rs2, imm))
        },
        _ => unreachable!(),
    }
}

enum InstrFormat {
    Ci,
    Css,
    Ciw,
    Cl,
    Cb,
    Cj,
}

#[inline(always)]
fn get_imm(i: u16, fmt: InstrFormat) -> u16 {
    match fmt {
        InstrFormat::Ci => ((i >> 7) & 0b10_0000) | ((i >> 2) & 0b1_1111),
        InstrFormat::Css => (i >> 7) & 0b11_1111,
        InstrFormat::Ciw => (i >> 5) & 0b1111_1111,
        InstrFormat::Cl => ((i >> 8) & 0b1_1100) | ((i >> 5) & 0b11),
        InstrFormat::Cb => ((i >> 5) & 0b1110_0000) | ((i >> 2) & 0b1_1111),
        InstrFormat::Cj => (i >> 2) & 0b111_1111_1111,
    }
}

trait Permutable {
    /// When going from an number to the permuted representation in an instruction.
    fn permute(self, perm: &[usize]) -> Self;

    /// When going from a permuted number in an instruction to the binary representation.
    fn inv_permute(self, perm: &[usize]) -> Self;
}

impl Permutable for u16 {
    fn inv_permute(self, perm: &[usize]) -> Self {
        debug_assert!(
            perm.len() <= 16,
            "Permutation of u16 cannot exceed 16 entries."
        );
        debug_assert!(
            perm.iter().all(|x| x < &16),
            "Permutation indices for u16 cannot exceed 15."
        );

        perm.iter()
            .rev()
            .enumerate()
            .map(|(bit, offset)| ((self >> bit) & 0b1) << offset)
            .sum()
    }

    fn permute(self, perm: &[usize]) -> Self {
        debug_assert!(
            perm.len() <= 16,
            "Permutation of u16 cannot exceed 16 entries."
        );
        debug_assert!(
            perm.iter().all(|x| x < &16),
            "Permutation indices for u16 cannot exceed 15."
        );

        perm.iter()
            .rev()
            .enumerate()
            .map(|(bit, offset)| ((self >> offset) & 0b1) << bit)
            .sum()
    }
}

impl Permutable for u32 {
    fn inv_permute(self, perm: &[usize]) -> Self {
        debug_assert!(
            perm.len() <= 32,
            "Permutation of u32 cannot exceed 32 entries."
        );
        debug_assert!(
            perm.iter().all(|x| x < &32),
            "Permutation indices for u32 cannot exceed 31."
        );

        perm.iter()
            .rev()
            .enumerate()
            .map(|(bit, offset)| ((self >> bit) & 0b1) << offset)
            .sum()
    }

    fn permute(self, perm: &[usize]) -> Self {
        debug_assert!(
            perm.len() <= 32,
            "Permutation of u32 cannot exceed 32 entries."
        );
        debug_assert!(
            perm.iter().all(|x| x < &32),
            "Permutation indices for u32 cannot exceed 31."
        );

        perm.iter()
            .rev()
            .enumerate()
            .map(|(bit, offset)| ((self >> offset) & 0b1) << bit)
            .sum()
    }
}

/// Perform sign-extension for the value `n` based on bit `b`.
/// Note: the bit `b` is one-indexed.
fn sign_extend(n: u16, b: u32) -> u16 {
    assert!(n <= 2_u16.pow(b));
    assert!(0 < b && b < 16);

    if n < 2_u16.pow(b - 1) {
        n
    } else {
        n.wrapping_sub(2_u16.pow(b))
    }
}

/// Perform sign-extension for the value `n` based on bit `b`.
/// Note: the bit `b` is one-indexed.
fn sign_extend32(n: u32, b: u32) -> u32 {
    assert!(n <= 2_u32.pow(b));
    assert!(0 < b && b < 32);

    if n < 2_u32.pow(b - 1) {
        n
    } else {
        n.wrapping_sub(2_u32.pow(b))
    }
}
