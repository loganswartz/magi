use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone)]
pub struct SM83RegisterBank {
    // 8 bit
    // general purpose
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub flags: FlagRegister,

    // 16 bit
    // program counter
    pub pc: u16,
    // stack pointer
    pub sp: u16,

    // instruction clock
    pub m: u8,
    pub t: u8,
}

impl SM83RegisterBank {
    /// Helper function to get the value of a 16-bit register.
    pub fn combined(&self, first: u8, second: u8) -> u16 {
        u16::from_le_bytes([first, second])
    }

    pub fn split(&mut self, value: u16) -> [u8; 2] {
        value.to_le_bytes()
    }

    pub fn hl(&self) -> u16 {
        self.combined(self.h, self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        let [first, second] = self.split(value);

        self.h = first;
        self.l = second;
    }
}

#[derive(Debug, Clone)]
pub struct FlagRegister {
    value: u8,
}

pub enum Flag {
    Zero,
    Subtract,
    HalfCarry,
    Carry,
}

impl Flag {
    pub fn value(&self) -> u8 {
        match self {
            Flag::Zero => 0b1000_0000,
            Flag::Subtract => 0b0100_0000,
            Flag::HalfCarry => 0b0010_0000,
            Flag::Carry => 0b0001_0000,
        }
    }
}

impl BitOr<u8> for Flag {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        self.value() | rhs
    }
}

impl BitOr<Flag> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Flag) -> Self::Output {
        self | rhs.value()
    }
}

impl BitAnd<u8> for Flag {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.value() & rhs
    }
}

impl BitAnd<Flag> for u8 {
    type Output = u8;

    fn bitand(self, rhs: Flag) -> Self::Output {
        self & rhs.value()
    }
}

impl BitXor<u8> for Flag {
    type Output = u8;

    fn bitxor(self, rhs: u8) -> Self::Output {
        self.value() ^ rhs
    }
}

impl BitXor<Flag> for u8 {
    type Output = u8;

    fn bitxor(self, rhs: Flag) -> Self::Output {
        self ^ rhs.value()
    }
}

impl Not for Flag {
    type Output = u8;

    fn not(self) -> Self::Output {
        !self.value()
    }
}

impl BitOrAssign<Flag> for u8 {
    fn bitor_assign(&mut self, rhs: Flag) {
        *self |= rhs.value();
    }
}

impl BitAndAssign<Flag> for u8 {
    fn bitand_assign(&mut self, rhs: Flag) {
        *self &= rhs.value();
    }
}

impl BitXorAssign<Flag> for u8 {
    fn bitxor_assign(&mut self, rhs: Flag) {
        *self ^= rhs.value();
    }
}

impl PartialEq<Flag> for u8 {
    fn eq(&self, rhs: &Flag) -> bool {
        *self == rhs.value()
    }
}

impl FlagRegister {
    pub fn new() -> Self {
        FlagRegister { value: 0b0000_0000 }
    }

    /// Check if a given bit is set.
    pub fn check(&mut self, flag: Flag) -> bool {
        (flag.value() & self.value) == flag
    }

    /// Set a given bit.
    pub fn set(&mut self, flag: Flag) {
        self.value |= flag;
    }

    /// Clear a given bit.
    pub fn unset(&mut self, flag: Flag) {
        self.value &= !flag;
    }

    /// Clear all flags.
    pub fn clear(&mut self) {
        self.value = 0b0000_0000;
    }
}

impl SM83RegisterBank {
    pub fn new() -> Self {
        SM83RegisterBank {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            flags: FlagRegister::new(),
            pc: 0,
            sp: 0,
            m: 0,
            t: 0,
        }
    }
}
