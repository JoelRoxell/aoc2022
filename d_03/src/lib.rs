use shared::read_lines;
use std::{collections::HashMap, vec};

pub fn solution_a(target: &str) -> u32 {
    let lines = read_lines(target).unwrap();

    sum_chars(
        lines
            .iter()
            .flat_map(|line| {
                let n = line.len();

                items_matches_in_packs(&[line[..n / 2].to_owned(), line[n / 2..].to_owned()])
            })
            .collect(),
    )
}

pub fn solution_b(target: &str, chunks: usize) -> u32 {
    let lines = read_lines(target).unwrap();

    sum_chars(
        lines
            .chunks(chunks)
            .flat_map(items_matches_in_packs)
            .collect::<Vec<_>>(),
    )
}

fn items_matches_in_packs(packs: &[String]) -> Vec<char> {
    let mut matches: Vec<char> = vec![];
    let mut comps: Vec<HashMap<char, usize>> = packs.iter().map(|&_| HashMap::new()).collect();

    packs.iter().enumerate().for_each(|(i, pack)| {
        let comp = &mut comps[i];

        for p in pack.chars() {
            comp.entry(p).or_insert(1);
        }
    });

    comps[0].iter().for_each(|(key, _)| {
        let in_all = comps[1..].iter().all(|comp| comp.get(key).is_some());

        if in_all {
            matches.push(*key)
        }
    });

    matches
}

fn sum_chars(chars: Vec<char>) -> u32 {
    chars
        .iter()
        .map(|v| {
            if v.is_lowercase() {
                (*v as u32) - 96
            } else {
                *v as u32 - 38
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_03_example() {
        assert_eq!(solution_a("src/demo.txt"), 157);
    }

    #[test]
    fn d_03_example_b() {
        assert_eq!(solution_b("src/demo.txt", 3), 70);
    }

    #[test]
    fn d_03_a() {
        assert_eq!(solution_a("src/input.txt"), 8493);
    }

    #[test]
    fn d_03_b() {
        assert_eq!(solution_b("src/input.txt", 3), 2552);
    }
}
