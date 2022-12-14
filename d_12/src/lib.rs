use shared::read_lines;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    vec,
};

pub fn solution(
    field: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    part2: bool,
) -> usize {
    let mut explored = HashSet::new();
    let mut frontier = BinaryHeap::new();

    let row_bounds = field.len() - 1;
    let col_bounds = field[0].len() - 1;

    let directions = vec![
        Direction::Up,
        Direction::Left,
        Direction::Right,
        Direction::Down,
    ];

    let start = Node {
        cost: 0,
        index: (start.0, start.1),
    };

    frontier.push(start);

    while let Some(current) = &mut frontier.pop() {
        let current_height = field[current.index.0][current.index.1];

        match part2 {
            true => {
                if current_height == 0 {
                    return current.cost;
                }
            }
            false => {
                if current.index == end {
                    return current.cost;
                }
            }
        }

        for direction in directions.iter() {
            let dir = match direction {
                Direction::Up => (-1, 0),
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
                Direction::Down => (1, 0),
            };

            let next_index = (
                (current.index.0 as isize - dir.0) as usize,
                (current.index.1 as isize - dir.1) as usize,
            );

            if next_index.0 > row_bounds || next_index.1 > col_bounds {
                continue;
            }

            let next_height = field[next_index.0][next_index.1];

            if (next_height as isize
                == (current_height as isize
                    + match part2 {
                        true => -1,
                        false => 1,
                    }))
                || match part2 {
                    true => next_height >= current_height,
                    false => next_height <= current_height,
                }
            {
                let n = Node {
                    index: (next_index.0, next_index.1),
                    cost: current.cost + 1,
                };

                if explored.insert(n.clone()) {
                    frontier.push(n);
                };
            }
        }

        explored.insert(current.clone());
    }

    usize::MAX
}

type Coordinate = (usize, usize);

pub fn parse_field(target: &str) -> (Vec<Vec<u8>>, Coordinate, Coordinate) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let field = read_lines(target)
        .expect("failed to read data file")
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    (match c {
                        'S' => {
                            start = (i, j);

                            'a'
                        }
                        'E' => {
                            end = (i, j);

                            'z'
                        }
                        _ => c,
                    }) as u8
                        - b'a'
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (field, start, end)
}

enum Direction {
    Up,
    Left,
    Right,
    Down,
}

type Index = (usize, usize);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Node {
    index: Index,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_12_example() {
        let (field, start, end) = parse_field("src/demo.txt");
        let res = solution(field, start, end, false);

        assert_eq!(res, 31);
    }

    #[test]
    fn d_12_example_b() {
        let (field, start, end) = parse_field("src/demo.txt");
        let res = solution(field, end, start, true);

        assert_eq!(res, 29);
    }

    #[test]
    #[ignore]
    fn d_12_a() {
        let (field, start, end) = parse_field("src/input.txt");
        let res = solution(field, start, end, false);

        assert_eq!(res, 440);
    }

    #[test]
    #[ignore]
    fn d_12_b() {
        let (field, start, end) = parse_field("src/input.txt");
        let res = solution(field, end, start, true);

        assert_eq!(res, 439);
    }
}
