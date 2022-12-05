use crate::{Crate, Instruction, MoveKind};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, char as single_char, digit1, newline},
    combinator::{all_consuming, map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, terminated},
    IResult,
};

#[derive(Clone)]
enum InternalCrate {
    Missing,
    Crate(char),
}

#[allow(clippy::from_over_into)]
impl Into<Option<Crate>> for InternalCrate {
    fn into(self) -> Option<Crate> {
        match self {
            Self::Missing => None,
            Self::Crate(c) => Some(Crate(c)),
        }
    }
}

fn parse_crate(input: &str) -> IResult<&str, Option<Crate>> {
    map(
        alt((
            value(InternalCrate::Missing, tag("   ")),
            map(
                delimited(single_char('['), anychar, single_char(']')),
                InternalCrate::Crate,
            ),
        )),
        |ic| ic.into(),
    )(input)
}

fn crates(input: &str) -> IResult<&str, Vec<(usize, Crate)>> {
    map(separated_list1(tag(" "), parse_crate), |v| {
        (1usize..)
            .zip(v.into_iter())
            .filter_map(|(idx, c)| c.map(|c| (idx, c)))
            .collect()
    })(input)
}

fn instruction_of_kind(kind: MoveKind) -> impl Fn(&str) -> IResult<&str, Instruction> {
    move |input: &str| {
        map(
            pair(
                delimited(tag("move "), map_res(digit1, str::parse), tag(" from ")),
                separated_pair(
                    map_res(digit1, str::parse::<usize>),
                    tag(" to "),
                    map_res(digit1, str::parse::<usize>),
                ),
            ),
            |(count, (src, dst))| Instruction {
                count,
                src: src - 1,
                dst: dst - 1,
                kind,
            },
        )(input)
    }
}

type ParseOutput = (Vec<(usize, Crate)>, Vec<Instruction>);

pub fn parse_of_move_kind(move_kind: MoveKind) -> impl Fn(&str) -> IResult<&str, ParseOutput> {
    move |input: &str| {
        let (input, crates) = map(terminated(separated_list1(newline, crates), newline), |v| {
            v.into_iter().flatten().collect()
        })(input)?;
        let (input, _) = terminated(take_until("\n"), newline)(input)?;
        let (input, _) = terminated(take_until("\n"), newline)(input)?;
        let (input, directions) =
            all_consuming(separated_list1(newline, instruction_of_kind(move_kind)))(input)?;
        Ok((input, (crates, directions)))
    }
}

/// Legacy from part1. Prefer parse_of_move_kind(MoveKind::Individual)
pub fn parse(input: &str) -> IResult<&str, ParseOutput> {
    parse_of_move_kind(MoveKind::Individual)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! c {
        ($char:literal) => {
            Crate($char)
        };
    }
    macro_rules! i {
        ($count:literal, $src:literal, $dst:literal, $kind:ident) => {
            Instruction {
                count: $count,
                src: $src - 1,
                dst: $dst - 1,
                kind: MoveKind::$kind,
            }
        };
    }
    #[test]
    fn whole_input() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let (_, got) = parse(input).expect("input must parse");
        let expected = (
            vec![
                (2, c!('D')),
                (1, c!('N')),
                (2, c!('C')),
                (1, c!('Z')),
                (2, c!('M')),
                (3, c!('P')),
            ],
            vec![
                i!(1, 2, 1, Individual),
                i!(3, 1, 3, Individual),
                i!(2, 2, 1, Individual),
                i!(1, 1, 2, Individual),
            ],
        );
        assert_eq!(got, expected);
    }
}
