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
    Lbu(IType),
    Lhu(IType),
    Lwu(IType),
    Ld(IType),

    // Store
    Sb(SType),
    Sh(SType),
    Sw(SType),
    Sd(SType),

    // OP-imm
    Addi(IType),
    Slti(IType),
    Sltiu(IType),
    Xori(IType),
    Ori(IType),
    Andi(IType),
    Slli(ShiftType),
    Srli(ShiftType),
    Srai(ShiftType),

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

    // Misc-mem
    Fence(FenceType),
    FenceI,

    // System
    Ecall(IType),
    Ebreak,
    Uret,
    Sret,
    Mret,
    Wfi,
    SfenceVma(RType),
    Csrrw(CsrType),
    Csrrs(CsrType),
    Csrrc(CsrType),
    Csrrwi(CsrIType),
    Csrrsi(CsrIType),
    Csrrci(CsrIType),

    // OP-imm 32
    Addiw(IType),
    Slliw(ShiftType),
    Srliw(ShiftType),
    Sraiw(ShiftType),

    // OP 32
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

    // Illegal
    Illegal,

    #[doc(hidden)]
    __Nonexhaustive,
}

// opcodes
const OP_LD: u32 = 3; // 0000011, I format (LD)
const OP_IMM: u32 = 19; // 0010011, I format (ADDI, NOP)
const OP_SD: u32 = 35; // 0100011, S format (SD)
const OP_OP: u32 = 51; // 0110011, R format (ADD, SUB, MUL, DIVU, REMU, SLTU)
const OP_LUI: u32 = 55; // 0110111, U format (LUI)
const OP_BRANCH: u32 = 99; // 1100011, B format (BEQ)
const OP_JALR: u32 = 103; // 1100111, I format (JALR)
const OP_JAL: u32 = 111; // 1101111, J format (JAL)
const OP_SYSTEM: u32 = 115; // 1110011, I format (ECALL)

// f3-codes
const F3_ADDI: u32 = 0; // 000
const F3_ADD: u32 = 0; // 000
const F3_SUB: u32 = 0; // 000
const F3_MUL: u32 = 0; // 000
const F3_DIVU: u32 = 5; // 101
const F3_REMU: u32 = 7; // 111
const F3_SLTU: u32 = 3; // 011
const F3_LD: u32 = 3; // 011
const F3_SD: u32 = 3; // 011
const F3_BEQ: u32 = 0; // 000
const F3_JALR: u32 = 0; // 000
const F3_ECALL: u32 = 0; // 000

// f7-codes
const F7_ADD: u32 = 0; // 0000000
const F7_MUL: u32 = 1; // 0000001
const F7_SUB: u32 = 32; // 0100000
const F7_DIVU: u32 = 1; // 0000001
const F7_REMU: u32 = 1; // 0000001
const F7_SLTU: u32 = 0; // 0000000

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
    pub fn new_ld(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Ld(IType::new(immediate, F3_LD, OP_LD, rd, rs1))
    }
    pub fn new_ecall() -> Instruction {
        Instruction::Ecall(IType::new(
            0,
            F3_ECALL,
            OP_SYSTEM,
            Register::Zero,
            Register::Zero,
        ))
    }
    pub fn new_jalr(rd: Register, rs1: Register, immediate: i32) -> Instruction {
        Instruction::Jalr(IType::new(immediate, F3_JALR, OP_JALR, rd, rs1))
    }
    pub fn new_sd(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Sd(SType::new(immediate, F3_SD, OP_SD, rs1, rs2))
    }
    pub fn new_beq(rs1: Register, rs2: Register, immediate: i32) -> Instruction {
        Instruction::Beq(BType::new(immediate, F3_BEQ, OP_BRANCH, rs1, rs2))
    }
    pub fn new_jal(rd: Register, immediate: i32) -> Instruction {
        Instruction::Jal(JType::new(immediate, OP_JAL, rd))
    }
    pub fn new_lui(rd: Register, immediate: i32) -> Instruction {
        Instruction::Lui(UType::new(immediate, OP_LUI, rd))
    }
}
