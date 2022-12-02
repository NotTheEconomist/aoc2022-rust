pub trait Scorable {
    fn score(&self) -> u64;
}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Scorable for Outcome {
    fn score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Scorable for Choice {
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Scorable for (Choice, Choice) {
    fn score(&self) -> u64 {
        let (a, b) = self;
        // do not score the opponent's choice
        b.score() + b.resolve_against(a).score()
    }
}

impl Choice {
    pub fn resolve_against(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Choice::Rock, Choice::Paper) => Outcome::Loss,
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Rock, Choice::Rock) => Outcome::Draw,
            (Choice::Paper, Choice::Paper) => Outcome::Draw,
            (Choice::Paper, Choice::Scissors) => Outcome::Loss,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
            (Choice::Scissors, Choice::Rock) => Outcome::Loss,
        }
    }
}
