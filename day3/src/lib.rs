use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rucksack(Compartment, Compartment);
impl Rucksack {
    pub fn new(first: Compartment, second: Compartment) -> Self {
        Self(first, second)
    }
    pub fn misplaced_item(&self) -> Option<u8> {
        self.0
            .counts
            .keys()
            .filter(|&key| self.1.counts.contains_key(key))
            .copied()
            .next()
    }
    pub fn get_badge_priority(&self, other: &Self, third: &Self) -> Option<u8> {
        self.0
            .counts
            .keys()
            .chain(self.1.counts.keys())
            .filter(|key| {
                other
                    .0
                    .counts
                    .keys()
                    .chain(other.1.counts.keys())
                    .collect::<HashSet<_>>()
                    .contains(key)
            })
            .filter(|key| {
                third
                    .0
                    .counts
                    .keys()
                    .chain(third.1.counts.keys())
                    .collect::<HashSet<_>>()
                    .contains(key)
            })
            .copied()
            .next()
    }
}

/// Gets the priority of an item
/// a - z maps to 1 - 26
/// A - Z maps to 27 - 52
/// all other values have no priority
pub fn get_priority(c: char) -> Option<u8> {
    if c.is_ascii_uppercase() {
        let offset = 65;
        <char as std::convert::TryInto<u8>>::try_into(c)
            .ok()
            .map(|v| v - offset + 27)
    } else if c.is_ascii_lowercase() {
        let offset = 97;
        <char as std::convert::TryInto<u8>>::try_into(c)
            .ok()
            .map(|v| v - offset + 1)
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compartment {
    counts: HashMap<u8, u32>,
}

impl Compartment {
    pub fn new(s: &str) -> Self {
        let mut counts = HashMap::new();
        for c in s.chars() {
            let priority = get_priority(c).expect("character must parse to u8");
            counts.entry(priority).and_modify(|p| *p += 1).or_insert(1);
        }

        Self { counts }
    }
}
