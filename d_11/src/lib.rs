use std::fs::read_to_string;

use regex::Regex;

pub fn solution(file: &str, loops: usize, part2: bool) -> isize {
    let mut ms = parse_input(file);
    let mut inspections = vec![0; ms.len()];
    let prime_product = ms.iter().map(|v| v.test).product::<isize>();

    for _ in 0..loops {
        for i in 0..ms.len() {
            let items = ms[i].items.drain(..).collect::<Vec<_>>();

            let current = ms[i].clone();

            for old in items {
                let new = match current.op {
                    Symbol::Add(v) => v + old as isize,
                    Symbol::Mult(v) => v * old as isize,
                    Symbol::Pow => (old as isize).pow(2),
                };

                let new = match part2 {
                    true => new % prime_product,
                    false => new / 3,
                };

                let rc_i = if new % current.test == 0 {
                    current.right
                } else {
                    current.left
                };

                inspections[i] += 1;
                ms[rc_i].items.push(new);
            }
        }
    }
    inspections.sort();

    inspections.iter().rev().take(2).product()
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Add(isize),
    Mult(isize),
    Pow,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<isize>,
    pub op: Symbol,
    pub test: isize,
    pub right: usize,
    pub left: usize,
}

pub fn parse_input(target: &str) -> Vec<Monkey> {
    let lines = read_to_string(target).unwrap();
    let number = Regex::new(r"(\d+)").unwrap();
    let op = Regex::new(r"old (\*|\+) (\d+|old)").unwrap();

    lines.split("\n\n").fold(vec![], |mut acc, raw_monkey| {
        let rows = raw_monkey.split('\n').collect::<Vec<_>>();
        let start_items = rows[1]
            .split("items:")
            .nth(1)
            .unwrap()
            .split(',')
            .map(|s| s.trim())
            .map(|s| s.parse::<isize>().expect("failed to parse item"))
            .collect::<Vec<_>>();

        let op = op.captures(rows[2]).unwrap();
        let sym = &op[1];
        let val = op[2].parse().unwrap_or(0);
        let sym = match sym {
            "*" if op[2].eq("old") => Symbol::Pow,
            "*" => Symbol::Mult(val),
            "+" => Symbol::Add(val),
            _ => panic!("not implemented symbol"),
        };

        let test = number.captures(rows[3]).unwrap()[1]
            .parse::<isize>()
            .unwrap();
        let right = number.captures(rows[4]).unwrap()[1]
            .parse::<usize>()
            .unwrap();
        let left = number.captures(rows[5]).unwrap()[1]
            .parse::<usize>()
            .unwrap();

        let m = Monkey {
            items: start_items,
            op: sym,
            test,
            right,
            left,
        };

        acc.push(m);

        acc
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn d_11_a() {
        assert_eq!(solution("src/input.txt", 20, false), 151312);
    }

    #[test]
    fn d_11_b() {
        assert_eq!(solution("src/input.txt", 10000, true), 51382025916);
    }
}
