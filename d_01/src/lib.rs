use shared::read_lines;

pub fn solution(target: &str, take: usize) -> usize {
    let lines = read_lines(target).unwrap();
    let mut i = 0;
    let mut res = lines.iter().fold(vec![], |mut elves, v| {
        if elves.get(i).is_none() {
            elves.push(0)
        }

        match v.parse::<usize>() {
            Ok(v) => {
                elves[i] += v;
            }
            Err(_) => {
                i += 1;
            }
        }

        elves
    });

    res.sort();

    let end = res.len();
    let start = end - take;

    res[start..end].iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn d_01_example() {
        assert_eq!(solution("./src/demo.txt", 1), 24000);
    }

    #[test]
    fn d_01a() {
        assert_eq!(solution("./src/input.txt", 1), 69883);
    }

    #[test]
    fn d_01b() {
        assert_eq!(solution("./src/input.txt", 3), 207576);
    }
}
