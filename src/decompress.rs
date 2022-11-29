use crate::DecodingError;

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
        0b101 /* C.J */ => Err(DecodingError::Unimplemented),
        0b110 /* C.BEQZ */ => Err(DecodingError::Unimplemented),
        0b111 /* C.BNEZ */ => Err(DecodingError::Unimplemented),
        _ => unreachable!(),
    }
}

pub fn decompress_q2(i: u16) -> DecompressionResult {
    match (i >> 13) & 0b111 {
        0b000 /* C.SLLI{,64} */ => Err(DecodingError::Unimplemented),
        0b001 /* C.FLDSP */ => Err(DecodingError::Unimplemented),
        0b010 /* C.LWSP */ => Err(DecodingError::Unimplemented),
        0b011 /* C.LDSP */ => Err(DecodingError::Unimplemented),
        0b100 /* C.{RJ,MV,EBREAK,JALR,ADD} */ => Err(DecodingError::Unimplemented),
        0b101 /* C.FSDSP */ => Err(DecodingError::Unimplemented),
        0b110 /* C.SWSP */ => Err(DecodingError::Unimplemented),
        0b111 /* C.SDSP */ => Err(DecodingError::Unimplemented),
        _ => unreachable!(),
    }
}
