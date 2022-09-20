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
    Slti(IType),
    Sltiu(IType),
    Xori(IType),
    Ori(IType),
    Andi(IType),
    Slli(IType),
    SrliSrai(IType),

    // OP-imm32
    Addiw(IType),
    Slliw(IType),
    Srliw(IType),
    Sraiw(IType),

    // OP
    Add(RType),
    Sub(RType),
    Sll(RType),
    Slt(RType),
    Sltu(RType),
    Xor(RType),
    Srl(RType),
    Sra(RType),
    Or(RType),
    And(RType),
    Mul(RType),
    Mulh(RType),
    Mulhsu(RType),
    Mulhu(RType),
    Div(RType),
    Divu(RType),
    Rem(RType),
    Remu(RType),

    // OP32
    Addw(RType),
    Subw(RType),
    Sllw(RType),
    Srlw(RType),
    Sraw(RType),
    Mulw(RType),
    Divw(RType),
    Divuw(RType),
    Remw(RType),
    Remuw(RType),

    // System
    Ecall(IType),
    Ebreak(IType),

    // Amo
    Lrw(RType),
    Scw(RType),
    Amoswapw(RType),
    Amoaddw(RType),
    Amoxorw(RType),
    Amoandw(RType),
    Amoorw(RType),
    Amominw(RType),
    Amomaxw(RType),
    Amominuw(RType),
    Amomaxuw(RType),
    Lrd(RType),
    Scd(RType),
    Amoswapd(RType),
    Amoaddd(RType),
    Amoxord(RType),
    Amoandd(RType),
    Amoord(RType),
    Amomind(RType),
    Amomaxd(RType),
    Amominud(RType),
    Amomaxud(RType),
}

// opcodes
const OP_LD: u32 = 3; // 0000011, I format (LD)
// 0010011, I format (ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI, NOP)
const OP_IMM: u32 = 19;
const OP_IMM32: u32 = 27; // 0011011, I format (ADDIW, SLLIW, SRLIW, SRAIW)
const OP_SD: u32 = 35; // 0100011, S format (SD)
const OP_OP: u32 = 51; // 0110011, R format (ADD, SUB, MUL, DIVU, REMU, SLTU)
// 0111011, R format (ADDW, SUBW, SSLW, SRLW, SRAW, MULW, DIVW, DIVUW, REMW
// REMUW)
const OP_OP32: u32 = 59;
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
const F3_SLTI: u32 = 2; // 010
const F3_SLTIU: u32 = 3; // 011
const F3_XORI: u32 = 4; // 100
const F3_ORI: u32 = 6; // 110
const F3_ANDI: u32 = 7; // 111
const F3_SLLI: u32 = 1; // 001
const F3_SRLI_SRAI: u32 = 5; // 101
const F3_ADDIW: u32 = 0; // 000
const F3_SLLIW: u32 = 1; // 001
const F3_SRLIW: u32 = 5; // 101
const F3_SRAIW: u32 = 5; // 101
const F3_ADD_ADDW: u32 = 0; // 000
const F3_SUB_SUBW: u32 = 0; // 000
const F3_SLL_SLLW: u32 = 1; // 001
const F3_SLT: u32 = 2; // 010
const F3_SLTU: u32 = 3; // 011
const F3_XOR: u32 = 4; // 100
const F3_SRL_SRLW: u32 = 5; // 101
const F3_SRA_SRAW: u32 = 5; // 101
const F3_OR: u32 = 6; // 110
const F3_AND: u32 = 7; // 111
const F3_MUL_MULW: u32 = 0; // 000
const F3_MULH: u32 = 1; // 001
const F3_MULHSU: u32 = 2; // 010
const F3_MULHU: u32 = 3; // 011
const F3_DIV_DIVW: u32 = 4; // 100
const F3_DIVU_DIVUW: u32 = 5; // 101
const F3_REM_REMW: u32 = 6; // 110
const F3_REMU_REMUW: u32 = 7; // 111
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
const F3_AMO32: u32 = 2; //010
const F3_AMO64: u32 = 3; //011
const F3_FENCE: u32 = 0; // 000

// f7-codes
const F7_ADD_ADDW: u32 = 0; // 0000000
const F7_SUB_SUBW: u32 = 32; // 0100000
const F7_SLL_SLLW: u32 = 0; // 0000000
const F7_SLT: u32 = 0; // 0000000
const F7_SLTU: u32 = 0; // 0000000
const F7_XOR: u32 = 0; // 0000000
const F7_SRL_SRLW: u32 = 0; // 0000000
const F7_SRA_SRAW: u32 = 32; // 0100000
const F7_OR: u32 = 0; // 0000000
const F7_AND: u32 = 0; // 0000000
const F7_MUL_MULW: u32 = 1; // 0000001
const F7_MULH: u32 = 1; // 0000001
const F7_MULHSU: u32 = 1; // 0000001
const F7_MULHU: u32 = 1; // 0000001
const F7_DIV_DIVW: u32 = 1; // 0000001
const F7_DIVU_DIVUW: u32 = 1; // 0000001
const F7_REM_REMW: u32 = 1; // 0000001
const F7_REMU_REMUW: u32 = 1; // 0000001
const F7_LRW_LRD: u32 = 2; // 00010 aq rl
const F7_SCW_SCD: u32 = 3; // 00011 aq rl
const F7_AMOSWAPW_AMOSWAPD: u32 = 3; // 00001 aq rl
const F7_AMOADDW_AMOADDD: u32 = 0; // 00000 aq rl
const F7_AMOXORW_AMOXORD: u32 = 4; // 00100 aq rl
const F7_AMOANDW_AMOANDD: u32 = 12; // 01100 aq rl
const F7_AMOORW_AMOORD: u32 = 8; // 01000 aq rl
const F7_AMOMINW_AMOMIND: u32 = 16; // 10000 aq rl
const F7_AMOMAXW_AMOMAXD: u32 = 20; // 10100 aq rl
const F7_AMOMINUW_AMOMINUD: u32 = 24; // 11000 aq rl
const F7_AMOMAXUW_AMOMAXUD: u32 = 28; // 11100 aq rl

impl Instruction {
    pub fn new_nop() -> Instruction {
        Self::new_addi(Register::Zero, Register::Zero, 0)
    }
    pub fn new_add(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Add(RType::new(F7_ADD_ADDW, F3_ADD_ADDW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_sub(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Sub(RType::new(F7_SUB_SUBW, F3_SUB_SUBW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_sll(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Sll(RType::new(F7_SLL_SLLW, F3_SLL_SLLW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_slt(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Slt(RType::new(F7_SLT, F3_SLT, OP_OP, rs1, rs2, rd))
    }
    pub fn new_sltu(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Sltu(RType::new(F7_SLTU, F3_SLTU, OP_OP, rs1, rs2, rd))
    }
    pub fn new_xor(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Xor(RType::new(F7_XOR, F3_XOR, OP_OP, rs1, rs2, rd))
    }
    pub fn new_srl(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Srl(RType::new(F7_SRL_SRLW, F3_SRL_SRLW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_sra(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Sra(RType::new(F7_SRA_SRAW, F3_SRA_SRAW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_or(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Or(RType::new(F7_OR, F3_OR, OP_OP, rs1, rs2, rd))
    }
    pub fn new_and(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::And(RType::new(F7_AND, F3_AND, OP_OP, rs1, rs2, rd))
    }
    pub fn new_mul(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Mul(RType::new(F7_MUL_MULW, F3_MUL_MULW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_mulh(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Mulh(RType::new(F7_MULH, F3_MULH, OP_OP, rs1, rs2, rd))
    }
    pub fn new_mulhsu(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Mulhsu(RType::new(F7_MULHSU, F3_MULHSU, OP_OP, rs1, rs2, rd))
    }
    pub fn new_mulhu(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Mulhu(RType::new(F7_MULHU, F3_MULHU, OP_OP, rs1, rs2, rd))
    }
    pub fn new_div(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Div(RType::new(F7_DIV_DIVW, F3_DIV_DIVW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_divu(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Divu(RType::new(F7_DIVU_DIVUW, F3_DIVU_DIVUW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_rem(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Rem(RType::new(F7_REM_REMW, F3_REM_REMW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_remu(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Remu(RType::new(F7_REMU_REMUW, F3_REMU_REMUW, OP_OP, rs1, rs2, rd))
    }
    pub fn new_addw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Addw(RType::new(F7_ADD_ADDW, F3_ADD_ADDW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_subw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Subw(RType::new(F7_SUB_SUBW, F3_SUB_SUBW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_sllw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Sllw(RType::new(F7_SLL_SLLW, F3_SLL_SLLW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_srlw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Srlw(RType::new(F7_SRL_SRLW, F3_SRL_SRLW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_sraw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Sraw(RType::new(F7_SRA_SRAW, F3_SRA_SRAW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_mulw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Mulw(RType::new(F7_MUL_MULW, F3_MUL_MULW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_divw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Divw(RType::new(F7_DIV_DIVW, F3_DIV_DIVW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_divuw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Divuw(RType::new(F7_DIVU_DIVUW, F3_DIVU_DIVUW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_remw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Remw(RType::new(F7_REM_REMW, F3_REM_REMW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_remuw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Remuw(RType::new(F7_REMU_REMUW, F3_REMU_REMUW, OP_OP32, rs1, rs2, rd))
    }
    pub fn new_addi(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Addi(IType::new(immediate, F3_ADDI, OP_IMM, rd, rs1))
    }
    pub fn new_slti(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Slti(IType::new(immediate, F3_SLTI, OP_IMM, rd, rs1))
    }
    pub fn new_sltiu(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Sltiu(IType::new(immediate, F3_SLTIU, OP_IMM, rd, rs1))
    }
    pub fn new_xori(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Xori(IType::new(immediate, F3_XORI, OP_IMM, rd, rs1))
    }
    pub fn new_ori(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Ori(IType::new(immediate, F3_ORI, OP_IMM, rd, rs1))
    }
    pub fn new_andi(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Andi(IType::new(immediate, F3_ANDI, OP_IMM, rd, rs1))
    }
    pub fn new_slli(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Slli(IType::new(immediate, F3_SLLI, OP_IMM, rd, rs1))
    }
    pub fn new_srli_srai(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::SrliSrai(IType::new(immediate, F3_SRLI_SRAI, OP_IMM, rd, rs1))
    }
    pub fn new_addiw(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Addiw(IType::new(immediate, F3_ADDIW, OP_IMM32, rd, rs1))
    }
    pub fn new_slliw(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Slliw(IType::new(immediate, F3_SLLIW, OP_IMM32, rd, rs1))
    }
    pub fn new_srliw(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Srliw(IType::new(immediate, F3_SRLIW, OP_IMM32, rd, rs1))
    }
    pub fn new_sraiw(rd: Register, rs1: Register, immediate: i32) -> Instruction {
      Instruction::Sraiw(IType::new(immediate, F3_SRAIW, OP_IMM32, rd, rs1))
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
        Instruction::Lrw(RType::new(F7_LRW_LRD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_scw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Scw(RType::new(F7_SCW_SCD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoswapw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Amoswapw(RType::new(F7_AMOSWAPW_AMOSWAPD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoaddw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Amoaddw(RType::new(F7_AMOADDW_AMOADDD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoxorw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Amoxorw(RType::new(F7_AMOXORW_AMOXORD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoandw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amoandw(RType::new(F7_AMOANDW_AMOANDD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoorw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amoorw(RType::new(F7_AMOORW_AMOORD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amominw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amominw(RType::new(F7_AMOMINW_AMOMIND, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amomaxw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amomaxw(RType::new(F7_AMOMAXW_AMOMAXD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amominuw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amominuw(RType::new(F7_AMOMINUW_AMOMINUD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amomaxuw(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amomaxuw(RType::new(F7_AMOMAXUW_AMOMAXUD, F3_AMO32, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_lrd(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Lrd(RType::new(F7_LRW_LRD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_scd(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Scd(RType::new(F7_SCW_SCD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoswapd(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Amoswapd(RType::new(F7_AMOSWAPW_AMOSWAPD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoaddd(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Amoaddd(RType::new(F7_AMOADDW_AMOADDD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoxord(rd: Register, rs1: Register, rs2: Register) -> Instruction {
        Instruction::Amoxord(RType::new(F7_AMOXORW_AMOXORD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoandd(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amoandd(RType::new(F7_AMOANDW_AMOANDD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amoord(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amoord(RType::new(F7_AMOORW_AMOORD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amomind(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amomind(RType::new(F7_AMOMINW_AMOMIND, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amomaxd(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amomaxd(RType::new(F7_AMOMAXW_AMOMAXD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amominud(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amominud(RType::new(F7_AMOMINUW_AMOMINUD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_amomaxud(rd: Register, rs1: Register, rs2: Register) -> Instruction {
      Instruction::Amomaxud(RType::new(F7_AMOMAXUW_AMOMAXUD, F3_AMO64, OP_AMO, rs1, rs2, rd))
    }
    pub fn new_fence(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        // TODO: Revisit
        // TODO: Implement me properly (imm[11:0] split into fm, pred, succ)!
        Instruction::Fence(IType::new(immediate, F3_FENCE, OP_FENCE, rd, rs1))
    }
}
