use std::fs::read_to_string;

use regex::Regex;

type State = Vec<Vec<char>>;
type Tasks = Vec<(usize, usize, usize)>;

pub fn solution(file: &str, keep_order: bool) -> String {
    let (init_state, tasks) = parse_game(file);
    let mut state = init_state;

    for (moves, from, to) in tasks {
        let s = &mut state;
        let mut creates = vec![];

        for _ in 0..moves {
            let c = s[from - 1].pop().unwrap();

            if keep_order {
                creates.insert(0, c);
            } else {
                creates.push(c)
            }
        }

        for c in creates {
            s[to - 1].push(c);
        }
    }

    state
        .iter()
        .map(|col| col[col.len() - 1])
        .collect::<String>()
}

pub fn parse_game(f: &str) -> (State, Tasks) {
    let content = read_to_string(f).unwrap();
    let lines = content.split("\n\n").collect::<Vec<_>>();
    let init_state = parse_field(lines[0]);
    let moves = parse_cmd(lines[1]);

    (init_state, moves)
}

pub fn parse_field(input: &str) -> State {
    let rows = input.split('\n').rev();
    let cols = rows
        .clone()
        .take(1)
        .map(|s| {
            s.trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().expect("failed to parse col"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let cols = &cols[0];

    rows.skip(1).fold(vec![vec![]; cols.len()], |mut acc, row| {
        cols.iter().for_each(|col| {
            let r = row.chars().nth((col - 1) * 4 + 1);

            if let Some(r) = r {
                if r.is_alphabetic() {
                    acc[col - 1].push(r);
                }
            }
        });

        acc
    })
}

pub fn parse_cmd(cmds: &str) -> Tasks {
    let cmds = cmds.split('\n').collect::<Vec<_>>();
    let mut tasks = vec![];

    for cmd in &cmds {
        let matcher = r"move (\d+) from (\d+) to (\d+)";
        let re = Regex::new(matcher).unwrap();

        for r in re.captures_iter(cmd) {
            let moves = r[1].parse::<usize>().unwrap();
            let from = r[2].parse::<usize>().unwrap();
            let to = r[3].parse::<usize>().unwrap();

            tasks.push((moves, from, to))
        }
    }

    tasks
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn d_05_example_a() {
        let res = solution("src/demo.txt", false);

        assert_eq!(res, "CMZ")
    }

    #[test]
    fn d_05_example_b() {
        let res = solution("src/demo.txt", true);

        assert_eq!(res, "MCD")
    }

    #[test]
    fn d_05_a() {
        let res = solution("src/input.txt", false);

        assert_eq!(res, "MQTPGLLDN")
    }

    #[test]
    fn d_05_b() {
        let res = solution("src/input.txt", true);

        assert_eq!(res, "LVZPSTTCZ")
    }
}
