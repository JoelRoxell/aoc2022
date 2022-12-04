use shared::read_lines;

pub fn solution(target: &str, overlap: bool) -> Result<usize, Box<dyn std::error::Error>> {
    let lines = read_lines(target)?;
    let pairs = lines
        .iter()
        .map(|line| {
            line.split(',')
                .map(|entry| {
                    let entries = entry.split('-').collect::<Vec<_>>();
                    let start = entries[0]
                        .parse::<usize>()
                        .expect("failed to parse start n");
                    let stop = entries[1]
                        .parse::<usize>()
                        .expect("failed to parse stop size");

                    (start..=stop).collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(pairs.iter().fold(0, |mut acc, pair| {
        let first = &pair[0];
        let second = &pair[1];

        let matches = first
            .iter()
            .filter(|v| second.contains(v))
            .collect::<Vec<_>>();

        if !overlap && matches.len() == first.len()
            || matches.len() == second.len()
            || overlap && !matches.is_empty()
        {
            acc += 1;
        }

        acc
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_04_example() {
        assert_eq!(solution("src/demo.txt", false).unwrap(), 2)
    }

    #[test]
    fn d_04_a() {
        assert_eq!(solution("src/input.txt", false).unwrap(), 490)
    }

    #[test]
    fn d_04_b() {
        assert_eq!(solution("src/input.txt", true).unwrap(), 921)
    }
}
