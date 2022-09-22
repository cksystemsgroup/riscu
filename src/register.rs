use core::fmt;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
#[repr(u32)]
pub enum Register {
    Zero = 0,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    Fp,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const LUT: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "fp", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];

        let idx: u32 = (*self).into();

        write!(f, "{}", LUT[idx as usize])
    }
}

impl From<u32> for Register {
    fn from(raw: u32) -> Register {
        unsafe { core::mem::transmute(raw) }
    }
}

impl From<Register> for u32 {
    fn from(reg: Register) -> u32 {
        unsafe { core::mem::transmute(reg) }
    }
}
