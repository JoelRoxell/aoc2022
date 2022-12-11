use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

pub fn run_the_line(moves: Vec<(Direction, usize)>, knots: usize) -> usize {
    let mut knots = vec![(0, 0); knots];
    let mut visited = HashSet::new();

    for (direction, count) in moves {
        for _ in 0..count {
            match direction {
                Direction::Up => knots[0].0 += 1,
                Direction::Right => knots[0].1 += 1,
                Direction::Left => knots[0].1 -= 1,
                Direction::Down => knots[0].0 -= 1,
            }

            for i in 0..knots.len() {
                if i == 0 {
                    continue;
                }

                let row_step: isize = knots[i - 1].0 - knots[i].0;
                let col_step: isize = knots[i - 1].1 - knots[i].1;
                let not_touching = row_step.abs() > 1 || col_step.abs() > 1;

                if not_touching {
                    knots[i].0 += row_step.signum();
                    knots[i].1 += col_step.signum();
                }

                if i == knots.len() - 1 {
                    visited.insert(knots[i]);
                }
            }
        }
    }

    visited.len()
}

pub fn parse_input(steps: Vec<String>) -> impl Iterator<Item = (Direction, usize)> + 'static {
    steps.into_iter().map(|line| {
        let mut res = line.split(' ');
        let direction = res.next().unwrap();
        let count = res.next().unwrap();

        (
            match direction {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => {
                    panic!("failed to parse direction")
                }
            },
            count.parse::<usize>().expect("failed to parse step count"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::read_lines;

    #[test]
    fn d_09_example_a() {
        let moves = read_lines("src/demo.txt").unwrap();
        let moves = parse_input(moves).collect::<Vec<_>>();

        let res = run_the_line(moves, 2);

        assert_eq!(13, res);
    }

    #[test]
    fn d_09_a() {
        let moves = read_lines("src/input.txt").unwrap();
        let moves = parse_input(moves).collect::<Vec<_>>();

        let res = run_the_line(moves, 2);

        assert_eq!(5883, res);
    }

    #[test]
    fn d_09_b() {
        let moves = read_lines("src/input.txt").unwrap();
        let moves = parse_input(moves).collect::<Vec<_>>();

        let res = run_the_line(moves, 10);

        assert_eq!(2367, res)
    }
}
