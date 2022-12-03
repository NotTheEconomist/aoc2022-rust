use std::{convert::Infallible, str::FromStr};

use day3::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
struct Input(Vec<String>);
impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(String::from).collect()))
    }
}
impl Input {
    fn into_rucksacks(self) -> Vec<Rucksack> {
        self.0
            .into_iter()
            .map(|line| {
                let length = line.len();
                let (first, second) = line.split_at(length / 2);
                assert_eq!(first.len(), second.len());
                let (c1, c2) = (Compartment::new(first), Compartment::new(second));
                Rucksack::new(c1, c2)
            })
            .collect()
    }
}

fn solve_part1(input: Input) -> u64 {
    input
        .into_rucksacks()
        .into_iter()
        .flat_map(|r| r.misplaced_item())
        .fold(0, |acc, next| acc + next as u64)
}

fn solve_part2(input: Input) -> u64 {
    input
        .into_rucksacks()
        .chunks_exact(3)
        .flat_map(|chunk| chunk[0].get_badge_priority(&chunk[1], &chunk[2]))
        .fold(0, |acc, next| acc + next as u64)
}

fn main() {
    let input: Input = INPUT.parse().expect("input must parse");
    let part1 = solve_part1(input.clone());
    println!("part1: {part1}");
    let part2 = solve_part2(input);
    println!("part2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn common_item() {
        let input: Input = INPUT.parse().expect("input must parse");
        let misplaced_items = input
            .into_rucksacks()
            .into_iter()
            .flat_map(|r| r.misplaced_item())
            .collect::<Vec<_>>();
        let expected = vec![16, 38, 42, 22, 20, 19];
        assert_eq!(misplaced_items, expected);
    }
    #[test]
    fn badge() {
        let input: Input = INPUT.parse().expect("input must parse");
        let badges = input
            .into_rucksacks()
            .chunks_exact(3)
            .flat_map(|chunk| chunk[0].get_badge_priority(&chunk[1], &chunk[2]))
            .collect::<Vec<_>>();
        let expect = vec![18, 52];
        assert_eq!(badges, expect);
    }
    #[test]
    fn rucksacks() {
        let input: Input = INPUT.parse().expect("input must parse");
        let rucksacks = input.into_rucksacks();
        let expected = vec![
            Rucksack::new(
                Compartment::new("vJrwpWtwJgWr"),
                Compartment::new("hcsFMMfFFhFp"),
            ),
            Rucksack::new(
                Compartment::new("jqHRNqRjqzjGDLGL"),
                Compartment::new("rsFMfFZSrLrFZsSL"),
            ),
            Rucksack::new(Compartment::new("PmmdzqPrV"), Compartment::new("vPwwTWBwg")),
            Rucksack::new(
                Compartment::new("wMqvLMZHhHMvwLH"),
                Compartment::new("jbvcjnnSBnvTQFn"),
            ),
            Rucksack::new(Compartment::new("ttgJtRGJ"), Compartment::new("QctTZtZT")),
            Rucksack::new(
                Compartment::new("CrZsJsPPZsGz"),
                Compartment::new("wwsLwLmpwMDw"),
            ),
        ];
        assert_eq!(rucksacks, expected);
    }
}
