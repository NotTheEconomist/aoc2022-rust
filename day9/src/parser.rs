use nom::{
    self,
    bytes::complete::tag,
    character::complete::{digit1, newline, one_of},
    combinator::{all_consuming, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::Vector;

fn line(input: &str) -> IResult<&str, Vector> {
    let (input, (direction, magnitude)) = separated_pair(
        one_of("UDLR"),
        tag(" "),
        map_res(digit1, |s: &str| s.parse::<i32>()),
    )(input)?;
    let vector = match direction {
        'U' => Vector { x: 0, y: magnitude },
        'D' => Vector {
            x: 0,
            y: -magnitude,
        },
        'R' => Vector { x: magnitude, y: 0 },
        'L' => Vector {
            x: -magnitude,
            y: 0,
        },
        _ => unreachable!(),
    };
    Ok((input, vector))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Vector>> {
    all_consuming(separated_list1(newline, line))(input)
}
