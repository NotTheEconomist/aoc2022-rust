use anyhow::Error;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Input {
    snacks: Vec<Vec<u64>>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let snacks = s
            .split("\n\n")
            .into_iter()
            .map(|elf| {
                elf.lines()
                    .map(|snack| snack.parse::<u64>())
                    .collect::<Result<Vec<u64>, _>>()
            })
            .collect::<Result<Vec<Vec<u64>>, _>>()?;
        Ok(Input { snacks })
    }
}

fn solve_part1(input: Input) -> Option<u64> {
    input
        .snacks
        .into_iter()
        .map(|snack_group| snack_group.into_iter().sum())
        .max()
}

fn solve_part2(input: Input) -> Option<u64> {
    let mut snacks: Vec<u64> = input
        .snacks
        .into_iter()
        .map(|snack_group| snack_group.into_iter().sum())
        .collect();
    snacks.sort();
    Some(snacks.into_iter().rev().take(3).sum())
}

fn main() {
    let input: Input = INPUT.parse().expect("Input must parse");
    let part1 = solve_part1(input.clone()).expect("input must have a solution to part1");
    println!("part1: {}", part1);
    let part2 = solve_part2(input).expect("input must have a solution to part2");
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("test_input.txt");
    #[test]
    fn solve_part1() {
        let input = INPUT.parse().expect("input must parse");
        assert_eq!(super::solve_part1(input), Some(24000));
    }
    #[test]
    fn solve_part2() {
        let input = INPUT.parse().expect("input must parse");
        assert_eq!(super::solve_part2(input), Some(45000));
    }
}
