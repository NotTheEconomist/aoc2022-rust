use std::str::FromStr;

use anyhow::Error;
use day2::{Choice, Outcome, Scorable};
use thiserror::Error;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct InputPart2(Vec<(Choice, Outcome)>);

impl FromStr for InputPart2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(' ')?;
                let a = match a {
                    "A" => Some(Choice::Rock),
                    "B" => Some(Choice::Paper),
                    "C" => Some(Choice::Scissors),
                    _ => None,
                }?;
                let b = match b {
                    "X" => Some(Outcome::Loss),
                    "Y" => Some(Outcome::Draw),
                    "Z" => Some(Outcome::Win),
                    _ => None,
                }?;
                Some((a, b))
            })
            .collect::<Option<Vec<(Choice, Outcome)>>>()
            .ok_or(MyError::ParseError)?;
        Ok(Self(lines))
    }
}

impl Scorable for InputPart2 {
    fn score(&self) -> u64 {
        self.0
            .iter()
            .map(|(their_choice, expected_outcome)| {
                let your_choice = match (their_choice, expected_outcome) {
                    (any, Outcome::Draw) => any.clone(),
                    (Choice::Rock, Outcome::Win) => Choice::Paper,
                    (Choice::Rock, Outcome::Loss) => Choice::Scissors,
                    (Choice::Scissors, Outcome::Win) => Choice::Rock,
                    (Choice::Scissors, Outcome::Loss) => Choice::Paper,
                    (Choice::Paper, Outcome::Win) => Choice::Scissors,
                    (Choice::Paper, Outcome::Loss) => Choice::Rock,
                };
                (their_choice.clone(), your_choice)
            })
            .fold(0, |acc, next| acc + next.score())
    }
}

#[derive(Debug, Clone)]
struct Input(Vec<(Choice, Choice)>);

impl Scorable for Input {
    fn score(&self) -> u64 {
        self.0.iter().fold(0, |acc, next| acc + next.score())
    }
}

#[derive(Error, Debug)]
enum MyError {
    #[error("One or more choices are badly formed")]
    ParseError,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(' ')?;
                let a = match a {
                    "A" => Some(Choice::Rock),
                    "B" => Some(Choice::Paper),
                    "C" => Some(Choice::Scissors),
                    _ => panic!("This should not be possible"),
                }?;
                let b = match b {
                    "X" => Some(Choice::Rock),
                    "Y" => Some(Choice::Paper),
                    "Z" => Some(Choice::Scissors),
                    _ => panic!("This should not be possible"),
                }?;
                Some((a, b))
            })
            .collect::<Option<Vec<(Choice, Choice)>>>()
            .ok_or(MyError::ParseError)?;
        Ok(Input(lines))
    }
}

fn solve_part1<I: Scorable>(input: I) -> u64 {
    input.score()
}

fn main() {
    let input: Input = INPUT.parse().expect("input must parse");
    let part1 = solve_part1(input);
    println!("part1: {}", part1);
    let input: InputPart2 = INPUT.parse().expect("input must parse");
    let part2 = solve_part1(input);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn solve_part1() {
        let input: super::Input = INPUT.parse().expect("input must parse");
        assert_eq!(super::solve_part1(input), 15);
    }

    #[test]
    fn solve_part2() {
        let input: super::InputPart2 = INPUT.parse().expect("input must parse");
        assert_eq!(super::solve_part1(input), 12);
    }
}
