use nom::IResult;
use nom::number::complete::le_u16 as nom_le_u16;
use nom::number::complete::u8 as nom_u8;

pub mod opcodes;

pub use crate::opcodes::*;

/// Parses an Intel 8080 instruction and stores its address.
/// If this is the first instruction, pass `None` for last instruction.
///
/// # Errors
/// If parsing fails, for example, if a instruction does not have the proper
/// amount of parameters.
///
/// # Examples
///
/// ```
/// use parser::{Address, Op, parse_8080_op};
///
/// let program = b"\x00\x00"; // Two nops
/// let (rest, instruction) = parse_8080_op(program, None).unwrap();
/// assert_eq!(Op::Nop, instruction.op);
/// assert_eq!(Address(0x0), instruction.address);
///
/// let (rest, instruction) = parse_8080_op(rest, Some(&instruction)).unwrap();
/// assert_eq!(Op::Nop, instruction.op);
/// assert_eq!(Address(0x1), instruction.address);
/// ```
pub fn parse_8080_op<'input>(
    input: &'input [u8],
    last_instruction: Option<&OpAndAddress>,
) -> IResult<&'input [u8], OpAndAddress> {
    let (i, (op, size)) = parse_instruction(input)?;

    let address = if let Some(last) = last_instruction {
        last.address + last.size
    } else {
        Address(0)
    };

    Ok((i, OpAndAddress { op, address, size }))
}

/// Any context required while parsing a single instruction.
#[derive(Default)]
struct Ctx {
    /// How many u8s we've consumed so far. Managed by wrappers around nom functions.
    consumed_so_far: u16,
}

/// A wrapper around nom's take that updates Ctx with the amount consumed.
fn u8<'input>(input: &'input [u8], ctx: &mut Ctx) -> IResult<&'input [u8], u8> {
    let res = nom_u8(input)?;
    ctx.consumed_so_far += 1;

    Ok(res)
}

/// Parses a u16 from two bytes (handling little-endian-ness) and updates the Ctx.
fn u16<'input>(input: &'input [u8], ctx: &mut Ctx) -> IResult<&'input [u8], u16> {
    let res = nom_le_u16(input)?;
    ctx.consumed_so_far += 2; // consumes two u8s

    Ok(res)
}

/// Parses an address from two bytes (handling little-endian-ness) and updates the Ctx.
fn addr<'input>(input: &'input [u8], ctx: &mut Ctx) -> IResult<&'input [u8], Address> {
    let (i, data) = u16(input, ctx)?;
    Ok((i, Address(data)))
}

/// Parses an instruction and how many parameters it used.
fn parse_instruction(input: &[u8]) -> IResult<&[u8], (Op, Size)> {
    let mut ctx = Ctx::default();
    let (i, byte) = u8(input, &mut ctx)?;

    let (i, op) = match byte {
        0x00 => (i, Op::Nop),
        0x01 => {
            let (i, data) = u16(i, &mut ctx)?;
            (i, Op::Lxi(RegisterPair::BC, data))
        }
        0x02 => (i, Op::Stax(RegisterPair::BC)),
        0x03 => (i, Op::Inx(RegisterPair::BC)),
        0x04 => (i, Op::Inr(RegisterOrMemory::Register(Register::B))),
        0x05 => (i, Op::Dcr(RegisterOrMemory::Register(Register::B))),
        0x06 => {
            let (i, data) = u8(i, &mut ctx)?;
            (i, Op::Mvi(RegisterOrMemory::Register(Register::B), data))
        }
        0x07 => (i, Op::Rcl),
        0x08 => (i, Op::Undefined),
        0x09 => (i, Op::Dad(RegisterPair::BC)),
        0x0a => todo!(),
        _ => todo!(),
    };

    Ok((i, (op, Size(ctx.consumed_so_far))))
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, error::Error};

    use super::*;

    #[test]
    fn test_different_sized_ops() -> Result<(), Box<dyn Error>> {
        let program = b"\x00\x06\xAA\x01\x22\x33";

        let mut expected = VecDeque::from([
            OpAndAddress {
                op: Op::Nop,
                address: Address(0x0),
                size: Size(1),
            },
            OpAndAddress {
                op: Op::Mvi(RegisterOrMemory::Register(Register::B), 0xAA),
                address: Address(0x1),
                size: Size(2),
            },
            OpAndAddress {
                op: Op::Lxi(RegisterPair::BC, 0x3322),
                address: Address(0x3),
                size: Size(3),
            },
        ]);

        let (rest, instruction) = parse_8080_op(program, None)?;
        assert_eq!(expected.pop_front().unwrap(), instruction);

        let (rest, instruction) = parse_8080_op(rest, Some(&instruction))?;
        assert_eq!(expected.pop_front().unwrap(), instruction);

        let (_, instruction) = parse_8080_op(rest, Some(&instruction))?;
        assert_eq!(expected.pop_front().unwrap(), instruction);

        Ok(())
    }

    #[test]
    fn basic_no_op_program() -> Result<(), Box<dyn Error>> {
        let program = b"\x00\x00\x00"; // Three nops
        let (rest, instruction) = parse_8080_op(program, None)?;
        assert_eq!(
            OpAndAddress {
                op: Op::Nop,
                address: Address(0x0),
                size: Size(1)
            },
            instruction
        );

        let (rest, instruction) = parse_8080_op(rest, Some(&instruction))?;
        assert_eq!(
            OpAndAddress {
                op: Op::Nop,
                address: Address(0x1),
                size: Size(1)
            },
            instruction
        );

        let (_, instruction) = parse_8080_op(rest, Some(&instruction))?;
        assert_eq!(
            OpAndAddress {
                op: Op::Nop,
                address: Address(0x2),
                size: Size(1)
            },
            instruction
        );

        Ok(())
    }
}
