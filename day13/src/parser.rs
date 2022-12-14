use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};

use crate::{Packet, Pair};

fn packet_num<T>(input: &str) -> IResult<&str, Packet<T>>
where
    T: FromStr + Clone + PartialOrd,
{
    map_res(digit1, |n: &str| n.parse::<T>().map(Packet::Num))(input)
}
fn packet_list<T>(input: &str) -> IResult<&str, Packet<T>>
where
    T: FromStr + Clone + PartialOrd,
{
    let (input, packets) = delimited(tag("["), separated_list0(tag(","), packet), tag("]"))(input)?;
    let packets = packets.into_iter().map(Box::new).collect::<Vec<_>>();
    Ok((input, Packet::List(packets)))
}
pub fn packet<T>(input: &str) -> IResult<&str, Packet<T>>
where
    T: FromStr + Clone + PartialOrd,
{
    alt((packet_num, packet_list))(input)
}
pub(crate) fn packet_pair<T>(input: &str) -> IResult<&str, Pair<T>>
where
    T: FromStr + Clone + PartialOrd,
{
    let (input, packet_a) = terminated(packet, newline)(input)?;
    let (input, packet_b) = packet(input)?;
    Ok((input, (packet_a, packet_b)))
}

pub fn packet_pairs<T>(input: &str) -> IResult<&str, Vec<Pair<T>>>
where
    T: FromStr + Clone + PartialOrd,
{
    separated_list1(tag("\n\n"), packet_pair)(input)
}
