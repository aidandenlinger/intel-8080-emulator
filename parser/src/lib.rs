//! A parsing library for Intel 8080 programs.
use core::result::Result as StdResult;
use nom::IResult;
use nom::Input;
use nom::Offset;
use nom::Parser;
use nom::combinator::consumed;
use nom::combinator::map;
use nom::error::ParseError;
use nom::number::complete::le_u16 as nom_le_u16;
use nom::number::complete::u8 as nom_u8;

pub mod error;
pub mod opcodes;

pub use crate::error::*;
pub use crate::opcodes::*;

/// Custom result returning this crate's error type
type PResult<I> = StdResult<I, crate::Error>;

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
) -> PResult<(&'input [u8], OpAndAddress)> {
    let (i, (op, size)) = parse_instruction(input)?;

    let address = if let Some(last) = last_instruction {
        last.address.checked_add(last.size)?
    } else {
        Address(0)
    };

    Ok((i, OpAndAddress { op, address, size }))
}

/// Any context required while parsing a single instruction.
#[derive(Default)]
struct Ctx {
    /// How many u8s we've consumed so far
    consumed_so_far: usize,
}

/// Wrapper function which takes a parser, runs it on input, and adds the number
/// of consumed bytes to Ctx.
fn update_ctx<'input, P, Error, Items>(
    parser: P,
    input: &'input [Items],
    ctx: &mut Ctx,
) -> IResult<&'input [Items], <P as Parser<&'input [Items]>>::Output, Error>
where
    &'input [Items]: Offset + Input,
    Error: ParseError<&'input [Items]>,
    P: Parser<&'input [Items], Error = Error>,
{
    let (i, (eaten, res)) = consumed(parser).parse(input)?;

    #[expect(clippy::expect_used, reason = "Difficult to pass a non-nom error here")]
    {
        ctx.consumed_so_far = ctx
            .consumed_so_far
            .checked_add(eaten.len())
            .expect("for bytes consumed to not overflow a usize");
    }

    Ok((i, res))
}

/// Parses a u8 and updates the Ctx.
fn u8<'input>(input: &'input [u8], ctx: &mut Ctx) -> IResult<&'input [u8], u8> {
    update_ctx(nom_u8, input, ctx)
}

/// Parses a u16 from two bytes (handling little-endian-ness) and updates the Ctx.
fn u16<'input>(input: &'input [u8], ctx: &mut Ctx) -> IResult<&'input [u8], u16> {
    update_ctx(nom_le_u16, input, ctx)
}

/// Parses an address from two bytes (handling little-endian-ness) and updates the Ctx.
#[expect(dead_code, reason = "it's only dead for now")]
fn addr<'input>(input: &'input [u8], ctx: &mut Ctx) -> IResult<&'input [u8], Address> {
    update_ctx(map(nom_le_u16, Address), input, ctx)
}

/// Parses a port from one byte and updates the Ctx.
#[expect(dead_code, reason = "it's only dead for now")]
fn port<'input>(input: &'input [u8], ctx: &mut Ctx) -> IResult<&'input [u8], Port> {
    update_ctx(map(nom_u8, Port), input, ctx)
}

/// Parses an instruction and how many parameters it used.
fn parse_instruction(input: &[u8]) -> PResult<(&[u8], (Op, Size))> {
    let mut ctx = Ctx::default();
    let (i, byte) = u8(input, &mut ctx)?;

    let (i, op) = match byte {
        0x00 => (i, Op::Nop),
        0x01 => {
            let (i, data) = u16(i, &mut ctx)?;
            (i, Op::Lxi(RegisterPair::BC, data))
        }
        0x02 => (i, Op::Stax(RegisterPairLdStAcc::BC)),
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
        _ => unimplemented!(),
    };

    let size = ctx
        .consumed_so_far
        .try_into()
        .map(Size)
        .map_err(|e| Error::Internal(format!("Bytes consumed larger than u16: {e}")))?;

    Ok((i, (op, size)))
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn incorrect_num_of_params() {
        let program = b"\x01\xAA"; // 0x01 needs two bytes, this only has one

        let expect_err = parse_8080_op(program, None);

        expect_err.unwrap_err();
    }

    #[test]
    fn different_sized_ops() -> PResult<()> {
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
    fn basic_no_op_program() -> PResult<()> {
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
