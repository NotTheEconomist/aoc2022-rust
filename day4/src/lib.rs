use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not parse {} into {}", .input, .target)]
    ParseError { input: String, target: String },
}

pub trait HasSuperset {
    fn has_superset(&self) -> bool;
}

pub trait HasOverlap {
    fn has_overlap(&self) -> bool;
}

pub type Pair = (Range, Range);
impl HasSuperset for Pair {
    fn has_superset(&self) -> bool {
        self.0.is_superset_of(&self.1) || self.1.is_superset_of(&self.0)
    }
}
impl HasOverlap for Pair {
    fn has_overlap(&self) -> bool {
        self.0.contains(self.1.start)
            || self.0.contains(self.1.end)
            || self.1.contains(self.0.start)
            || self.1.contains(self.0.end)
    }
}
pub struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn contains(&self, n: i32) -> bool {
        self.start <= n && n <= self.end
    }

    pub fn is_superset_of(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or_else(|| Error::ParseError {
            input: s.to_string(),
            target: "Range".to_string(),
        })?;
        let (start, end) = (
            start.parse().map_err(|_| Error::ParseError {
                input: s.to_string(),
                target: "Range".to_string(),
            })?,
            end.parse().map_err(|_| Error::ParseError {
                input: s.to_string(),
                target: "Range".to_string(),
            })?,
        );
        Ok(Self { start, end })
    }
}
