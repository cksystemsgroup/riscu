use crate::types::*;
use crate::Register;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Instruction {
    // LUI
    Lui(UType),

    // AUIPC
    Auipc(UType),

    // Jal
    Jal(JType),

    // Jalr
    Jalr(IType),

    // Branch
    Beq(BType),
    Bne(BType),
    Blt(BType),
    Bge(BType),
    Bltu(BType),
    Bgeu(BType),

    // Load
    Lb(IType),
    Lh(IType),
    Lw(IType),
    Ld(IType),
    Lbu(IType),
    Lhu(IType),
    Lwu(IType),

    // Store
    Sb(SType),
    Sh(SType),
    Sw(SType),
    Sd(SType),

    // Fence
    Fence(IType),

    // OP-imm
    Addi(IType),
    Andi(IType),

    // OP-imm32
    Addiw(IType),

    // OP
    Add(RType),
    Sub(RType),
    Sltu(RType),
    Mul(RType),
    Divu(RType),
    Remu(RType),

    // System
    Ecall(IType),
    Ebreak(IType),

    // Amo
    Lrw(RType),
    Scw(RType),
    Amoswapw(RType),
}

// opcodes
const OP_LD: u32 = 3; // 0000011, I format (LD)
const OP_IMM: u32 = 19; // 0010011, I format (ADDI, ANDI, NOP)
const OP_IMM32: u32 = 27; // 0011011, I format (ADDIW)
const OP_SD: u32 = 35; // 0100011, S format (SD)
const OP_OP: u32 = 51; // 0110011, R format (ADD, SUB, MUL, DIVU, REMU, SLTU)
const OP_LUI: u32 = 55; // 0110111, U format (LUI)
const OP_AUIPC: u32 = 23; //0010111, U format (AUIPC)
const OP_BRANCH: u32 = 99; // 1100011, B format (BEQ)
const OP_JALR: u32 = 103; // 1100111, I format (JALR)
const OP_JAL: u32 = 111; // 1101111, J format (JAL)
const OP_SYSTEM: u32 = 115; // 1110011, I format (ECALL)
const OP_AMO: u32 = 47; // 0101111, R format (AMO)
const OP_FENCE: u32 = 15; // 0001111, I format (FENCE)

// f3-codes
const F3_ADDI: u32 = 0; // 000
const F3_ANDI: u32 = 7; // 111
const F3_ADDIW: u32 = 0; // 000
const F3_ADD: u32 = 0; // 000
const F3_SUB: u32 = 0; // 000
const F3_MUL: u32 = 0; // 000
const F3_DIVU: u32 = 5; // 101
const F3_REMU: u32 = 7; // 111
const F3_SLTU: u32 = 3; // 011
const F3_LB: u32 = 0; // 000
const F3_LH: u32 = 1; // 001
const F3_LW: u32 = 2; // 010
const F3_LD: u32 = 3; // 011
const F3_LBU: u32 = 4; // 100
const F3_LHU: u32 = 5; // 101
const F3_LWU: u32 = 6; // 110
const F3_SB: u32 = 0; // 000
const F3_SH: u32 = 1; // 001
const F3_SW: u32 = 2; // 010
const F3_SD: u32 = 3; // 011
const F3_BEQ: u32 = 0; // 000
const F3_BNE: u32 = 1; // 001
const F3_BLT: u32 = 4; // 100
const F3_BGE: u32 = 5; // 101
const F3_BLTU: u32 = 6; // 110
const F3_BGEU: u32 = 7; // 111
const F3_JALR: u32 = 0; // 000
const F3_SYSTEM: u32 = 0; // 000
const F3_RV32A: u32 = 2; //010
const F3_FENCE: u32 = 0; // 000

// f7-codes
const F7_ADD: u32 = 0; // 0000000
const F7_MUL: u32 = 1; // 0000001
const F7_SUB: u32 = 32; // 0100000
const F7_DIVU: u32 = 1; // 0000001
const F7_REMU: u32 = 1; // 0000001
const F7_SLTU: u32 = 0; // 0000000
const F7_LRW: u32 = 2; // 00010 aq rl
const F7_SCW: u32 = 3; // 00011 aq rl
const F7_AMOSWAPW: u32 = 3; // 00001 aq rl

impl Instruction {
    pub fn new_nop() -> Instruction {
        Self::new_addi(Register::Zero, Register::Zero, 0)
    }
    pub fn new_add(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Add(RType::new(F7_ADD, F3_ADD, OP_OP, rs1, rs2, rd))
    }
    pub fn new_sub(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Sub(RType::new(F7_SUB, F3_SUB, OP_OP, rs1, rs2, rd))
    }
    pub fn new_mul(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Mul(RType::new(F7_MUL, F3_MUL, OP_OP, rs1, rs2, rd))
    }
    pub fn new_divu(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Divu(RType::new(F7_DIVU, F3_DIVU, OP_OP, rs1, rs2, rd))
    }
    pub fn new_remu(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Remu(RType::new(F7_REMU, F3_REMU, OP_OP, rs1, rs2, rd))
    }
    pub fn new_sltu(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Sltu(RType::new(F7_SLTU, F3_SLTU, OP_OP, rs1, rs2, rd))
    }
    pub fn new_addi(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Addi(IType::new(immediate, F3_ADDI, OP_IMM, rd, rs1))
    }
    pub fn new_andi(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Andi(IType::new(immediate, F3_ANDI, OP_IMM, rd, rs1))
    }
    pub fn new_addiw(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Addiw(IType::new(immediate, F3_ADDIW, OP_IMM32, rd, rs1))
    }
    pub fn new_lb(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Lb(IType::new(immediate, F3_LB, OP_LD, rd, rs1))
    }
    pub fn new_lh(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Lh(IType::new(immediate, F3_LH, OP_LD, rd, rs1))
    }
    pub fn new_lw(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Lw(IType::new(immediate, F3_LW, OP_LD, rd, rs1))
    }
    pub fn new_ld(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Ld(IType::new(immediate, F3_LD, OP_LD, rd, rs1))
    }
    pub fn new_lbu(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Lbu(IType::new(immediate, F3_LBU, OP_LD, rd, rs1))
    }
    pub fn new_lhu(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Lhu(IType::new(immediate, F3_LHU, OP_LD, rd, rs1))
    }
    pub fn new_lwu(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Lwu(IType::new(immediate, F3_LWU, OP_LD, rd, rs1))
    }
    pub fn new_ecall() -> Instruction {
        Instruction::Ecall(IType::new(
            0,
            F3_SYSTEM,
            OP_SYSTEM,
            Register::Zero,
            Register::Zero,
        ))
    }
    pub fn new_ebreak() -> Instruction {
        Instruction::Ebreak(IType::new(
            0,
            F3_SYSTEM,
            OP_SYSTEM,
            Register::Zero,
            Register::from(1), // 000000000001
        ))
    }
    pub fn new_jalr(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Jalr(IType::new(immediate, F3_JALR, OP_JALR, rd, rs1))
    }
    pub fn new_sb(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Sb(SType::new(immediate, F3_SB, OP_SD, rs1, rs2))
    }
    pub fn new_sh(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Sh(SType::new(immediate, F3_SH, OP_SD, rs1, rs2))
    }
    pub fn new_sw(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Sw(SType::new(immediate, F3_SW, OP_SD, rs1, rs2))
    }
    pub fn new_sd(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Sd(SType::new(immediate, F3_SD, OP_SD, rs1, rs2))
    }
    pub fn new_beq(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Beq(BType::new(immediate, F3_BEQ, OP_BRANCH, rs1, rs2))
    }
    pub fn new_bne(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Bne(BType::new(immediate, F3_BNE, OP_BRANCH, rs1, rs2))
    }
    pub fn new_blt(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Blt(BType::new(immediate, F3_BLT, OP_BRANCH, rs1, rs2))
    }
    pub fn new_bge(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Bge(BType::new(immediate, F3_BGE, OP_BRANCH, rs1, rs2))
    }
    pub fn new_bltu(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Bltu(BType::new(immediate, F3_BLTU, OP_BRANCH, rs1, rs2))
    }
    pub fn new_bgeu(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Bgeu(BType::new(immediate, F3_BGEU, OP_BRANCH, rs1, rs2))
    }
    pub fn new_jal(rd: Register, immediate: i32) -> Instruction {
        Instruction::Jal(JType::new(immediate, OP_JAL, rd))
    }
    pub fn new_lui(rd: Register, immediate: i32) -> Instruction {
        Instruction::Lui(UType::new(immediate, OP_LUI, rd))
    }
    pub fn new_auipc(rd: Register, immediate: i32) -> Instruction {
        Instruction::Lui(UType::new(immediate, OP_AUIPC, rd))
    }
    pub fn new_lrw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Lrw(RType::new(F7_LRW, F3_RV32A, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_scw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Scw(RType::new(F7_SCW, F3_RV32A, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoswapw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Amoswapw(RType::new(F7_AMOSWAPW, F3_RV32A, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_fence(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        // TODO: Revisit
        // TODO: Implement me properly (imm[11:0] split into fm, pred, succ)!
        Instruction::Fence(IType::new(immediate, F3_FENCE, OP_FENCE, rd, rs1))
    }
}
