use std::str::FromStr;

use anyhow::{Context, Result};
use day5::{Crate, Instruction, Yard};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Input = INPUT.parse().expect("Input must parse");
    let part1 = solve_part1(input.clone()).expect("part1 must have a solution");
    println!("part1: {part1}");
    let part2 = solve_part2(input).expect("part2 must have a solution");
    println!("part2: {part2}");
}

fn solve_part1(input: Input) -> Option<String> {
    let mut yard: Yard = input.shipping_yard.into();
    let instructions = input.instructions;

    for instruction in instructions {
        yard.act(instruction).ok()?
    }
    yard.topmost()
}

fn solve_part2(input: Input) -> Option<String> {
    let mut yard: Yard = input.shipping_yard.into();
    let instructions = input.instructions.into_iter().map(Instruction::to_grouped);

    for instruction in instructions {
        yard.act(instruction).ok()?
    }
    yard.topmost()
}

#[derive(Debug, Clone)]
struct Input {
    shipping_yard: Vec<(usize, Crate)>,
    instructions: Vec<Instruction>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        day5::parser::parse(s)
            .map(|(_, (shipping_yard, instructions))| Self {
                shipping_yard,
                instructions,
            })
            .map_err(|e| e.to_owned())
            .context("failed to parse")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn solve_part1() {
        let input: Input = INPUT.parse().expect("input must parse");
        let part1 = super::solve_part1(input).expect("part1 must have a solution");
        assert_eq!(part1, String::from("CMZ"));
    }

    #[test]
    fn solve_part2() {
        let input: Input = INPUT.parse().expect("input must parse");
        let part2 = super::solve_part2(input).expect("part2 must have a solution");
        assert_eq!(part2, String::from("MCD"));
    }
}
