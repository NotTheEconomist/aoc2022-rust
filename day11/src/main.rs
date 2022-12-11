use std::str::FromStr;

use day11::{parser, Monkey, Monkeys};
use nom::Finish;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(input: Input) -> usize {
    let mut monkeys = Monkeys::new(input.0);
    for _ in 0..20 {
        monkeys.do_round()
    }
    let mut counts = monkeys.inspection_count;
    counts.sort();
    counts.into_iter().rev().take(2).product()
}

struct Input(Vec<Monkey>);
impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, monkeys) = nom::combinator::all_consuming(parser::parse)(s)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(Self(monkeys))
    }
}

fn main() {
    let input: Input = INPUT.parse().expect("Input must parse");
    let part1 = solve_part1(input);
    println!("part1: {part1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn solve_part1() {
        let input = INPUT.parse::<Input>().expect("input must parse");
        let part1 = super::solve_part1(input);
        assert_eq!(part1, 10605)
    }
}
