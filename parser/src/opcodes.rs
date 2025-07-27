use std::ops::Add;

/// All Intel 8080 operations. Note: I am making data big endian.
#[derive(PartialEq, Debug)]
pub enum Op {
    /// Load register pair immediate
    Lxi(RegisterPair, u16),
    /// No op
    Nop,
    /// Move immediate ((r) <- u8) / Move to Memory Immediate ((HL) <- u8)
    Mvi(Register, u8),
}

/// The accessible registers.
#[derive(PartialEq, Debug)]
pub enum Register {
    /// Accumulator
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

/// 16 bit registers, handled as pairs of 8-bit registers (except for SP, the stack pointer.)
#[derive(PartialEq, Debug)]
pub enum RegisterPair {
    /// High-order register B, low-order register C
    BC,
    /// High-order register D, low-order register E
    DE,
    /// High-order register H, low-order register L
    HL,
    /// Stack pointer - a direct 16-bit register
    SP,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Address(pub u16);

#[derive(PartialEq, Debug)]
pub struct OpAndAddress {
    pub op: Op,
    pub address: Address,
    /// How many u8s were used to parse this instruction.
    /// Used to calculate the starting address of the next instruction.
    pub(crate) size: Size,
}

/// How many bytes have been used in parsing this instruction.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct Size(pub u16);

impl Add<Size> for Address {
    type Output = Address;

    fn add(self, rhs: Size) -> Self::Output {
        Address(self.0 + rhs.0)
    }
}
