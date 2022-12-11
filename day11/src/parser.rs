use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
    IResult,
};

use crate::Monkey;

fn digit_parser(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse)(input)
}

fn items(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    let (input, numbers) = separated_list1(tag(", "), digit_parser)(input)?;
    Ok((input, numbers))
}

fn multiply(input: &str) -> IResult<&str, Box<dyn Fn(i64) -> i64>> {
    let (input, _) = tag("new = old * ")(input)?;
    let (input, constant) = digit_parser(input)?;
    Ok((input, Box::new(move |item| item * constant)))
}

fn add(input: &str) -> IResult<&str, Box<dyn Fn(i64) -> i64>> {
    let (input, _) = tag("new = old + ")(input)?;
    let (input, constant) = digit_parser(input)?;
    Ok((input, Box::new(move |item| item + constant)))
}

fn square(input: &str) -> IResult<&str, Box<dyn Fn(i64) -> i64>> {
    let (input, _) = tag("new = old * old")(input)?;
    Ok((input, Box::new(move |item| item * item)))
}

fn op(input: &str) -> IResult<&str, Box<dyn Fn(i64) -> i64>> {
    let (input, _) = tag("  Operation: ")(input)?;
    let (input, operation) = alt((multiply, add, square))(input)?;
    Ok((input, operation))
}

fn test(input: &str) -> IResult<&str, Box<dyn Fn(i64) -> usize>> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    let (input, divisor) = digit_parser(input)?;
    let (input, true_usize) = preceded(
        newline,
        preceded(tag("    If true: throw to monkey "), digit_parser),
    )(input)?;
    let (input, false_usize) = preceded(
        newline,
        preceded(tag("    If false: throw to monkey "), digit_parser),
    )(input)?;
    let test = Box::new(move |item| {
        if item % divisor == 0 {
            true_usize as usize
        } else {
            false_usize as usize
        }
    });
    Ok((input, test))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = pair(tag("Monkey "), digit1)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;
    let (input, items) = terminated(items, newline)(input)?;
    let (input, op) = terminated(op, newline)(input)?;
    let (input, test) = terminated(test, opt(newline))(input)?;
    let monkey = Monkey {
        items: VecDeque::from(items),
        op,
        test,
    };
    Ok((input, monkey))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, monkey)(input)
}
