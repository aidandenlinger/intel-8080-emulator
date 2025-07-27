use nom::IResult;
use nom::bytes::complete::take as nom_take;

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

/// A wrapper around nom's name that also updates Ctx with the amount consumed.
/// Used to enforce
fn take<'input>(n: u16, input: &'input [u8], ctx: &mut Ctx) -> IResult<&'input [u8], &'input [u8]> {
    let res = nom_take(n)(input)?;
    ctx.consumed_so_far += n;

    Ok(res)
}

/// Parses an instruction and how many parameters it used.
fn parse_instruction(input: &[u8]) -> IResult<&[u8], (Op, Size)> {
    let mut ctx = Ctx::default();
    let (i, byte) = take(1, input, &mut ctx)?;

    let (i, op) = match byte[0] {
        0x00 => (i, Op::Nop),
        _ => todo!(),
    };

    Ok((i, (op, Size(ctx.consumed_so_far))))
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

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
