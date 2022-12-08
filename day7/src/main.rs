use day7::instructions::Instruction;
use day7::FileSystem;

use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input: Input = INPUT.parse().expect("input must parse");
    let part1 = solve_part1(input.clone());
    println!("part1: {part1}");
    let part2 = solve_part2(input);
    println!("part2: {part2}");
}

fn solve_part1(input: Input) -> u64 {
    let filesystem: FileSystem = input.0.into();
    filesystem
        .sizes()
        .into_iter()
        .filter_map(|(_, size)| if size <= 100000 { Some(size) } else { None })
        .sum::<u64>()
}

fn solve_part2(input: Input) -> u64 {
    let filesystem: FileSystem = input.0.into();
    let sizes = filesystem.sizes();
    let total_space: u64 = 70000000;
    let used_space: u64 = *sizes
        .get(&vec!["".to_string()])
        .expect("the root must have a size");
    let free_space = total_space - used_space;
    let needed_space: u64 = 30000000;
    let must_free = needed_space - free_space;
    sizes
        .into_iter()
        .filter_map(|(_, size)| if size >= must_free { Some(size) } else { None })
        .min()
        .unwrap_or(must_free)
}

#[derive(Debug, Clone)]
struct Input(Vec<Instruction>);
impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, instructions) =
            day7::parser::parse(s).map_err(|_| "input must parse".to_string())?;
        Ok(Self(instructions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn solve_part1() {
        let input: Input = INPUT.parse().expect("input must parse");
        let got = super::solve_part1(input);
        assert_eq!(got, 95437)
    }
}
