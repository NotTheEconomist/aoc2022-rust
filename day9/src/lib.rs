pub mod parser;
use std::{
    collections::HashSet,
    ops::{Add, AddAssign, SubAssign},
};

#[derive(Copy, Debug, Clone)]
pub struct Vector {
    x: i32,
    y: i32,
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl Vector {
    fn distance_between(a: &(i32, i32), b: &(i32, i32)) -> Self {
        Self {
            x: a.0 - b.0,
            y: a.1 - b.1,
        }
    }
    fn points_to_adjacent(&self) -> bool {
        matches!(
            &self,
            Vector {
                x: -1..=1,
                y: -1..=1
            }
        )
    }
}

trait Normalizable {
    fn normalize(self) -> Self;
}

impl Normalizable for Vector {
    fn normalize(self) -> Self {
        match self {
            Vector { x, y } if x < 0 && y < 0 => Vector { x: -1, y: -1 },
            Vector { x, y } if x > 0 && y < 0 => Vector { x: 1, y: -1 },
            Vector { x, y } if x < 0 && y > 0 => Vector { x: -1, y: 1 },
            Vector { x, y } if x > 0 && y > 0 => Vector { x: 1, y: 1 },
            Vector { x, y: 0 } if x < 0 => Vector { x: -1, y: 0 },
            Vector { x, y: 0 } if x > 0 => Vector { x: 1, y: 0 },
            Vector { x: 0, y } if y < 0 => Vector { x: 0, y: -1 },
            Vector { x: 0, y } if y > 0 => Vector { x: 0, y: 1 },
            _ => unreachable!(),
        }
    }
}

impl Normalizable for (i32, i32) {
    fn normalize(self) -> Self {
        match self {
            (x, y) if x < 0 && y < 0 => (-1, -1),
            (x, y) if x > 0 && y < 0 => (1, -1),
            (x, y) if x < 0 && y > 0 => (-1, 1),
            (x, y) if x > 0 && y > 0 => (1, 1),
            (x, 0) if x < 0 => (-1, 0),
            (x, 0) if x > 0 => (1, 0),
            (0, y) if y < 0 => (0, -1),
            (0, y) if y > 0 => (0, 1),
            _ => unreachable!(),
        }
    }
}

impl Vector {
    fn reverse(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add<Vector> for (i32, i32) {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        let (x, y) = self;
        (x + rhs.x, y + rhs.y)
    }
}

impl AddAssign<Vector> for (i32, i32) {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.x;
        self.1 += rhs.y;
    }
}

#[derive(Debug)]
pub struct LongTail {
    // stretching from head to tail
    positions: [(i32, i32); 10],
}

impl LongTail {
    pub fn new() -> Self {
        Self {
            positions: [(0, 0); 10],
        }
    }

    pub fn apply_vector_and_record_steps(
        &mut self,
        mut vector: Vector,
        record: &mut HashSet<(i32, i32)>,
    ) {
        while !matches!(vector, Vector { x: 0, y: 0 }) {
            let step = vector.normalize();
            vector -= step;
            self.positions[0] += step;
            for (i, next_i) in (0..self.positions.len()).zip(1..self.positions.len()) {
                let (leader, follower) =
                    (self.positions[i], self.positions.get_mut(next_i).unwrap());
                let difference = Vector::distance_between(&leader, follower);
                if difference.points_to_adjacent() {
                    continue;
                }
                let step = difference.normalize();
                *follower += step;
            }
            record.insert(*self.positions.last().unwrap());
        }
    }
}

impl Default for LongTail {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Tail {
    position: (i32, i32),
    relative_head_position: (i32, i32),
}

impl Tail {
    pub fn new() -> Self {
        Tail {
            position: (0, 0),
            relative_head_position: (0, 0),
        }
    }

    pub fn apply_vector_and_record_steps(
        &mut self,
        vector: Vector,
        record: &mut HashSet<(i32, i32)>,
    ) {
        self.relative_head_position += vector;

        let mut step_towards_head = || -> bool {
            if let Some(unit_vector) = match self.relative_head_position {
                (-1..=1, -1..=1) => None,
                (i32::MIN..=-2, 0) => Some(Vector { x: -1, y: 0 }),
                (2..=i32::MAX, 0) => Some(Vector { x: 1, y: 0 }),
                (0, i32::MIN..=-2) => Some(Vector { x: 0, y: -1 }),
                (0, 2..=i32::MAX) => Some(Vector { x: 0, y: 1 }),
                (x, y) if x < 0 && y < 0 => Some(Vector { x: -1, y: -1 }), // SW
                (x, y) if x < 0 && y > 0 => Some(Vector { x: -1, y: 1 }),  // NW
                (x, y) if x > 0 && y > 0 => Some(Vector { x: 1, y: 1 }),   // NE
                (x, y) if x > 0 && y < 0 => Some(Vector { x: 1, y: -1 }),  // SE
                _ => unreachable!(),
            } {
                self.position += unit_vector;
                self.relative_head_position += unit_vector.reverse();
                record.insert(self.position);
            }
            matches!(self.relative_head_position, (-1..=1, -1..=1))
        };

        loop {
            let should_stop = step_towards_head();
            if should_stop {
                break;
            }
        }
    }
}

impl Default for Tail {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tail_moves() {
        let mut tail = Tail::new();
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        set.insert((0, 0));
        tail.apply_vector_and_record_steps(Vector { x: 4, y: 0 }, &mut set);
        assert_eq!(set.len(), 4);
        assert!(set.contains(&(0, 0)));
        assert!(set.contains(&(1, 0)));
        assert!(set.contains(&(2, 0)));
        assert!(set.contains(&(3, 0)));
        tail.apply_vector_and_record_steps(Vector { x: 0, y: 3 }, &mut set);
        assert!(set.contains(&(4, 1)));
        assert!(set.contains(&(4, 2)));
        assert_eq!(set.len(), 6);
        tail.apply_vector_and_record_steps(Vector { x: -2, y: 0 }, &mut set);
        assert!(set.contains(&(3, 3)));
        assert_eq!(set.len(), 7);
        tail.apply_vector_and_record_steps(Vector { x: 0, y: -5 }, &mut set);
        assert!(set.contains(&(2, 2)));
        assert!(set.contains(&(2, 1)));
        assert!(set.contains(&(2, 0))); // already exists
        assert!(set.contains(&(2, -1)));
        assert_eq!(set.len(), 10);
    }

    #[test]
    fn long_tail_test_input() {
        let (_, vectors) = parser::parse(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        )
        .expect("input must parse");
        let mut tail = LongTail::new();
        let mut record = HashSet::new();
        record.insert((0, 0));
        for vector in vectors {
            tail.apply_vector_and_record_steps(vector, &mut record);
        }
        assert_eq!(record.len(), 36);
    }
}
