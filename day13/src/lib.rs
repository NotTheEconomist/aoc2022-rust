use std::cmp::Ordering;
use std::fmt::Debug;

pub mod parser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Packet<T: Clone + PartialOrd> {
    List(Vec<Box<Packet<T>>>),
    Num(T),
}

type Pair<T> = (Packet<T>, Packet<T>);

impl<T> Ord for Packet<T>
where
    T: Eq + Clone + PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> PartialOrd for Packet<T>
where
    T: Clone + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::List(this), Packet::List(other)) => {
                for (this, other) in this.iter().zip(other.iter()) {
                    let cmp = this.partial_cmp(other);
                    if let Some(Ordering::Equal) = cmp {
                        continue;
                    } else {
                        return cmp;
                    }
                }
                this.len().partial_cmp(&other.len())
            }
            (this @ Packet::List(_), Packet::Num(other)) => {
                this.partial_cmp(&Packet::List(vec![Box::new(Packet::Num(other.clone()))]))
            }
            (Packet::Num(this), other @ Packet::List(_)) => {
                Packet::List(vec![Box::new(Packet::Num(this.clone()))]).partial_cmp(other)
            }
            (Packet::Num(this), Packet::Num(other)) => this.partial_cmp(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packet_ordering() {
        let (_, (a, b)) =
            parser::packet_pair::<u8>("[1,2,3,4,5]\n[1,2,3,4,5]").expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Equal)));
        let (a, b) = (Packet::Num(3), Packet::Num(2));
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Greater)));
        let (_, (a, b)) =
            parser::packet_pair::<u8>("[1,1,3,1,1]\n[1,1,5,1,1]").expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Less)));
        let (_, (a, b)) =
            parser::packet_pair::<u8>("[[1],[2,3,4]]\n[[1],4]").expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Less)));
        let (_, (a, b)) = parser::packet_pair::<u8>("[9]\n[[8,7,6]]").expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Greater)));
        let (_, (a, b)) =
            parser::packet_pair::<u8>("[[4,4],4,4]\n[[4,4],4,4,4]").expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Less)));
        let (_, (a, b)) =
            parser::packet_pair::<u8>("[7,7,7,7]\n[7,7,7]").expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Greater)));
        let (_, (a, b)) = parser::packet_pair::<u8>("[]\n[3]").expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Less)));
        let (_, (a, b)) = parser::packet_pair::<u8>("[[[]]]\n[[]]").expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Greater)));
        let (_, (a, b)) =
            parser::packet_pair::<u8>("[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]")
                .expect("input must parse");
        assert!(matches!(a.partial_cmp(&b), Some(Ordering::Greater)));
    }
}
