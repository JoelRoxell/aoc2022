use serde::Deserialize;
use std::{cmp::Ordering, fs::read_to_string};

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum Packet {
    N(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::N(b)) => a.cmp(&vec![Self::N(*b)]),
            (Self::N(a), Self::List(b)) => vec![Self::N(*a)].cmp(b),
            (Self::N(a), Self::N(b)) => a.cmp(b),
        }
    }
}

pub fn parse_input(target: &str) -> Vec<[Packet; 2]> {
    let data = read_to_string(target).unwrap();

    let data = data
        .split("\n\n")
        .map(|pair| {
            let mut chunks = pair.split('\n');
            let p1 = chunks.next().unwrap();
            let p2 = chunks.next().unwrap();
            let p1: Packet = serde_json::from_str(p1).unwrap();
            let p2: Packet = serde_json::from_str(p2).unwrap();

            [p1, p2]
        })
        .collect::<Vec<_>>();

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_13_example() {
        let pairs = parse_input("src/demo.txt");

        let res = pairs.iter().enumerate().fold(0, |mut acc, (i, pairs)| {
            if pairs[0] < pairs[1] {
                acc += i + 1;
            }

            acc
        });

        assert_eq!(res, 13)
    }

    #[test]
    fn d_13_a() {
        let pairs = parse_input("src/input.txt");

        let res = pairs.iter().enumerate().fold(0, |mut acc, (i, pairs)| {
            if pairs[0] < pairs[1] {
                acc += i + 1;
            }

            acc
        });

        assert_eq!(res, 5506)
    }

    #[test]
    fn d_13_b() {
        let packets = parse_input("src/input.txt");
        let mut packets = packets.iter().flatten().collect::<Vec<_>>();

        let d1: Packet = serde_json::from_str("[[2]]").unwrap();
        let d2: Packet = serde_json::from_str("[[6]]").unwrap();

        packets.push(&d1);
        packets.push(&d2);
        packets.sort_unstable();

        let res = packets.iter().enumerate().fold(1, |mut acc, (i, packet)| {
            if *packet == &d1 || *packet == &d2 {
                acc *= i + 1
            }

            acc
        });

        assert_eq!(res, 21756)
    }
}
