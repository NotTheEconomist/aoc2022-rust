use std::str::FromStr;

use day8::Forest;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(input: Input) -> usize {
    let forest = Forest::new(input.0);
    forest
        .iter_coords()
        .filter(|&coords| forest.is_tree_visible(coords).unwrap_or(false))
        .count()
}
fn solve_part2(input: Input) -> usize {
    let forest = Forest::new(input.0);
    forest
        .iter_coords()
        .filter_map(|coords| forest.scenic_score(coords))
        .max()
        .unwrap()
}

fn main() {
    let input: Input = INPUT.parse().expect("input must parse");
    let part1 = solve_part1(input.clone());
    println!("part1: {part1}");
    let part2 = solve_part2(input);
    println!("part2: {part2}");
}

#[derive(Clone, Debug)]
struct Input(Vec<Vec<u8>>);
impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).map(|n| n as u8))
                    .collect::<Option<Vec<u8>>>()
            })
            .collect::<Option<Vec<Vec<u8>>>>()
            .ok_or_else(|| String::from("Failed to parse input"))?;
        Ok(Self(data))
    }
}
