use std::{convert::Infallible, str::FromStr};

use day10::{get_instructions_up_to_clock_cycle, parser, Instruction, Instructions, Register};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Input(String);
impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl Input {
    fn into_instructions(self) -> Result<Vec<Instruction>, String> {
        let (_, instructions) =
            parser::parse(&self.0).map_err(|_| "Failed to parse instructions".to_string())?;
        Ok(instructions)
    }
}

fn solve_part1(input: Input) -> i32 {
    let register = Register::default();
    let instructions = input
        .into_instructions()
        .expect("input must parse into instructions");
    let registers_at_cycles = [20, 60, 100, 140, 180, 220]
        .map(|cycle| {
            (
                cycle,
                get_instructions_up_to_clock_cycle(cycle, &instructions),
            )
        })
        .map(move |(cycle, instructions)| {
            (
                cycle,
                register.clone().reduce_instructions(instructions.copied()),
            )
        })
        .into_iter();

    registers_at_cycles.fold(0, |acc, (cycle, Register(n))| acc + n * (cycle as i32))
}

fn solve_part2(input: Input) -> String {
    let register = Register::default();
    let instructions: Instructions = input
        .into_instructions()
        .expect("input must parse into instructions")
        .into();
    instructions
        .scan(register, |register, instruction| {
            let result = register.clone();
            *register += instruction;
            Some(result)
        })
        .collect::<Vec<Register>>()
        .chunks_exact(40)
        .map(|line| {
            line.iter()
                .zip(0..)
                .map(|(Register(x), pixel)| (x - 1..=x + 1).contains(&pixel))
                .map(|enabled| if enabled { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    let input: Input = INPUT.parse().expect("input must parse");
    let part1 = solve_part1(input.clone());
    println!("part1: {part1}");
    let part2 = solve_part2(input);
    println!("part2: \n{part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn part1() {
        let input: Input = INPUT.parse().expect("Input must parse");
        let part1 = solve_part1(input);
        assert_eq!(part1, 13140);
    }
    #[test]
    fn part2() {
        let input: Input = INPUT.parse().expect("Input must parse");
        let part2 = solve_part2(input);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(part2, expected);
    }

    #[test]
    fn instructions_by_cycle() {
        let input: Instructions = INPUT
            .parse::<Input>()
            .unwrap()
            .into_instructions()
            .unwrap()
            .into();
        let instructions = input.collect::<Vec<Instruction>>();
        assert_eq!(
            instructions[..10],
            vec![
                Instruction::NoOp,
                Instruction::AddX(15),
                Instruction::NoOp,
                Instruction::AddX(-11),
                Instruction::NoOp,
                Instruction::AddX(6),
                Instruction::NoOp,
                Instruction::AddX(-3),
                Instruction::NoOp,
                Instruction::AddX(5)
            ]
        );
        let registers = instructions
            .into_iter()
            .scan(Register::default(), |register, instruction| {
                *register += instruction;
                Some(register.clone())
            })
            .collect::<Vec<Register>>();
        assert_eq!(
            registers.chunks_exact(40).next().unwrap()[..10],
            vec![
                Register(1),
                Register(16),
                Register(16),
                Register(5),
                Register(5),
                Register(11),
                Register(11),
                Register(8),
                Register(8),
                Register(13),
            ]
        );
    }
}
