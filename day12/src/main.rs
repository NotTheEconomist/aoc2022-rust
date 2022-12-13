use day12::{Grid, Location};
use petgraph::{algo::dijkstra::dijkstra, graphmap::DiGraphMap};

const INPUT: &str = include_str!("input.txt");

fn solve_part1(s: &str) -> usize {
    let grid = s.parse::<Grid>().expect("input must parse");
    let graph = DiGraphMap::from_edges(grid.edges());
    let path = dijkstra(&graph, grid.start, Some(grid.end), |_| 1);
    *path.get(&grid.end).unwrap() as usize
}

fn solve_part2(s: &str) -> usize {
    let grid = s.parse::<Grid>().expect("input must parse");
    let graph = DiGraphMap::from_edges(grid.edges());
    let starting_points = grid
        .as_ref()
        .iter()
        .filter(|(_, Location { height, .. })| *height == 0)
        .map(|(&coords, _)| coords);
    starting_points
        .flat_map(|start| {
            dijkstra(&graph, start, Some(grid.end), |_| 1)
                .get(&grid.end)
                .map(|n| *n as usize)
        })
        .min()
        .unwrap()
}

fn main() {
    let part1 = solve_part1(INPUT);
    let part2 = solve_part2(INPUT);
    println!("part1: {part1}\npart2: {part2}");
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("test_input.txt");

    #[test]
    fn solve_part1() {
        let result = super::solve_part1(INPUT);
        assert_eq!(result, 31)
    }

    #[test]
    fn solve_part2() {
        let result = super::solve_part2(INPUT);
        assert_eq!(result, 29)
    }
}
