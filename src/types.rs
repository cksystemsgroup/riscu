// Copyright 2019 Jonathan Behrens
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This module was modified by the Selfie authors.

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

        Self((((((((((funct7 << 5) + rs2) << 5) + rs1) << 3) + funct3) << 5) + rd) << 7) + opcode)
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
pub struct IType(pub u32);
impl IType {
    pub(crate) fn new(
        immediate: i32,
        funct3: u32,
        opcode: u32,
        rd: Register,
        rs1: Register,
    ) -> Self {
        assert!(-(2_i32.pow(11)) <= immediate && immediate < 2_i32.pow(11));
        assert!(funct3 < 2_u32.pow(3));
        assert!(opcode < 2_u32.pow(7));

        let rd: u32 = rd.into();
        let rs1: u32 = rs1.into();

        let immediate = sign_shrink(immediate, 12);

        Self((((((((immediate << 5) + rs1) << 3) + funct3) << 5) + rd) << 7) + opcode)
    }
    pub fn imm(&self) -> i32 {
        sign_extend(self.0 >> 20, 12)
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
        assert!(-(2_i32.pow(11)) <= immediate && immediate < 2_i32.pow(11));
        assert!(funct3 < 2_u32.pow(3));
        assert!(opcode < 2_u32.pow(7));

        let rs1: u32 = rs1.into();
        let rs2: u32 = rs2.into();

        let immediate = sign_shrink(immediate, 12);

        let imm1 = get_bits(immediate, 5, 7);
        let imm2 = get_bits(immediate, 0, 5);

        Self((((((((((imm1 << 5) + rs2) << 5) + rs1) << 3) + funct3) << 5) + imm2) << 7) + opcode)
    }
    pub fn imm(&self) -> i32 {
        sign_extend(((self.0 >> 20) & 0xfe0) | ((self.0 >> 7) & 0x1f), 12)
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
        assert!(-(2_i32.pow(12)) <= immediate && immediate < 2_i32.pow(12));
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
            (((((((((((((imm1 << 6) + imm2) << 5) + rs2) << 5) + rs1) << 3) + funct3) << 4)
                + imm3)
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
        )
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

        Self((((immediate << 5) + rd) << 7) + opcode)
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
        assert!(-(2_i32.pow(20)) <= immediate && immediate < 2_i32.pow(20));
        assert!(opcode < 2_u32.pow(7));

        let rd: u32 = rd.into();

        let immediate = sign_shrink(immediate, 21);

        let imm1 = get_bits(immediate, 20, 1);
        let imm2 = get_bits(immediate, 1, 10);
        let imm3 = get_bits(immediate, 11, 1);
        let imm4 = get_bits(immediate, 12, 8);

        Self((((((((((imm1 << 10) + imm2) << 1) + imm3) << 8) + imm4) << 5) + rd) << 7) + opcode)
    }
    pub fn imm(&self) -> i32 {
        sign_extend(
            ((self.0 & 0x8000_0000) >> 11)
                | ((self.0 & 0x7fe0_0000) >> 20)
                | ((self.0 & 0x0010_0000) >> 9)
                | (self.0 & 0x000f_f000),
            21,
        )
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

fn get_bits(n: u32, i: u32, b: u32) -> u32 {
    assert!(0 < b && b <= i + b && i + b < 32);

    if i == 0 {
        n % 2_u32.pow(b)
    } else {
        // shift to-be-loaded bits all the way to the left
        // to reset all bits to the left of them, then
        // shift to-be-loaded bits all the way to the right and return
        (n << (32 - (i + b))) >> (32 - b)
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

        assert_eq!(RType(0x00c58633).rs2(), Register::A2); // add x12,x11,x12
        assert_eq!(RType(0x40b50533).rs2(), Register::A1); // sub x10,x10,x11

        assert_eq!(RType(0x00c58633).rd(), Register::A2); // add x12,x11,x12
        assert_eq!(RType(0x40b50533).rd(), Register::A0); // sub x10,x10,x11
    }

    #[test]
    fn itype() {
        assert_eq!(IType(0x01853683).rd(), Register::A3); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).rd(), Register::S8); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).rd(), Register::A4); // Ld x14,0(x15)

        assert_eq!(IType(0x01853683).rs1(), Register::A0); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).rs1(), Register::Sp); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).rs1(), Register::A5); // Ld x14,0(x15)

        assert_eq!(IType(0x01853683).imm(), 24); // Ld x13,24(x10)
        assert_eq!(IType(0x02013c03).imm(), 32); // Ld x24,32(x2)
        assert_eq!(IType(0x0007b703).imm(), 0); // Ld x14,0(x15)
    }

    #[test]
    #[allow(overflowing_literals)]
    fn btype() {
        assert_eq!(BType(0x06f58063).imm(), 0x80002724 - 0x800026c4); // beq x11,x15,80002724
        assert_eq!(BType(0x06f58063).imm(), 0x80002648 - 0x800025e8); // beq x11,x15,80002648
        assert_eq!(BType(0x00050a63).imm(), 0x800024e8 - 0x800024d4); // beq x10,x0,800024e8
        assert_eq!(BType(0x03ff0663).imm(), 0x80000040 - 0x80000014); // beq x30,x31,80000040
    }

    #[test]
    fn utype() {
        assert_eq!(UType(0x00001a37).rd(), Register::S4); // lui x20,0x1
        assert_eq!(UType(0x800002b7).rd(), Register::T0); // lui x5,0x80000
        assert_eq!(UType(0x212120b7).rd(), Register::Ra); // lui x1,0x21212

        assert_eq!(UType(0x00001a37).rd(), Register::S4); // lui x20,0x1
        assert_eq!(UType(0x800002b7).rd(), Register::T0); // lui x5,0x80000
        assert_eq!(UType(0x212120b7).rd(), Register::Ra); // lui x1,0x21212
    }

    #[test]
    #[allow(overflowing_literals)]
    fn jtype() {
        assert_eq!(JType(0xfe1ff06f).imm(), 0x800029eci32 - 0x80002a0ci32); // jal x0,800029ec
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
}
