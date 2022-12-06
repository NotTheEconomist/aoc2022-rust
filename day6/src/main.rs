use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = INPUT;
    let part1 = solve_part1(input).expect("part1 must have a solution");
    println!("part1: {part1}");
    let part2 = solve_part2(input).expect("part2 must have a solution");
    println!("part2: {part2}");
}

fn solve_part1(s: &'static str) -> Result<usize> {
    let (_, result) = day6::parser::line_through_start_code(s)?;
    Ok(result.len())
}
fn solve_part2(s: &'static str) -> Result<usize> {
    let (_, result) = day6::parser::line_through_message_code(s)?;
    Ok(result.len())
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn solve_part1() {
        let part1 = super::solve_part1(INPUT).expect("must have a solution");
        assert_eq!(part1, 7);
    }
}
