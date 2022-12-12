use std::{collections::VecDeque, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
    IResult,
};

use crate::{Monkey, MonkeyMath, Test};

fn digit_parser<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse)(input)
}

fn items<T: FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, numbers) = separated_list1(tag(", "), digit_parser)(input)?;
    Ok((input, numbers))
}

type OpType<T> = Box<dyn Fn(&mut T)>;

fn multiply<T>(input: &str) -> IResult<&str, OpType<T>>
where
    T: FromStr,
    T: MonkeyMath,
{
    let (input, _) = tag("new = old * ")(input)?;
    let (input, constant) = digit_parser(input)?;
    Ok((input, Box::new(move |item| item.mul_constant(constant))))
}

fn add<T>(input: &str) -> IResult<&str, OpType<T>>
where
    T: FromStr,
    T: MonkeyMath,
{
    let (input, _) = tag("new = old + ")(input)?;
    let (input, constant) = digit_parser(input)?;
    Ok((input, Box::new(move |item| item.add_constant(constant))))
}

fn square<T>(input: &str) -> IResult<&str, OpType<T>>
where
    T: FromStr,
    T: MonkeyMath,
{
    let (input, _) = tag("new = old * old")(input)?;
    Ok((input, Box::new(move |item| item.square_self())))
}

fn op<T>(input: &str) -> IResult<&str, OpType<T>>
where
    T: FromStr,
    T: MonkeyMath,
{
    let (input, _) = tag("  Operation: ")(input)?;
    let (input, operation) = alt((multiply, add, square))(input)?;
    Ok((input, operation))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    let (input, divisor) = nom::character::complete::i64(input)?;
    let (input, true_recipient) = preceded(
        newline,
        preceded(
            tag("    If true: throw to monkey "),
            nom::character::complete::u32,
        ),
    )(input)?;
    let (input, false_recipient) = preceded(
        newline,
        preceded(
            tag("    If false: throw to monkey "),
            nom::character::complete::u32,
        ),
    )(input)?;
    let test = Test {
        divisor,
        true_recipient: true_recipient as usize,
        false_recipient: false_recipient as usize,
    };
    Ok((input, test))
}

fn monkey<T>(input: &str) -> IResult<&str, Monkey<T>>
where
    T: FromStr,
    T: MonkeyMath,
{
    let (input, _) = pair(tag("Monkey "), digit1)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;
    let (input, items): (&str, Vec<T>) = terminated(items, newline)(input)?;
    let (input, op) = terminated(op, newline)(input)?;
    let (input, test) = terminated(test, opt(newline))(input)?;
    let monkey = Monkey {
        items: VecDeque::from(items),
        op,
        test,
    };
    Ok((input, monkey))
}

pub fn parse<T>(input: &str) -> IResult<&str, Vec<Monkey<T>>>
where
    T: FromStr,
    T: MonkeyMath,
{
    separated_list1(newline, monkey)(input)
}
