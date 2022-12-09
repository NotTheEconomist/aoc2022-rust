use std::collections::HashSet;

use day9::*;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (_, vectors) = parser::parse(INPUT).expect("input must parse");
    let mut record = HashSet::new();
    record.insert((0, 0));
    let mut tail = Tail::new();

    for vector in vectors.clone() {
        tail.apply_vector_and_record_steps(vector, &mut record);
    }
    let part1 = record.len();
    println!("part1: {part1}");

    let mut tail = LongTail::new();
    let mut record = HashSet::new();
    record.insert((0, 0));
    for vector in vectors {
        tail.apply_vector_and_record_steps(vector, &mut record);
    }
    let part2 = record.len();
    println!("part2: {part2}");
}
