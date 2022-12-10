pub mod parser;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    pub fn cycle_cost(&self) -> u8 {
        match self {
            Self::NoOp => 1,
            Self::AddX(_) => 2,
        }
    }
}

pub struct Instructions {
    instructions: Vec<Instruction>,
    cursor: usize,
    delayed_instruction: (usize, Option<Instruction>),
}

impl From<Vec<Instruction>> for Instructions {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            cursor: 0,
            delayed_instruction: (0, None),
        }
    }
}

impl Iterator for Instructions {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        match self.delayed_instruction {
            (0, None) => { /* do nothing, go on! */ }
            (0, Some(instruction)) => {
                self.delayed_instruction = (0, None);
                return Some(instruction);
            }
            (delay, opt_instruction) => {
                self.delayed_instruction = (delay - 1, opt_instruction);
                return Some(Instruction::NoOp);
            }
        }
        match self.instructions.get(self.cursor)? {
            Instruction::NoOp => {
                self.cursor += 1;
                Some(Instruction::NoOp)
            }
            instruction @ Instruction::AddX(_) => {
                self.cursor += 1;
                self.delayed_instruction = (0, Some(*instruction));
                Some(Instruction::NoOp)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Register(pub i32);

impl Default for Register {
    fn default() -> Self {
        Self(1)
    }
}

impl AddAssign<Instruction> for Register {
    fn add_assign(&mut self, rhs: Instruction) {
        match rhs {
            Instruction::NoOp => {}
            Instruction::AddX(rhs) => self.0 += rhs,
        }
    }
}
impl Add<Instruction> for Register {
    type Output = Self;

    fn add(self, rhs: Instruction) -> Self::Output {
        match rhs {
            Instruction::NoOp => self,
            Instruction::AddX(n) => Self(self.0 + n),
        }
    }
}

/// From a list of instructions, return the portion that happen in `cycle` cycles or less
pub fn get_instructions_up_to_clock_cycle(
    cycle: usize,
    instructions: &[Instruction],
) -> impl Iterator<Item = &Instruction> {
    instructions.iter().scan(0, move |state, instruction| {
        *state += instruction.cycle_cost() as usize;
        if *state < cycle {
            Some(instruction)
        } else {
            None
        }
    })
}

impl Register {
    pub fn reduce_instructions(self, instructions: impl Iterator<Item = Instruction>) -> Self {
        instructions.fold(self, |register, instruction| register + instruction)
    }
    pub fn iter_with_instructions(&self, instructions: Vec<Instruction>) -> Vec<Register> {
        let mut instructions = instructions.into_iter();
        let mut cycle = 1;
        let mut register = Register::default();
        let mut registers = vec![register.clone()];
        while cycle <= 240 {
            match instructions.next() {
                Some(Instruction::NoOp) => {
                    registers.push(register.clone());
                    cycle += 1;
                }
                Some(instruction @ Instruction::AddX(_)) => {
                    registers.push(register.clone());
                    cycle += 1;
                    register += instruction;
                    registers.push(register.clone());
                    cycle += 1;
                }
                None => {
                    panic!("We ran out of instructions before we ran out of cycles");
                }
            }
        }
        todo!()
    }
}
