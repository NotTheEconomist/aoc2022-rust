use std::{borrow::Borrow, collections::HashMap, str::FromStr};

use anyhow::Context;

#[derive(Debug)]
pub struct Location {
    coordinates: (usize, usize),
    pub height: usize,
    marker: Marker,
}

#[derive(Debug)]
enum Marker {
    Start,
    End,
    Nothing,
}

#[derive(Debug)]
pub struct Grid {
    grid: HashMap<(usize, usize), Location>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl AsRef<HashMap<(usize, usize), Location>> for Grid {
    fn as_ref(&self) -> &HashMap<(usize, usize), Location> {
        self.grid.borrow()
    }
}

pub type Edge = ((usize, usize), (usize, usize), usize);
impl Grid {
    pub fn edges(&self) -> Vec<Edge> {
        self.grid
            .iter()
            .flat_map(|(&coords @ (x, y), Location { height, .. })| {
                let right_neighbor = (x + 1, y);
                let down_neighbor = (x, y + 1);
                [right_neighbor, down_neighbor]
                    .into_iter()
                    // the flat_map eats the Optional failures when getting neighbors past the edge of the map
                    .flat_map(move |neighbor| {
                        self.grid
                            .get(&neighbor)
                            .map(
                                |Location {
                                     height: neighbor_height,
                                     ..
                                 }| {
                                    (neighbor, *neighbor_height as i32 - *height as i32)
                                },
                            )
                            .map(|(neighbor, height_diff)| {
                                match height_diff {
                                    // if the neighbor is 2 or more steps up, the neighbor can walk to me but not vice versa
                                    2..=i32::MAX => {
                                        vec![(neighbor, coords, height_diff as usize)]
                                    }
                                    // if the neighbor and I are within one step of each other, we can walk to each other
                                    -1..=1 => vec![
                                        (neighbor, coords, height_diff as usize),
                                        (coords, neighbor, -height_diff as usize),
                                    ],
                                    // if the neighbor is 2 or more steps down, I can walk to it but not vice versa
                                    i32::MIN..=-2 => {
                                        vec![(coords, neighbor, height_diff as usize)]
                                    }
                                }
                            })
                    })
                    .flatten()
            })
            .collect::<Vec<_>>()
    }
}
impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locations = s.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, height)| Location {
                coordinates: (x, y),
                height: match height {
                    'S' => 0,
                    'E' => 'z' as usize - 'a' as usize,
                    c => c as usize - 'a' as usize,
                },
                marker: match height {
                    'S' => Marker::Start,
                    'E' => Marker::End,
                    _ => Marker::Nothing,
                },
            })
        });
        let mut map = HashMap::new();
        for location in locations {
            map.insert(location.coordinates, location);
        }
        let start_coords = map
            .iter()
            .find(|(_, Location { marker, .. })| matches!(marker, Marker::Start))
            .map(|(coords, _)| *coords)
            .with_context(|| "No start coordinates exist")?;
        let end_coords = map
            .iter()
            .find(|(_, Location { marker, .. })| matches!(marker, Marker::End))
            .map(|(coords, _)| *coords)
            .with_context(|| "No end coordinates exist")?;

        Ok(Grid {
            grid: map,
            start: start_coords,
            end: end_coords,
        })
    }
}
