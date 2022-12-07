use crate::{DecodingError, Register};

type DecompressionResult = Result<u32, DecodingError>;

enum CrInstr {
    Sub,
}

enum CiInstr {
    Addi,
    Lw,
}

enum CbInstr {
    Beq,
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
        CiInstr::Lw => mold(imm, Register::Sp as u16, 0b010, rd, 0b0000011),
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
        0b010 /* C.LW */ => Err(DecodingError::Unimplemented),
        0b011 /* C.LD */ => Err(DecodingError::Unimplemented),
        0b100 => Err(DecodingError::Reserved),
        0b101 /* C.FSD */ => Err(DecodingError::Unimplemented),
        0b110 /* C.SW */ => Err(DecodingError::Unimplemented),
        0b111 /* C.SD */ => Err(DecodingError::Unimplemented),
        _ => unreachable!(),
    }
}

pub fn decompress_q1(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 /* C.ADDI */ => Err(DecodingError::Unimplemented),
        0b001 /* C.ADDIW */ => Err(DecodingError::Unimplemented),
        0b010 /* C.LI */ => {
            let rd = (i >> 7) & 0b11111;
            let imm = get_imm(i, InstrFormat::Ci);

            assert!(rd != 0, "rd == 0 is reserved!");

            Ok(build_itype(CiInstr::Addi, rd, Register::Zero as u16, imm))
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
        0b101 /* C.J */ => {
            let imm = get_imm(i, InstrFormat::Cj).inv_permute(&[11, 4, 9, 8, 10, 6, 7, 3, 2, 1, 5]);

            Ok(build_jtype(imm))
        },
        0b110 /* C.BEQZ */ => {
            let rs1 = 8 + ((i >> 7) & 0b111);
            let offset = get_imm(i, InstrFormat::Cb).inv_permute(&[8, 4, 3, 7, 6, 2, 1, 5]);

            Ok(build_btype(CbInstr::Beq, rs1, offset))
        },
        0b111 /* C.BNEZ */ => Err(DecodingError::Unimplemented),
        _ => unreachable!(),
    }
}

pub fn decompress_q2(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 /* C.SLLI{,64} */ => Err(DecodingError::Unimplemented),
        0b001 /* C.FLDSP */ => Err(DecodingError::Unimplemented),
        0b010 /* C.LWSP */ => {
            let rd = (i >> 7) & 0b11111;
            let imm = get_imm(i, InstrFormat::Ci).inv_permute(&[5, 4, 3, 2, 7, 6]);

            assert!(rd != 0, "rd == 0 is reserved!");

            Ok(build_itype(CiInstr::Lw, rd, 0, imm))
        }
        0b011 /* C.LDSP */ => Err(DecodingError::Unimplemented),
        0b100 /* C.{RJ,MV,EBREAK,JALR,ADD} */ => Err(DecodingError::Unimplemented),
        0b101 /* C.FSDSP */ => Err(DecodingError::Unimplemented),
        0b110 /* C.SWSP */ => Err(DecodingError::Unimplemented),
        0b111 /* C.SDSP */ => Err(DecodingError::Unimplemented),
        _ => unreachable!(),
    }
}

enum InstrFormat {
    Ci,
    Ciw,
    Cb,
    Cj,
}

#[inline(always)]
fn get_imm(i: u16, fmt: InstrFormat) -> u16 {
    match fmt {
        InstrFormat::Ci => ((i >> 7) & 0b10_0000) | ((i >> 2) & 0b1_1111),
        InstrFormat::Ciw => (i >> 5) & 0b1111_1111,
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
