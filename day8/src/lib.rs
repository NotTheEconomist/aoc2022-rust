pub struct Forest(Vec<Vec<u8>>);

trait CoordinateWayfinding
where
    Self: Sized,
{
    fn up(&self) -> Option<Self>;
    fn right(&self) -> Option<Self>;
    fn left(&self) -> Option<Self>;
    fn down(&self) -> Option<Self>;
}

type Coords = (usize, usize);
impl CoordinateWayfinding for Coords {
    fn up(&self) -> Option<Self> {
        let (x, y) = *self;
        let y = y.checked_sub(1)?;
        Some((x, y))
    }

    fn right(&self) -> Option<Self> {
        let (x, y) = *self;
        let x = x.checked_add(1)?;
        Some((x, y))
    }

    fn left(&self) -> Option<Self> {
        let (x, y) = *self;
        let x = x.checked_sub(1)?;
        Some((x, y))
    }

    fn down(&self) -> Option<Self> {
        let (x, y) = *self;
        let y = y.checked_add(1)?;
        Some((x, y))
    }
}

impl Forest {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        Self(data)
    }
    pub fn iter_coords(&self) -> impl Iterator<Item = Coords> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
            .collect::<Vec<_>>()
            .into_iter()
    }
    pub fn in_forest(&self, coords: Coords) -> bool {
        let height = self.0.len();
        let width = self.0.get(0).unwrap().len();
        let (x, y) = coords;
        y <= height && x <= width
    }
    fn up_from(&self, coords: Coords) -> Vec<&u8> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, height)| {
                    if x == coords.0 && y < coords.1 {
                        Some(height)
                    } else {
                        None
                    }
                })
            })
            .rev()
            .collect()
    }
    fn down_from(&self, coords: Coords) -> Vec<&u8> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, height)| {
                    if x == coords.0 && y > coords.1 {
                        Some(height)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
    fn left_from(&self, coords: Coords) -> Vec<&u8> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, height)| {
                    if y == coords.1 && x < coords.0 {
                        Some(height)
                    } else {
                        None
                    }
                })
            })
            .rev()
            .collect()
    }
    fn right_from(&self, coords: Coords) -> Vec<&u8> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, height)| {
                    if y == coords.1 && x > coords.0 {
                        Some(height)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn scenic_score(&self, coords: Coords) -> Option<usize> {
        let (x, y) = coords;
        let tree_height = self.0.get(y)?.get(x)?;
        Some(
            [
                self.up_from(coords),
                self.right_from(coords),
                self.down_from(coords),
                self.left_from(coords),
            ]
            .map(|path| {
                path.into_iter()
                    .fold((0, false), |(count, should_stop), neighbor_height| {
                        if should_stop {
                            (count, should_stop)
                        } else {
                            (count + 1, should_stop || neighbor_height >= tree_height)
                        }
                    })
            })
            .map(|(count, _)| count)
            .into_iter()
            .product(),
        )
    }

    pub fn is_tree_visible(&self, coords: Coords) -> Option<bool> {
        let tree_height = self.0.get(coords.1)?.get(coords.0)?;
        Some(
            [
                self.left_from(coords),
                self.up_from(coords),
                self.right_from(coords),
                self.down_from(coords),
            ]
            .into_iter()
            .any(|direction| {
                direction
                    .into_iter()
                    .all(|neighbor_height| neighbor_height < tree_height)
            }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_tree_visible_count() {
        let input = Forest(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        let count = input
            .iter_coords()
            .filter(|&coords| input.is_tree_visible(coords).unwrap_or(false))
            .count();
        assert_eq!(count, 21);
    }

    #[test]
    fn scenic_score() {
        let input = Forest(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        let coords = (2, 1);
        assert_eq!(input.scenic_score(coords), Some(4));
        let coords = (2, 3);
        assert_eq!(input.scenic_score(coords), Some(8));
    }
}
