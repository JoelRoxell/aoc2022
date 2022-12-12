use std::vec;

pub enum Instruction {
    Noop,
    Add(isize),
}

pub fn cpu(instructions: Vec<Instruction>, spots: Vec<usize>) -> isize {
    let mut cycle_mems = vec![];
    let mut cycle = 0;
    let mut reg = 1;

    for instruction in instructions {
        let op_cost = match instruction {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        };

        for _ in 0..op_cost {
            cycle += 1;

            if spots.contains(&cycle) {
                cycle_mems.push(reg * cycle as isize)
            }
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::Add(v) => reg += v,
        }
    }

    cycle_mems.iter().fold(0, |mut acc, v| {
        acc += v;
        acc
    })
}

const COLS: usize = 40;
const ROWS: usize = 6;
const SPRITE_WIDTH: usize = 3;

pub fn write_display(instructions: Vec<Instruction>) -> String {
    let mut cycle = 1;
    let mut reg = 1;
    let mut display = ['.'; ROWS * COLS];

    fn get_symbol(cycle: usize, reg: isize) -> char {
        let col = (cycle - 1) % COLS;
        let in_sprite = (col as isize).abs_diff(reg) <= SPRITE_WIDTH / 2;

        if in_sprite {
            '#'
        } else {
            '.'
        }
    }

    for instruction in instructions {
        display[cycle - 1] = get_symbol(cycle, reg);
        cycle += 1;

        match instruction {
            Instruction::Noop => {}
            Instruction::Add(v) => {
                display[cycle - 1] = get_symbol(cycle, reg);
                cycle += 1;
                reg += v;
            }
        }
    }

    display
        .chunks(display.len() / ROWS)
        .map(|line| line.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn parse_instructions(lines: Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .map(|line| {
            let mut r = line.split(' ');

            let op = r.next().unwrap();
            let v = r.next().unwrap_or("");

            match op {
                "addx" => Instruction::Add(v.parse().unwrap_or(0)),
                _ => Instruction::Noop,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_10_example() {
        let instructions = parse_instructions(shared::read_lines("src/demo.txt").unwrap());

        let res = cpu(instructions, vec![20, 60, 100, 140, 180, 220]);

        assert_eq!(res, 13140)
    }

    #[test]
    fn d_10_a() {
        let instructions = parse_instructions(shared::read_lines("src/input.txt").unwrap());

        let res = cpu(instructions, vec![20, 60, 100, 140, 180, 220]);

        assert_eq!(res, 13520)
    }

    #[test]
    fn d_10_b() {
        let instructions = parse_instructions(shared::read_lines("src/input.txt").unwrap());
        let img = write_display(instructions);

        println!("{img}")
    }
}
