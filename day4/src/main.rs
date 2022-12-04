use std::{convert::Infallible, str::FromStr};

use day4::{HasOverlap, HasSuperset, Pair, Range};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Input(Vec<String>);
impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(String::from).collect()))
    }
}
impl Input {
    fn into_pairs(self) -> Vec<Pair> {
        self.0
            .into_iter()
            .flat_map(|line| {
                let (a, b) = line.split_once(',')?;
                let (a, b) = (a.parse::<Range>().ok()?, b.parse::<Range>().ok()?);
                Some((a, b))
            })
            .collect()
    }
}

fn solve_part1(input: Input) -> usize {
    input
        .into_pairs()
        .into_iter()
        .filter(|pair| pair.has_superset())
        .count()
}

fn solve_part2(input: Input) -> usize {
    input
        .into_pairs()
        .into_iter()
        .filter(|pair| pair.has_overlap())
        .count()
}

fn main() {
    let input = INPUT.parse::<Input>().expect("input must parse");
    let part1 = solve_part1(input.clone());
    println!("part1: {part1}");
    let part2 = solve_part2(input);
    println!("part2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn solve_part1() {
        let input: Input = INPUT.parse().expect("input must parse");
        let part1 = super::solve_part1(input);
        let expected = 2;
        assert_eq!(part1, expected);
    }

    #[test]
    fn solve_part2() {
        let input: Input = INPUT.parse().expect("input must parse");
        let part2 = super::solve_part2(input);
        let expected = 4;
        assert_eq!(part2, expected);
    }
}
