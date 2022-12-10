use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, newline},
    combinator::{all_consuming, map_res, opt},
    multi::separated_list1,
    IResult,
};

use crate::Instruction;

fn instruction_addx(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("addx ")(input)?;
    let (input, is_negative) = opt(complete::char('-'))(input)?;
    let (input, n) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;

    let result = match is_negative {
        Some(_) => Instruction::AddX(-n),
        None => Instruction::AddX(n),
    };

    Ok((input, result))
}

fn instruction_noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;
    Ok((input, Instruction::NoOp))
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((instruction_addx, instruction_noop))(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    all_consuming(separated_list1(newline, instruction))(input)
}
