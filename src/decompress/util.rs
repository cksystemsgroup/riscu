use crate::Register;

pub(super) enum CrInstr {
    Sub,
    Add,
    Or,
    And,
}

pub(super) enum CiInstr {
    Addi,
    Addiw,
    Lw,
    Ld,
    Jalr,
    Slli,
    Srai,
}

pub(super) enum CbInstr {
    Beq,
    Bne,
}

pub(super) enum CsInstr {
    Sw,
    Sd,
}

pub(super) enum CuInstr {
    Lui,
}

pub(super) fn build_rtype(instruction_type: CrInstr, rd: u16, rs1: u16, rs2: u16) -> u32 {
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
        CrInstr::And => mold(0b0000000, rs2, rs1, 0b111, rd, 0b0110011),
    }
}

pub(super) fn build_itype(instruction_type: CiInstr, rd: u16, rs1: u16, imm: u16) -> u32 {
    let mold = |imm: u16, rs1: u16, funct3: u32, rd: u16, opcode: u32| -> u32 {
        let rd: u32 = rd.into();
        let rs1: u32 = rs1.into();
        let imm: u32 = imm.into();

        (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
    };

    match instruction_type {
        CiInstr::Addi => mold(imm, rs1, 0b000, rd, 0b0010011),
        CiInstr::Addiw => mold(imm, rs1, 0b000, rd, 0b0011011),
        CiInstr::Lw => mold(imm, rs1, 0b010, rd, 0b0000011),
        CiInstr::Ld => mold(imm, rs1, 0b011, rd, 0b0000011),
        CiInstr::Jalr => mold(imm, rs1, 0b000, rd, 0b1100111),
        CiInstr::Slli => mold(imm, rs1, 0b001, rd, 0b0010011),
        CiInstr::Srai => mold((0b0100000 << 5) | imm, rs1, 0b101, rd, 0b0010011),
    }
}

pub(super) fn build_btype(instruction_type: CbInstr, rs1: u16, imm: u16) -> u32 {
    let mold = |imm: u16, rs1: u16, rs2: u16, funct3: u32, opcode: u32| -> u32 {
        let rs1: u32 = rs1.into();
        let rs2: u32 = rs2.into();

        let imm: u32 = imm.permute(&[12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11]).into();

        // FIXME: check if this is correct!
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

pub(super) fn build_stype(instruction_type: CsInstr, rs1: u16, rs2: u16, imm: u16) -> u32 {
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

pub(super) fn build_utype(instruction_type: CuInstr, rd: u16, imm: u32) -> u32 {
    let mold = |imm: u32, rd: u16, opcode: u32| -> u32 {
        let rd: u32 = rd.into();
        let imm: u32 = imm >> 12;

        (imm << 12) | (rd << 7) | opcode
    };

    match instruction_type {
        CuInstr::Lui => mold(imm, rd, 0b0110111),
    }
}

pub(super) fn build_jtype(imm: u16) -> u32 {
    let mold = |imm: u16, rd: u16, opcode: u32| -> u32 {
        let rd: u32 = rd.into();

        let imm = sign_extend32(imm.into(), 12).permute(&[
            20, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 11, 19, 18, 17, 16, 15, 14, 13, 12,
        ]);

        (imm << 12) | (rd << 7) | opcode
    };

    mold(imm, Register::Zero as u16, 0b1101111)
}

pub(super) enum InstrFormat {
    Ci,
    Css,
    Ciw,
    Cl,
    Cs,
    Cb,
    Cj,
}

#[inline(always)]
pub(super) fn get_imm(i: u16, fmt: InstrFormat) -> u16 {
    match fmt {
        InstrFormat::Ci => ((i >> 7) & 0b10_0000) | ((i >> 2) & 0b1_1111),
        InstrFormat::Css => (i >> 7) & 0b11_1111,
        InstrFormat::Ciw => (i >> 5) & 0b1111_1111,
        InstrFormat::Cl => ((i >> 8) & 0b1_1100) | ((i >> 5) & 0b11),
        InstrFormat::Cs => ((i >> 8) & 0b1_1100) | ((i >> 5) & 0b11),
        InstrFormat::Cb => ((i >> 5) & 0b1110_0000) | ((i >> 2) & 0b1_1111),
        InstrFormat::Cj => (i >> 2) & 0b111_1111_1111,
    }
}

pub(super) trait Permutable {
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
pub(super) fn sign_extend16(n: u16, b: u32) -> u16 {
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
pub(super) fn sign_extend32(n: u32, b: u32) -> u32 {
    assert!(n <= 2_u32.pow(b));
    assert!(0 < b && b < 32);

    if n < 2_u32.pow(b - 1) {
        n
    } else {
        n.wrapping_sub(2_u32.pow(b))
    }
}
