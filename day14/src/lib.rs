use std::{collections::HashMap, fmt::Display};

pub mod parser;

pub trait Movable: Copy {
    fn down(self) -> Self;
    fn left(self) -> Self;
    fn right(self) -> Self;
}

impl Movable for (i32, i32) {
    fn down(self) -> Self {
        let (x, y) = self;
        (x, y + 1)
    }

    fn left(self) -> Self {
        let (x, y) = self;
        (x - 1, y)
    }

    fn right(self) -> Self {
        let (x, y) = self;
        (x + 1, y)
    }
}

#[derive(Clone, Debug, Copy)]
enum Object {
    Wall,
    Sand,
}

#[derive(Clone, Debug)]
pub struct SandMaze {
    tiles: HashMap<(i32, i32), Object>,
    sand_origin: (i32, i32),
    floor: Option<i32>,
}

impl Display for SandMaze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = *self
            .tiles
            .keys()
            .map(|(x, _)| x)
            .min_by(|x1, x2| x1.cmp(x2))
            .unwrap()
            - 1;
        let max_x = *self
            .tiles
            .keys()
            .map(|(x, _)| x)
            .max_by(|x1, x2| x1.cmp(x2))
            .unwrap()
            + 1;
        let min_y = *self
            .tiles
            .keys()
            .map(|(_, y)| y)
            .min_by(|y1, y2| y1.cmp(y2))
            .unwrap();
        let min_y = if self.sand_origin.1 < min_y {
            self.sand_origin.1 - 1
        } else {
            min_y - 1
        };
        let max_y = *self
            .tiles
            .keys()
            .map(|(_, y)| y)
            .max_by(|y1, y2| y1.cmp(y2))
            .unwrap()
            + 1;
        writeln!(
            f,
            "+{}+",
            ['-']
                .repeat((max_x as i32 - min_x as i32) as usize + 1)
                .into_iter()
                .collect::<String>()
        )?;
        for y in min_y..=max_y {
            write!(f, "|")?;
            for x in min_x..=max_x {
                let symbol = if (x, y) == self.sand_origin {
                    '+'
                } else {
                    match self.tiles.get(&(x, y)) {
                        None => '.',
                        Some(Object::Wall) => '#',
                        Some(Object::Sand) => 'o',
                    }
                };
                write!(f, "{symbol}")?
            }
            writeln!(f, "|")?
        }
        writeln!(
            f,
            "+{}+",
            ['-']
                .repeat(max_x as usize - min_x as usize + 1)
                .into_iter()
                .collect::<String>()
        )?;
        Ok(())
    }
}

impl Iterator for SandMaze {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

impl SandMaze {
    pub fn from_lines(lines: Vec<(i32, i32)>, sand_origin: (i32, i32)) -> Self {
        let tiles = lines
            .into_iter()
            .map(|coords| (coords, Object::Wall))
            .collect::<HashMap<_, _>>();
        Self {
            tiles,
            sand_origin,
            floor: None,
        }
    }

    pub fn from_lines_with_floor(lines: Vec<(i32, i32)>, sand_origin: (i32, i32)) -> Self {
        let floor_y = lines.iter().map(|(_, y)| y).max().unwrap_or(&0) + 2;
        let tiles = lines
            .into_iter()
            .map(|coords| (coords, Object::Wall))
            .collect::<HashMap<_, _>>();
        Self {
            tiles,
            sand_origin,
            floor: Some(floor_y),
        }
    }
    pub fn step(&mut self) -> Option<(i32, i32)> {
        if self.tiles.get(&self.sand_origin).is_some() {
            return None;
        }
        let mut sand = self.sand_origin;
        let floor = self.floor.unwrap_or(i32::MAX);
        loop {
            let new_location = [sand.down(), sand.left().down(), sand.right().down()]
                .into_iter()
                .fold(
                    None,
                    |acc: Option<(i32, i32)>, next @ (_, y)| -> Option<(i32, i32)> {
                        acc.or(if self.tiles.get(&next).is_none() && y < floor {
                            Some(next)
                        } else {
                            None
                        })
                    },
                );
            match new_location {
                None => {
                    self.tiles.insert(sand, Object::Sand);
                    return Some(sand);
                }
                Some((_, 200..=i32::MAX)) => return None,
                Some(new_location) => {
                    sand = new_location;
                }
            }
        }
    }
}
