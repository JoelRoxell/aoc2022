pub fn is_visible(field: &Field, target: Coordinate, (row_max, col_max): Coordinate) -> bool {
    let current = field[target.0][target.1];
    let dirs = vec![
        Directions::Left,
        Directions::Right,
        Directions::Up,
        Directions::Down,
    ];

    dirs.into_iter().any(|dir| {
        Position::new(target.0, target.1, dir, row_max, col_max)
            .all(|tree| field[tree.0][tree.1] < current)
    })
}

pub fn scores(field: &Field, target: Coordinate, (row_max, col_max): Coordinate) -> i32 {
    let current = field[target.0][target.1];
    let dirs = vec![
        Directions::Left,
        Directions::Right,
        Directions::Up,
        Directions::Down,
    ];

    dirs.into_iter()
        .map(|dir| {
            let mut count = 0;

            for tree in Position::new(target.0, target.1, dir, row_max, col_max) {
                let v = field[tree.0][tree.1];

                if v >= current {
                    count += 1;
                    break;
                } else if v < current {
                    count += 1
                }
            }

            count
        })
        .product()
}

type Field = Vec<Vec<usize>>;
type Coordinate = (usize, usize);

struct Position {
    row: usize,
    col: usize,
    direction: Directions,
    max_row: usize,
    max_col: usize,
}

impl Position {
    fn new(row: usize, col: usize, direction: Directions, max_row: usize, max_col: usize) -> Self {
        Position {
            row,
            col,
            direction,
            max_row,
            max_col,
        }
    }
}

impl Iterator for Position {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col == 0 || self.row == 0 {
            return None;
        }

        match self.direction {
            Directions::Left => self.col -= 1,
            Directions::Right => self.col += 1,
            Directions::Up => self.row -= 1,
            Directions::Down => self.row += 1,
        }

        if self.row >= self.max_row || self.col >= self.max_col {
            None
        } else {
            Some((self.row, self.col))
        }
    }
}

enum Directions {
    Left,
    Right,
    Up,
    Down,
}

pub fn parse_field(field: Vec<String>) -> (Field, Coordinate) {
    let field = field
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let row_max = field.len();
    let col_max = field[0].len();

    (field, (row_max, col_max))
}

#[cfg(test)]
mod tests {
    use shared::read_lines;

    use super::*;

    #[test]
    fn d_08_a() {
        let lines = read_lines("src/input.txt").unwrap();
        let (field, bounds) = parse_field(lines);
        let mut visible = 0;

        (0..bounds.0).for_each(|row| {
            (0..bounds.1).for_each(|col| match is_visible(&field, (row, col), bounds) {
                true => visible += 1,
                false => {}
            })
        });

        assert_eq!(visible, 1787);
    }

    #[test]
    fn d_08_b() {
        let lines = read_lines("src/input.txt").unwrap();
        let (field, bounds) = parse_field(lines);

        let res = (0..bounds.0).fold(vec![], |mut acc, row| {
            (0..bounds.1).for_each(|col| acc.push(scores(&field, (row, col), bounds)));

            acc
        });

        assert_eq!(*res.iter().max().unwrap(), 440640);
    }
}
