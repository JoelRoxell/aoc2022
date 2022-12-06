use std::collections::HashMap;

pub fn find_start_buff(buff: &str, flag_size: usize) -> usize {
    buff.chars()
        .collect::<Vec<_>>()
        .windows(flag_size)
        .position(|chunk| {
            let mut seen = HashMap::new();

            chunk.iter().enumerate().all(|(i, c)| match seen.get(c) {
                Some(_) => false,
                None => {
                    seen.insert(*c, i);

                    true
                }
            })
        })
        .expect("found no marker")
        + flag_size
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn d06_example() {
        let demo = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let flag = find_start_buff(demo, 4);

        assert_eq!(flag, 7);

        let demo = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let flag = find_start_buff(demo, 4);

        assert_eq!(flag, 5);
    }

    #[test]
    fn d06_a() {
        let data = read_to_string("src/input.txt").unwrap();
        let flag = find_start_buff(&data, 4);

        assert_eq!(flag, 1804);
    }

    #[test]
    fn d06_b() {
        let data = read_to_string("src/input.txt").unwrap();
        let flag = find_start_buff(&data, 14);

        assert_eq!(flag, 2508);
    }
}
