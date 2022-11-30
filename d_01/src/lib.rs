pub fn solution(n: usize) -> usize {
    n * 2
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn d_01_example() {
        assert_eq!(solution(2), 4);
    }
}
