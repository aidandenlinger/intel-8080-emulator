use std::ops::Add;

#[derive(PartialEq, Debug)]
pub enum Op {
    Nop,
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
