use core::fmt;

use crate::Register;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct RType(pub u32);
impl RType {
    pub(crate) fn new(
        funct7: u32,
        funct3: u32,
        opcode: u32,
        rd: Register,
        rs1: Register,
        rs2: Register,
    ) -> Self {
        assert!(funct7 < 2_u32.pow(7));
        assert!(funct3 < 2_u32.pow(3));
        assert!(opcode < 2_u32.pow(7));

        let rs2: u32 = rs2.into();
        let rs1: u32 = rs1.into();
        let rd: u32 = rd.into();

        Self(((((funct7 << 5) + rs2 << 5) + rs1 << 3) + funct3 << 5) + rd << 7 + opcode)
    }
    pub fn rs2(&self) -> Register {
        Register::from((self.0 >> 20) & 0x1f)
    }
    pub fn rs1(&self) -> Register {
        Register::from((self.0 >> 15) & 0x1f)
    }
    pub fn rd(&self) -> Register {
        Register::from((self.0 >> 7) & 0x1f)
    }
}

impl fmt::Debug for RType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rd: {:?}, rs1: {:?}, rs2: {:?}",
            self.rd(),
            self.rs1(),
            self.rs2()
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct CsrType(pub u32);
impl CsrType {
    pub fn csr(&self) -> u32 {
        self.0 >> 20
    }
    pub fn rs1(&self) -> u32 {
        (self.0 >> 15) & 0x1f
    }
    pub fn rd(&self) -> u32 {
        (self.0 >> 7) & 0x1f
    }
}

impl fmt::Debug for CsrType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rd: {}, rs1: {}, csr: {}",
            self.rd(),
            self.rs1(),
            self.csr()
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct CsrIType(pub u32);
impl CsrIType {
    pub fn csr(&self) -> u32 {
        self.0 >> 20
    }
    pub fn zimm(&self) -> u32 {
        (self.0 >> 15) & 0x1f
    }
    pub fn rd(&self) -> u32 {
        (self.0 >> 7) & 0x1f
    }
}

impl fmt::Debug for CsrIType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rd: {}, zimm: {}, csr: {}",
            self.rd(),
            self.zimm(),
            self.csr()
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct IType(pub u32);
impl IType {
    pub(crate) fn new(
        immediate: i32,
        funct3: u32,
        opcode: u32,
        rd: Register,
        rs1: Register,
    ) -> Self {
        assert!(-2_i32.pow(11) <= immediate && immediate < 2_i32.pow(11));
        assert!(funct3 < 2_u32.pow(3));
        assert!(opcode < 2_u32.pow(7));

        let rd: u32 = rd.into();
        let rs1: u32 = rs1.into();

        let immediate = sign_shrink(immediate, 12);

        Self(((((immediate << 5) + rs1 << 3) + funct3 << 5) + rd << 5) + opcode)
    }
    pub fn imm(&self) -> i32 {
        sign_extend(self.0 >> 20, 12) as i32
    }
    pub fn rs1(&self) -> Register {
        Register::from((self.0 >> 15) & 0x1f)
    }
    pub fn rd(&self) -> Register {
        Register::from((self.0 >> 7) & 0x1f)
    }
}

impl fmt::Debug for IType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rd: {:?}, rs1: {:?}, imm: {}",
            self.rd(),
            self.rs1(),
            self.imm()
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct SType(pub u32);
impl SType {
    pub(crate) fn new(
        immediate: i32,
        funct3: u32,
        opcode: u32,
        rs1: Register,
        rs2: Register,
    ) -> Self {
        assert!(-2_i32.pow(11) <= immediate && immediate < 2_i32.pow(11));
        assert!(funct3 < 2_u32.pow(3));
        assert!(opcode < 2_u32.pow(7));

        let rs1: u32 = rs1.into();
        let rs2: u32 = rs2.into();

        let immediate = sign_shrink(immediate, 12);

        let imm1 = get_bits(immediate, 5, 7);
        let imm2 = get_bits(immediate, 0, 5);

        Self(((((((imm1 << 5) + rs2 << 5) + rs1 << 3) + funct3 << 5) + imm2) << 7) + opcode)
    }
    pub fn imm(&self) -> i32 {
        sign_extend(((self.0 >> 20) & 0xfe0) | ((self.0 >> 7) & 0x1f), 12) as i32
    }
    pub fn rs1(&self) -> Register {
        Register::from((self.0 >> 15) & 0x1f)
    }
    pub fn rs2(&self) -> Register {
        Register::from((self.0 >> 20) & 0x1f)
    }
}

impl fmt::Debug for SType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "imm: {}, rs1: {:?}, rs2: {:?}",
            self.imm(),
            self.rs1(),
            self.rs2()
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct BType(pub u32);
impl BType {
    pub(crate) fn new(
        immediate: i32,
        funct3: u32,
        opcode: u32,
        rs1: Register,
        rs2: Register,
    ) -> Self {
        assert!(-2_i32.pow(12) <= immediate && immediate < 2_i32.pow(12));
        assert!(funct3 < 2_u32.pow(3));
        assert!(opcode < 2_u32.pow(7));

        let rs1: u32 = rs1.into();
        let rs2: u32 = rs2.into();

        let immediate = sign_shrink(immediate, 13);

        let imm1 = get_bits(immediate, 12, 1);
        let imm2 = get_bits(immediate, 5, 6);
        let imm3 = get_bits(immediate, 1, 4);
        let imm4 = get_bits(immediate, 11, 1);

        Self(
            ((((((((((imm1 << 6) + imm2 << 5) + rs2 << 5) + rs1 << 3) + funct3) << 4) + imm3)
                << 1)
                + imm4)
                << 7)
                + opcode,
        )
    }
    pub fn imm(&self) -> i32 {
        sign_extend(
            ((self.0 & 0x8000_0000) >> 19)
                | ((self.0 & 0x7e00_0000) >> 20)
                | ((self.0 & 0x0000_0f00) >> 7)
                | ((self.0 & 0x0000_0080) << 4),
            13,
        ) as i32
    }
    pub fn rs1(&self) -> Register {
        Register::from((self.0 >> 15) & 0x1f)
    }
    pub fn rs2(&self) -> Register {
        Register::from((self.0 >> 20) & 0x1f)
    }
}

impl fmt::Debug for BType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "imm: {}, rs1: {:?}, rs2: {:?}",
            self.imm(),
            self.rs1(),
            self.rs2()
        )
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct UType(pub u32);
impl UType {
    pub(crate) fn new(immediate: i32, opcode: u32, rd: Register) -> Self {
        assert!(immediate > 0 && immediate < 2_i32.pow(21));
        assert!(opcode < 2_u32.pow(7));

        let rd: u32 = rd.into();

        let immediate = sign_shrink(immediate, 20);

        Self(((immediate << 5) + rd << 7) + opcode)
    }
    pub fn imm(&self) -> u32 {
        (self.0 & 0xfffff000) >> 12
    }
    pub fn rd(&self) -> Register {
        Register::from((self.0 >> 7) & 0x1f)
    }
}

impl fmt::Debug for UType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rd: {:?}, imm: {}", self.rd(), self.imm())
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct JType(pub u32);
impl JType {
    pub(crate) fn new(immediate: i32, opcode: u32, rd: Register) -> Self {
        assert!(-2_i32.pow(20) <= immediate && immediate < 2_i32.pow(20));
        assert!(opcode < 2_u32.pow(7));

        let rd: u32 = rd.into();

        let immediate = sign_shrink(immediate, 21);

        let imm1 = get_bits(immediate, 20, 1);
        let imm2 = get_bits(immediate, 1, 10);
        let imm3 = get_bits(immediate, 11, 1);
        let imm4 = get_bits(immediate, 12, 8);

        Self(((((((imm1 << 10) + imm2 << 1) + imm3 << 8) + imm4 << 5) + rd) << 7) + opcode)
    }
    pub fn imm(&self) -> i32 {
        sign_extend(
            ((self.0 & 0x8000_0000) >> 11)
                | ((self.0 & 0x7fe0_0000) >> 20)
                | ((self.0 & 0x0010_0000) >> 9)
                | (self.0 & 0x000f_f000),
            21,
        ) as i32
    }
    pub fn rd(&self) -> Register {
        Register::from((self.0 >> 7) & 0x1f)
    }
}

impl fmt::Debug for JType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rd: {:?}, imm: {}", self.rd(), self.imm())
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct FenceType(pub u32);
impl FenceType {
    pub fn pred(&self) -> u32 {
        (self.0 >> 24) & 0xf
    }
    pub fn succ(&self) -> u32 {
        (self.0 >> 20) & 0xf
    }
}

impl fmt::Debug for FenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pred: {}, succ: {}", self.pred(), self.succ())
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct ShiftType(pub u32);
impl ShiftType {
    pub fn shamt(&self) -> u32 {
        (self.0 >> 20) & 0x3f
    }
    pub fn rs1(&self) -> u32 {
        (self.0 >> 15) & 0x1f
    }
    pub fn rd(&self) -> u32 {
        (self.0 >> 7) & 0x1f
    }
}

impl fmt::Debug for ShiftType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rd: {}, rs1: {}, shamt: {}",
            self.rd(),
            self.rs1(),
            self.shamt()
        )
    }
}

fn get_bits(n: u32, i: u32, b: u32) -> u32 {
    assert!(0 < b && b <= i + b && i + b < 32);

    if i == 0 {
        n % 2_u32.pow(b)
    } else {
        // shift to-be-loaded bits all the way to the left
        // to reset all bits to the left of them, then
        // shift to-be-loaded bits all the way to the right and return
        (n << 32 - (i + b)) >> 32 - b
    }
}

fn sign_extend(n: u32, b: u32) -> i32 {
    assert!(n <= 2_u32.pow(b));
    assert!(0 < b && b < 32);

    unsafe {
        core::mem::transmute({
            if n < 2_u32.pow(b - 1) {
                n
            } else {
                n.wrapping_sub(2_u32.pow(b))
            }
        })
    }
}

fn sign_shrink(immediate: i32, sign: u32) -> u32 {
    assert!(sign > 0 && sign < 32);

    get_bits(immediate as u32, 0, sign)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rtype() {
        assert_eq!(RType(0x00c58633).rs1(), Register::A1); // add x12,x11,x12
        assert_eq!(RType(0x40b50533).rs1(), Register::A0); // sub x10,x10,x11
        assert_eq!(RType(0x00209f33).rs1(), Register::Ra); // sll x30,x1,x2
        assert_eq!(RType(0x0020af33).rs1(), Register::Ra); // slt x30,x1,x2
        assert_eq!(RType(0x0020bf33).rs1(), Register::Ra); // sltu x30,x1,x2
        assert_eq!(RType(0x00f647b3).rs1(), Register::A2); // xor x15,x12,x15
        assert_eq!(RType(0x0020d0b3).rs1(), Register::Ra); // srl x1,x1,x2
        assert_eq!(RType(0x4020df33).rs1(), Register::Ra); // sra x30,x1,x2
        assert_eq!(RType(0x00b7e5b3).rs1(), Register::A5); // or x11,x15,x11
        assert_eq!(RType(0x00d57533).rs1(), Register::A0); // and x10,x10,x13

        assert_eq!(RType(0x00c58633).rs2(), Register::A2); // add x12,x11,x12
        assert_eq!(RType(0x40b50533).rs2(), Register::A1); // sub x10,x10,x11
        assert_eq!(RType(0x00209f33).rs2(), Register::Sp); // sll x30,x1,x2
        assert_eq!(RType(0x0020af33).rs2(), Register::Sp); // slt x30,x1,x2
        assert_eq!(RType(0x0020bf33).rs2(), Register::Sp); // sltu x30,x1,x2
        assert_eq!(RType(0x00f647b3).rs2(), Register::A5); // xor x15,x12,x15
        assert_eq!(RType(0x0020d0b3).rs2(), Register::Sp); // srl x1,x1,x2
        assert_eq!(RType(0x4020df33).rs2(), Register::Sp); // sra x30,x1,x2
        assert_eq!(RType(0x00b7e5b3).rs2(), Register::A1); // or x11,x15,x11
        assert_eq!(RType(0x00d57533).rs2(), Register::A3); // and x10,x10,x13

        assert_eq!(RType(0x00c58633).rd(), Register::A2); // add x12,x11,x12
        assert_eq!(RType(0x40b50533).rd(), Register::A0); // sub x10,x10,x11
        assert_eq!(RType(0x00209f33).rd(), Register::T5); // sll x30,x1,x2
        assert_eq!(RType(0x0020af33).rd(), Register::T5); // slt x30,x1,x2
        assert_eq!(RType(0x0020bf33).rd(), Register::T5); // sltu x30,x1,x2
        assert_eq!(RType(0x00f647b3).rd(), Register::A5); // xor x15,x12,x15
        assert_eq!(RType(0x0020d0b3).rd(), Register::Ra); // srl x1,x1,x2
        assert_eq!(RType(0x4020df33).rd(), Register::T5); // sra x30,x1,x2
        assert_eq!(RType(0x00b7e5b3).rd(), Register::A1); // or x11,x15,x11
        assert_eq!(RType(0x00d57533).rd(), Register::A0); // and x10,x10,x13
    }

    #[test]
    fn csrtype() {
        assert_eq!(CsrType(0x10569073).rs1(), 13); // csrrw x0,stvec,x13
        assert_eq!(CsrType(0x18079073).rs1(), 15); // csrrw x0,satp,x15
        assert_eq!(CsrType(0x10551073).rs1(), 10); // csrrw x0,stvec,x10
        assert_eq!(CsrType(0x1007a073).rs1(), 15); // csrrs x0,sstatus,x15
        assert_eq!(CsrType(0x1006a073).rs1(), 13); // csrrs x0,sstatus,x13
        assert_eq!(CsrType(0x1004b073).rs1(), 9); // csrrc x0,sstatus,x9
        assert_eq!(CsrType(0x100db073).rs1(), 27); // csrrc x0,sstatus,x27
        assert_eq!(CsrType(0x1006b073).rs1(), 13); // csrrc x0,sstatus,x13

        assert_eq!(CsrType(0x10569073).rd(), 0); // csrrw x0,stvec,x13
        assert_eq!(CsrType(0x18079073).rd(), 0); // csrrw x0,satp,x15
        assert_eq!(CsrType(0x10551073).rd(), 0); // csrrw x0,stvec,x10
        assert_eq!(CsrType(0x1007a073).rd(), 0); // csrrs x0,sstatus,x15

        assert_eq!(CsrType(0x10569073).csr(), 0x105); // csrrw x0,stvec,x13
        assert_eq!(CsrType(0x18079073).csr(), 0x180); // csrrw x0,satp,x15
        assert_eq!(CsrType(0x10551073).csr(), 0x105); // csrrw x0,stvec,x10
        assert_eq!(CsrType(0x1007a073).csr(), 0x100); // csrrs x0,sstatus,x15
    }

    #[test]
    fn csritype() {
        assert_eq!(CsrIType(0x14005073).zimm(), 0); // csrrwi x0,sscratch,0
        assert_eq!(CsrIType(0x10016073).zimm(), 2); // csrrsi x0,sstatus,2
        assert_eq!(CsrIType(0x100176f3).zimm(), 2); // csrrci x13,sstatus,2
        assert_eq!(CsrIType(0x10017773).zimm(), 2); // csrrci x14,sstatus,2

        assert_eq!(CsrIType(0x14005073).rd(), 0); // csrrwi x0,sscratch,0
        assert_eq!(CsrIType(0x10016073).rd(), 0); // csrrsi x0,sstatus,2
        assert_eq!(CsrIType(0x100176f3).rd(), 13); // csrrci x13,sstatus,2
        assert_eq!(CsrIType(0x10017773).rd(), 14); // csrrci x14,sstatus,2

        assert_eq!(CsrIType(0x14005073).csr(), 0x140); // csrrwi x0,sscratch,0
        assert_eq!(CsrIType(0x10016073).csr(), 0x100); // csrrsi x0,sstatus,2
    }

    #[test]
    fn itype() {
        assert_eq!(IType(0x02008283).rd(), Register::T0); // lb x5,32(x1)
        assert_eq!(IType(0x00708283).rd(), Register::T0); // lb x5,7(x1)
        assert_eq!(IType(0x00108f03).rd(), Register::T5); // lb x30,1(x1)
        assert_eq!(IType(0x00411f03).rd(), Register::T5); // Lh x30,4(x2)
        assert_eq!(IType(0x00611f03).rd(), Register::T5); // Lh x30,6(x2)
        assert_eq!(IType(0x00811f03).rd(), Register::T5); // Lh x30,8(x2)
        assert_eq!(IType(0x02052403).rd(), Register::Fp); // Lw x8,32(x10)
        assert_eq!(IType(0x03452683).rd(), Register::A3); // Lw x13,52(x10)
        assert_eq!(IType(0x0006a703).rd(), Register::A4); // Lw x14,0(x13)
        assert_eq!(IType(0x0006c783).rd(), Register::A5); // Lbu x15,0(x13)
        assert_eq!(IType(0x0006c703).rd(), Register::A4); // Lbu x14,0(x13)
        assert_eq!(IType(0x0007c683).rd(), Register::A3); // Lbu x13,0(x15)
        assert_eq!(IType(0x0060df03).rd(), Register::T5); // Lhu x30,6(x1)
        assert_eq!(IType(0xffe0df03).rd(), Register::T5); // Lhu x30,-2(x1)
        assert_eq!(IType(0x0002d303).rd(), Register::T1); // Lhu x6,0(x5)
        assert_eq!(IType(0x00346303).rd(), Register::T1); // Lwu x6,3(x8)
        assert_eq!(IType(0x0080ef03).rd(), Register::T5); // Lwu x30,8(x1)
        assert_eq!(IType(0x0000ef03).rd(), Register::T5); // Lwu x30,0(x1)
        assert_eq!(IType(0x01853683).rd(), Register::A3); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).rd(), Register::S8); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).rd(), Register::A4); // Ld x14,0(x15)

        assert_eq!(IType(0x02008283).rs1(), Register::Ra); // lb x5,32(x1)
        assert_eq!(IType(0x00708283).rs1(), Register::Ra); // lb x5,7(x1)
        assert_eq!(IType(0x00108f03).rs1(), Register::Ra); // lb x30,1(x1)
        assert_eq!(IType(0x00411f03).rs1(), Register::Sp); // Lh x30,4(x2)
        assert_eq!(IType(0x00611f03).rs1(), Register::Sp); // Lh x30,6(x2)
        assert_eq!(IType(0x00811f03).rs1(), Register::Sp); // Lh x30,8(x2)
        assert_eq!(IType(0x02052403).rs1(), Register::A0); // Lw x8,32(x10)
        assert_eq!(IType(0x03452683).rs1(), Register::A0); // Lw x13,52(x10)
        assert_eq!(IType(0x0006a703).rs1(), Register::A3); // Lw x14,0(x13)
        assert_eq!(IType(0x0006c783).rs1(), Register::A3); // Lbu x15,0(x13)
        assert_eq!(IType(0x0006c703).rs1(), Register::A3); // Lbu x14,0(x13)
        assert_eq!(IType(0x0007c683).rs1(), Register::A5); // Lbu x13,0(x15)
        assert_eq!(IType(0x0060df03).rs1(), Register::Ra); // Lhu x30,6(x1)
        assert_eq!(IType(0xffe0df03).rs1(), Register::Ra); // Lhu x30,-2(x1)
        assert_eq!(IType(0x0002d303).rs1(), Register::T0); // Lhu x6,0(x5)
        assert_eq!(IType(0x00346303).rs1(), Register::Fp); // Lwu x6,3(x8)
        assert_eq!(IType(0x0080ef03).rs1(), Register::Ra); // Lwu x30,8(x1)
        assert_eq!(IType(0x0000ef03).rs1(), Register::Ra); // Lwu x30,0(x1)
        assert_eq!(IType(0x01853683).rs1(), Register::A0); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).rs1(), Register::Sp); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).rs1(), Register::A5); // Ld x14,0(x15)

        assert_eq!(IType(0x02008283).imm(), 32); // lb x5,32(x1)
        assert_eq!(IType(0x00708283).imm(), 7); // lb x5,7(x1)
        assert_eq!(IType(0x00108f03).imm(), 1); // lb x30,1(x1)
        assert_eq!(IType(0x00411f03).imm(), 4); // Lh x30,4(x2)
        assert_eq!(IType(0x00611f03).imm(), 6); // Lh x30,6(x2)
        assert_eq!(IType(0x00811f03).imm(), 8); // Lh x30,8(x2)
        assert_eq!(IType(0x02052403).imm(), 32); // Lw x8,32(x10)
        assert_eq!(IType(0x03452683).imm(), 52); // Lw x13,52(x10)
        assert_eq!(IType(0x0006a703).imm(), 0); // Lw x14,0(x13)
        assert_eq!(IType(0x0006c783).imm(), 0); // Lbu x15,0(x13)
        assert_eq!(IType(0x0006c703).imm(), 0); // Lbu x14,0(x13)
        assert_eq!(IType(0x0007c683).imm(), 0); // Lbu x13,0(x15)
        assert_eq!(IType(0x0060df03).imm(), 6); // Lhu x30,6(x1)
        assert_eq!(IType(0xffe0df03).imm(), -2i32); // Lhu x30,-2(x1)
        assert_eq!(IType(0x0002d303).imm(), 0); // Lhu x6,0(x5)
        assert_eq!(IType(0x00346303).imm(), 3); // Lwu x6,3(x8)
        assert_eq!(IType(0x0080ef03).imm(), 8); // Lwu x30,8(x1)
        assert_eq!(IType(0x0000ef03).imm(), 0); // Lwu x30,0(x1)
        assert_eq!(IType(0x01853683).imm(), 24); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).imm(), 32); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).imm(), 0); // Ld x14,0(x15)
    }

    #[test]
    #[allow(overflowing_literals)]
    fn btype() {
        assert_eq!(BType(0x0420c063).imm(), 0x80002ea4 - 0x80002e64); // blt x1,x2,80002ea4
        assert_eq!(BType(0x06f58063).imm(), 0x80002724 - 0x800026c4); // beq x11,x15,80002724
        assert_eq!(BType(0x06f58063).imm(), 0x80002648 - 0x800025e8); // beq x11,x15,80002648
        assert_eq!(BType(0x00050a63).imm(), 0x800024e8 - 0x800024d4); // beq x10,x0,800024e8
        assert_eq!(BType(0x03ff0663).imm(), 0x80000040 - 0x80000014); // beq x30,x31,80000040
        assert_eq!(BType(0xfe069ae3).imm(), 0x800026f0i32 - 0x800026fci32); // bne x13,x0,800026f0
        assert_eq!(BType(0x00f5f463).imm(), 0x80002290 - 0x80002288); // bgeu x11,x15,80002290
        assert_eq!(BType(0x1e301c63).imm(), 0x800003c4 - 0x800001cc); // bne x0,x3,800003c4
        assert_eq!(BType(0x13df1063).imm(), 0x800030dc - 0x80002fbc); // bne x30,x29,800030dc
        assert_eq!(BType(0x37df1263).imm(), 0x80002f90 - 0x80002c2c); // bne x30,x29,80002f90
    }

    #[test]
    fn utype() {
        //let i = Instruction::new_lui(Register::Ra, 0xfffff);
        //if let Instruction::Lui(utype) = i {
        //assert_eq!(utype.imm(), 0xfffff_u32);
        //}
        assert_eq!(UType(0x00001a37).rd(), Register::S4); // lui x20,0x1
        assert_eq!(UType(0x800002b7).rd(), Register::T0); // lui x5,0x80000
        assert_eq!(UType(0x212120b7).rd(), Register::Ra); // lui x1,0x21212
        assert_eq!(UType(0xffffe517).rd(), Register::A0); // auipc x10,0xffffe
        assert_eq!(UType(0xfffff797).rd(), Register::A5); // auipc x15,0xfffff
        assert_eq!(UType(0xfffff797).rd(), Register::A5); // auipc x15,0xfffff

        assert_eq!(UType(0x00001a37).rd(), Register::S4); // lui x20,0x1
        assert_eq!(UType(0x800002b7).rd(), Register::T0); // lui x5,0x80000
        assert_eq!(UType(0x212120b7).rd(), Register::Ra); // lui x1,0x21212
        assert_eq!(UType(0xffffe517).rd(), Register::A0); // auipc x10,0xffffe
        assert_eq!(UType(0xfffff797).rd(), Register::A5); // auipc x15,0xfffff
        assert_eq!(UType(0xfffff797).rd(), Register::A5); // auipc x15,0xfffff
    }

    #[test]
    #[allow(overflowing_literals)]
    fn jtype() {
        assert_eq!(JType(0xfe1ff06f).imm(), 0x800029eci32 - 0x80002a0ci32); // jal x0,800029ec
        assert_eq!(JType(0x0000006f).imm(), 0x80002258 - 0x80002258); // jal x0,80002258
        assert_eq!(JType(0xf89ff06f).imm(), 0x800027aci32 - 0x80002824i32); // jal x0,800027ac
        assert_eq!(JType(0x0240006f).imm(), 0x8000215c - 0x80002138); // jal x0,8000215c
        assert_eq!(JType(0xd89ff0ef).imm(), 0x80002230i32 - 0x800024a8i32); // jal x1,80002230
        assert_eq!(JType(0x008007ef).imm(), 0x8000265c - 0x80002654); // jal x15,8000265c
        assert_eq!(JType(0x0240006f).imm(), 0x80002154 - 0x80002130); // jal x0,80002154
        assert_eq!(JType(0xf71ff06f).imm(), 0x80002750i32 - 0x800027e0i32); // jal x0,80002750
        assert_eq!(JType(0x00c0006f).imm(), 0x8000000c - 0x80000000); // jal x0,8000000c

        assert_eq!(JType(0xfe1ff06f).rd(), Register::Zero); // jal x0,800029ec
        assert_eq!(JType(0x0000006f).rd(), Register::Zero); // jal x0,80002258
        assert_eq!(JType(0xf89ff06f).rd(), Register::Zero); // jal x0,800027ac
        assert_eq!(JType(0x0240006f).rd(), Register::Zero); // jal x0,8000215c
        assert_eq!(JType(0xd89ff0ef).rd(), Register::Ra); // jal x1,80002230
        assert_eq!(JType(0x008007ef).rd(), Register::A5); // jal x15,8000265c
        assert_eq!(JType(0x0240006f).rd(), Register::Zero); // jal x0,80002154
        assert_eq!(JType(0xf71ff06f).rd(), Register::Zero); // jal x0,80002750
        assert_eq!(JType(0x00c0006f).rd(), Register::Zero); // jal x0,8000000c
    }

    #[test]
    fn fencetype() {
        assert_eq!(FenceType(0x0310000f).pred(), 0x3); // fence rw,w
        assert_eq!(FenceType(0x0820000f).pred(), 0x8); // fence i,r
        assert_eq!(FenceType(0x0ff0000f).pred(), 0xf); // fence iorw,iorw
        assert_eq!(FenceType(0x0140000f).pred(), 0x1); // fence w,o

        assert_eq!(FenceType(0x0310000f).succ(), 0x1); // fence rw,w
        assert_eq!(FenceType(0x0820000f).succ(), 0x2); // fence i,r
        assert_eq!(FenceType(0x0ff0000f).succ(), 0xf); // fence iorw,iorw
        assert_eq!(FenceType(0x0140000f).succ(), 0x4); // fence w,o
    }

    #[test]
    fn shifttype() {
        assert_eq!(ShiftType(0x0057979b).shamt(), 0x5); // slliw x15,x15,0x5
        assert_eq!(ShiftType(0x0057979b).shamt(), 0x5); // slliw x15,x15,0x5
        assert_eq!(ShiftType(0x00e09f1b).shamt(), 0xe); // slliw x30,x1,0xe
        assert_eq!(ShiftType(0x0017d61b).shamt(), 0x1); // srliw x12,x15,0x1
        assert_eq!(ShiftType(0x01f0df1b).shamt(), 0x1f); // srliw x30,x1,0x1f
        assert_eq!(ShiftType(0x0017d61b).shamt(), 0x1); // srliw x12,x15,0x1
        assert_eq!(ShiftType(0x41f0df1b).shamt(), 0x1f); // sraiw x30,x1,0x1f
        assert_eq!(ShiftType(0x4000df1b).shamt(), 0x0); // sraiw x30,x1,0x0
        assert_eq!(ShiftType(0x4070d09b).shamt(), 0x7); // sraiw x1,x1,0x7

        assert_eq!(ShiftType(0x0057979b).rs1(), 15); // slliw x15,x15,0x5
        assert_eq!(ShiftType(0x0057979b).rs1(), 15); // slliw x15,x15,0x5
        assert_eq!(ShiftType(0x00e09f1b).rs1(), 1); // slliw x30,x1,0xe
        assert_eq!(ShiftType(0x0017d61b).rs1(), 15); // srliw x12,x15,0x1
        assert_eq!(ShiftType(0x01f0df1b).rs1(), 1); // srliw x30,x1,0x1f
        assert_eq!(ShiftType(0x0017d61b).rs1(), 15); // srliw x12,x15,0x1
        assert_eq!(ShiftType(0x41f0df1b).rs1(), 1); // sraiw x30,x1,0x1f
        assert_eq!(ShiftType(0x4000df1b).rs1(), 1); // sraiw x30,x1,0x0
        assert_eq!(ShiftType(0x4070d09b).rs1(), 1); // sraiw x1,x1,0x7

        assert_eq!(ShiftType(0x0057979b).rd(), 15); // slliw x15,x15,0x5
        assert_eq!(ShiftType(0x0057979b).rd(), 15); // slliw x15,x15,0x5
        assert_eq!(ShiftType(0x00e09f1b).rd(), 30); // slliw x30,x1,0xe
        assert_eq!(ShiftType(0x0017d61b).rd(), 12); // srliw x12,x15,0x1
        assert_eq!(ShiftType(0x01f0df1b).rd(), 30); // srliw x30,x1,0x1f
        assert_eq!(ShiftType(0x0017d61b).rd(), 12); // srliw x12,x15,0x1
        assert_eq!(ShiftType(0x41f0df1b).rd(), 30); // sraiw x30,x1,0x1f
        assert_eq!(ShiftType(0x4000df1b).rd(), 30); // sraiw x30,x1,0x0
        assert_eq!(ShiftType(0x4070d09b).rd(), 1); // sraiw x1,x1,0x7
    }
}
