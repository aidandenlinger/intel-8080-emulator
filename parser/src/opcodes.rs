use subenum::subenum;

use crate::{Error, PResult};

/// All Intel 8080 operations
#[derive(PartialEq, Debug)]
pub enum Op {
    // Data transfer group
    /// Move from arg 2 to arg 1. NOTE - memory to memory is invalid
    Mov(RegisterOrMemory, RegisterOrMemory),
    /// Move Immediate / Move to Memory Immediate
    Mvi(RegisterOrMemory, u8),
    /// Load register pair immediate
    Lxi(RegisterPair, u16),
    /// Load accumulator direct
    Lda(Address),
    /// Store accumulator direct
    Sta(Address),
    /// Load HL direct
    Lhld(Address),
    /// Store HL direct
    Shld(Address),
    /// Load accumulator indirect
    Ldax(RegisterPairLdStAcc),
    /// Store accumulator indirect
    Stax(RegisterPairLdStAcc),
    /// Exchange HL with DE
    Xchg,

    // Arithmetic Group
    /// Add register/memory (to accumulator)
    Add(RegisterOrMemory),
    /// Add immediate (to accumulator)
    Adi(u8),
    /// Add register or memory with carry (to accumulator)
    Adc(RegisterOrMemory),
    /// Add immediate with carry (to accumulator)
    Aci(u8),
    /// Subtract register/memory (to accumulator)
    Sub(RegisterOrMemory),
    /// Subtract immediate (to accumulator)
    Sui(u8),
    /// Subtract register/memory with borrow (to accumulator)
    Sbb(RegisterOrMemory),
    /// Subtract immediate with borrow (to accumulator)
    Sbi(u8),
    /// Increment register/memory
    Inr(RegisterOrMemory),
    /// Decrement register/memory
    Dcr(RegisterOrMemory),
    /// Increment register pair
    Inx(RegisterPair),
    /// Decrement register pair
    Dcx(RegisterPair),
    /// Add register pair to H and L
    Dad(RegisterPair),
    /// Decimal Adjust Accumulator
    Daa,

    // Logical group
    /// AND register/memory and accumulator (to accumulator)
    Ana(RegisterOrMemory),
    /// AND immediate (to accumulator)
    Ani(u8),
    /// Exclusive OR register (to accumulator)
    Xra(RegisterOrMemory),
    /// Exclusive OR immediate (to accumulator)
    Xri(u8),
    /// OR register/memory (to accumulator)
    Ora(RegisterOrMemory),
    /// OR Immediate (to accumulator)
    Ori(u8),
    /// Compare register/memory to accumulator (Z=1 if equal, CY=1 if A<register/memory)
    Cmp(RegisterOrMemory),
    /// Compare immediate to accumulator (Z=1 if equal, CY=1 if A<data)
    Cpi(u8),
    /// Rotate left (accumulator)
    Rcl,
    /// Rotate right (accumulator)
    Rrc,
    /// Rotate left through carry (accumulator)
    Ral,
    /// Rotate right through carry (accumulator)
    Rar,
    /// Complement accumulator
    Cma,
    /// Complement carry
    Cmc,
    /// Set carry
    Stc,

    // Branch Group
    /// Jump
    Jmp(Address),
    /// Jump not zero
    Jnz(Address),
    /// Jump zero
    Jz(Address),
    /// Jump no carry
    Jnc(Address),
    /// Jump carry
    Jc(Address),
    /// Jump parity odd
    Jpo(Address),
    /// Jump parity even
    Jpe(Address),
    /// Jump plus
    Jp(Address),
    /// Jump minus
    Jm(Address),

    /// Call
    Call(Address),
    /// Call not zero
    Cnz(Address),
    /// Call zero
    Cz(Address),
    /// Call no carry
    Cnc(Address),
    /// Call carry
    Cc(Address),
    /// Call parity odd
    Cpo(Address),
    /// Call parity even
    Cpe(Address),
    /// Call plus
    Cp(Address),
    /// Call minus
    Cm(Address),

    /// Return
    Ret,
    /// Return not zero
    Rnz,
    /// Return zero
    Rz,
    /// Return no carry
    Rnc,
    /// Return carry
    Rc,
    /// Return parity odd
    Rpo,
    /// Return parity even
    Rpe,
    /// Return plus
    Rp,
    /// Return minus
    Rm,

    /// Restart
    Rst(RestartNum),
    /// Jump HL indirect - move HL to PC
    Pchl,

    // Stack, I/O, and Machine Control Group
    /// Push
    Push(RegisterPairNonSp),
    /// Push processor status word
    PushPsw,
    /// Pop
    Pop(RegisterPairNonSp),
    /// Pop processor status word
    PopPsw,
    /// Exchange stack top with HL
    Xthl,
    /// Move HL to SP
    Sphl,
    /// In
    In(Port),
    /// Out
    Out(Port),
    /// Enable Interrupts
    Ei,
    /// Disable interrupts
    Di,
    /// Halt
    Hlt,
    /// No op
    Nop,

    /// Undefined - this isn't a valid opcode
    Undefined,
}

#[derive(PartialEq, Debug)]
pub struct RestartNum(u8);

impl RestartNum {
    /// Generates a `RestartNum` from a u8.
    ///
    /// # Errors
    /// If input is not within 0..8
    pub fn new(data: u8) -> PResult<Self> {
        let accepted_range = 0..8;

        if !accepted_range.contains(&data) {
            return Err(Error::Internal(format!(
                "{data} not in {accepted_range:#?}"
            )));
        }

        Ok(Self(data))
    }
}

/// Locations to store data in
#[derive(PartialEq, Debug)]
pub enum RegisterOrMemory {
    Register(Register),
    /// The memory address held by register pair HL
    Memory,
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

#[subenum(RegisterPairLdStAcc, RegisterPairNonSp)]
#[derive(PartialEq, Debug)]
/// 16 bit registers, handled as pairs of 8-bit registers (except for SP, the stack pointer.)
pub enum RegisterPair {
    /// High-order register B, low-order register C
    #[subenum(RegisterPairLdStAcc, RegisterPairNonSp)]
    BC,
    /// High-order register D, low-order register E
    #[subenum(RegisterPairLdStAcc, RegisterPairNonSp)]
    DE,
    /// High-order register H, low-order register L
    #[subenum(RegisterPairNonSp)]
    HL,
    /// Stack pointer - a direct 16-bit register
    SP,
}

/// 8-bit address of an I/O device
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Port(pub u8);

/// 16-bit address quanitity
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
pub struct Size(pub u16);

impl Address {
    /// Adds a size to an address.
    ///
    /// # Errors
    /// If the result overflows the 16-bit address space.
    pub fn checked_add(self, rhs: Size) -> PResult<Self> {
        self.0
            .checked_add(rhs.0)
            .map(Self)
            .ok_or_else(|| Error::Internal(format!("Overflow adding {self:?} to {rhs:?}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_restart_nums() -> PResult<()> {
        let num = RestartNum::new(0)?;
        assert_eq!(num.0, 0);

        let num = RestartNum::new(7)?;
        assert_eq!(num.0, 7);

        let err = RestartNum::new(8);
        err.unwrap_err();

        Ok(())
    }
}
