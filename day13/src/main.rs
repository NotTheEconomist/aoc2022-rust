use std::cmp::Ordering;

use day13::*;

const INPUT: &str = include_str!("input.txt");

fn solve_part1(s: &str) -> usize {
    let (_, packet_pairs) = parser::packet_pairs::<u8>(s).expect("input must parse");
    (1..)
        .zip(packet_pairs.into_iter())
        .filter_map(|(i, (a, b))| {
            if let Some(Ordering::Less) | Some(Ordering::Equal) = a.partial_cmp(&b) {
                Some(i)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part2(s: &str) -> usize {
    let (_, packet_pairs) = parser::packet_pairs::<u8>(s).expect("input must parse");
    let divisor_pairs = [
        parser::packet("[[2]]").map(|(_, packet)| packet).unwrap(),
        parser::packet("[[6]]").map(|(_, packet)| packet).unwrap(),
    ];
    let mut packets = packet_pairs
        .into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .chain(divisor_pairs.clone())
        .collect::<Vec<_>>();
    packets.sort();
    (1..)
        .zip(packets)
        .filter_map(|(i, packet)| {
            if divisor_pairs.contains(&packet) {
                Some(i)
            } else {
                None
            }
        })
        .product::<usize>()
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
        assert_eq!(super::solve_part1(INPUT), 13);
    }
    #[test]
    fn solve_part2() {
        assert_eq!(super::solve_part2(INPUT), 140);
    }
}
