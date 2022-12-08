use crate::instructions::{DirName, Instruction, ListOutput};
use nom::{
    self,
    branch::alt,
    bytes::complete::{is_a, tag, take_until},
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(newline, command)(input)
}
fn command(input: &str) -> IResult<&str, Instruction> {
    preceded(tag("$ "), alt((cd, ls)))(input)
}

fn cd(input: &str) -> IResult<&str, Instruction> {
    dbg!("cd");
    let (input, _) = tag("cd ")(input)?;
    let (input, name) = take_until("\n")(input)?;
    let target = match name {
        "/" => DirName::Root,
        ".." => DirName::Parent,
        _ => DirName::Name(name.to_string()),
    };
    Ok((input, Instruction::ChangeDir(target)))
}

fn ls(input: &str) -> IResult<&str, Instruction> {
    dbg!("ls");
    let (input, _) = tag("ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, list_output) = ls_output(input)?;
    Ok((input, Instruction::List(list_output)))
}

fn ls_output(input: &str) -> IResult<&str, Vec<ListOutput>> {
    separated_list1(newline, ls_output_line)(input)
}

fn filename(input: &str) -> IResult<&str, &str> {
    is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.")(input)
}

fn dir(input: &str) -> IResult<&str, ListOutput> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = filename(input)?;
    Ok((input, ListOutput::Dir(name.to_string())))
}

fn file(input: &str) -> IResult<&str, ListOutput> {
    let (input, size) = map_res(digit1, |size: &str| size.parse::<u64>())(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = filename(input)?;
    Ok((input, ListOutput::File(size, name.to_string())))
}

fn ls_output_line(input: &str) -> IResult<&str, ListOutput> {
    alt((dir, file))(input)
}
