use crate::DecodingError;

type DecompressionResult = Result<u32, DecodingError>;

enum CInstr {
    Csub,
}

fn build_rtype(instruction_type: CInstr, rd: u16, rs1: u16, rs2: u16) -> u32 {
    let mold = |funct7: u32, rs2: u16, rs1: u16, funct3: u32, rd: u16, opcode: u32| -> u32 {
        let rd: u32 = rd.into();
        let rs1: u32 = rs1.into();
        let rs2: u32 = rs2.into();

        (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
    };

    match instruction_type {
        CInstr::Csub => mold(0b0100000, rs2, rs1, 0b000, rd, 0b0110011),
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
        0b010 => Err(DecodingError::Unimplemented),
        0b011 => Err(DecodingError::Unimplemented),
        0b100 => match (i >> 10) & 0b11 {
            0b00 => Err(DecodingError::Unimplemented),
            0b01 => Err(DecodingError::Unimplemented),
            0b10 => Err(DecodingError::Unimplemented),
            0b11 => {
                let rs1_rd = 8 + ((i >> 7) & 0b111);
                let rs2 = 8 + ((i >> 2) & 0b111);

                match ((i >> 12) & 0b1, (i >> 5) & 0b11) {
                    (0, 0b00) => Ok(build_rtype(CInstr::Csub, rs1_rd, rs1_rd, rs2)),
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
