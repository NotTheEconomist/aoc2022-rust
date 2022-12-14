use day14::*;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(s: &str) -> usize {
    let (_, walls) = parser::lines(s).expect("input must parse");
    let maze = SandMaze::from_lines(walls, (500, 0));
    (1usize..).zip(maze).map(|(i, _)| i).max().unwrap()
}

fn solve_part2(s: &str) -> usize {
    let (_, walls) = parser::lines(s).expect("input must parse");
    let maze = SandMaze::from_lines_with_floor(walls, (500, 0));
    (1usize..).zip(maze).map(|(i, _)| i).max().unwrap()
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
        let steps = super::solve_part1(INPUT);
        assert_eq!(steps, 24);
    }

    #[test]
    fn solve_part2() {
        let steps = super::solve_part2(INPUT);
        assert_eq!(steps, 93);
    }
}
