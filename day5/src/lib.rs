pub mod parser;

use thiserror::Error;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Crate(char);
impl Crate {
    pub fn new(c: char) -> Self {
        Self(c)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
    kind: MoveKind,
}

impl Instruction {
    pub fn to_individual(self) -> Self {
        Self {
            kind: MoveKind::Individual,
            ..self
        }
    }
    pub fn to_grouped(self) -> Self {
        Self {
            kind: MoveKind::Grouped,
            ..self
        }
    }
    pub fn toggle_kind(self) -> Self {
        let new_kind = match self.kind {
            MoveKind::Individual => MoveKind::Grouped,
            MoveKind::Grouped => MoveKind::Individual,
        };
        Self {
            kind: new_kind,
            ..self
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum MoveKind {
    Individual,
    Grouped,
}

#[derive(Debug, Error)]
pub enum InstructionError {
    #[error("Tried to move an item from source {0} that does not exist")]
    SourceNotFound(usize),
    #[error("Tried to move an item to desination {0} that does not exist")]
    DestinationNotFound(usize),
    #[error("Tried to move {0} items, but only {1} exist")]
    InsufficientCountError(usize, usize),
    #[error("Tried to move more items than exist")]
    PopError,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Yard(Vec<Vec<Crate>>);
impl From<Vec<(usize, Crate)>> for Yard {
    fn from(input: Vec<(usize, Crate)>) -> Self {
        let mut result: Vec<Vec<Crate>> = Vec::new();
        for (slot, crate_) in input.into_iter() {
            while result.len() < slot {
                result.push(Vec::<Crate>::new());
            }
            let idx = slot - 1;
            result
                .get_mut(idx)
                .expect("we should create empty vectors to match the length just above")
                .push(crate_);
        }
        // since we construct these vectors from top down, we need to reverse them when finished so the
        // top is back at the top.
        for sub_vec in result.iter_mut() {
            sub_vec.reverse()
        }
        Self(result)
    }
}

impl Yard {
    pub fn new(data: Vec<Vec<Crate>>) -> Self {
        Self(data)
    }
    pub fn act(&mut self, instruction: Instruction) -> Result<(), InstructionError> {
        let src = self
            .0
            .get_mut(instruction.src)
            .ok_or(InstructionError::SourceNotFound(instruction.src))?;
        if src.len() < instruction.count {
            return Err(InstructionError::InsufficientCountError(
                instruction.count,
                src.len(),
            ));
        }
        let data = (0..instruction.count).map(|_| src.pop().ok_or(InstructionError::PopError));
        // since we're picking off the stack one by one, if we're supposed to be moving things
        // as a group we have to reverse the order here
        let data = match instruction.kind {
            MoveKind::Individual => data.collect::<Result<Vec<_>, _>>()?,
            MoveKind::Grouped => data
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .rev()
                .collect(),
        };
        let dst = self
            .0
            .get_mut(instruction.dst)
            .ok_or(InstructionError::DestinationNotFound(instruction.dst))?;
        dst.extend(data);
        Ok(())
    }

    pub fn topmost(&self) -> Option<String> {
        self.0
            .iter()
            .map(|v| {
                if let Some(Crate(c)) = v.last() {
                    Some(c)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yard_from_input() {
        let input = vec![
            (2, Crate('D')),
            (1, Crate('N')),
            (2, Crate('C')),
            (1, Crate('Z')),
            (2, Crate('M')),
            (3, Crate('P')),
        ];
        let yard: Yard = input.into();
        let expected = Yard(vec![
            vec![Crate('Z'), Crate('N')],
            vec![Crate('M'), Crate('C'), Crate('D')],
            vec![Crate('P')],
        ]);
        assert_eq!(yard, expected);
    }
    #[test]
    fn topmost() {
        let yard = Yard(vec![
            vec![Crate('C'), Crate('D')],
            vec![Crate('M')],
            vec![Crate('Z')],
        ]);
        assert_eq!(yard.topmost(), Some("DMZ".to_string()));
    }

    #[test]
    fn act() {
        let mut yard = Yard(vec![
            vec![Crate('C'), Crate('D')],
            vec![Crate('M')],
            vec![Crate('Z')],
        ]);
        let instruction = Instruction {
            count: 1,
            src: 0,
            dst: 1,
            kind: MoveKind::Individual,
        };
        yard.act(instruction).expect("act must succeed");

        let expected = Yard(vec![
            vec![Crate('C')],
            vec![Crate('M'), Crate('D')],
            vec![Crate('Z')],
        ]);
        assert_eq!(yard, expected);
    }
}
