use shared::read_lines;
use std::error::Error;

pub fn solution(target: &str, transform: bool) -> Result<usize, Box<dyn Error>> {
    let sum = read_lines(target)?.iter().fold(0, |mut score, line| {
        let codes = line.split_whitespace().collect::<Vec<_>>();

        let op_raw = codes[0];
        let you_raw = codes[1];
        let op = Hand::from(op_raw);

        score += if transform {
            Hand::from(Hand::transform_cheat(op_raw, you_raw))
        } else {
            Hand::from(you_raw)
        }
        .vs(&op);

        score
    });

    Ok(sum)
}

#[derive(Debug, PartialEq)]
enum Variant {
    Rock = 1,
    Paper,
    Scissor,
}

enum CheatCode {
    Loose,
    Draw,
    Win,
}

#[derive(Debug)]
struct Hand {
    variant: Variant,
}

static WIN: usize = 6;
static DRAW: usize = 3;

impl From<&str> for Hand {
    fn from(char: &str) -> Self {
        let variant = match char {
            "A" => Variant::Rock,
            "X" => Variant::Rock,
            "B" => Variant::Paper,
            "Y" => Variant::Paper,
            "C" => Variant::Scissor,
            "Z" => Variant::Scissor,
            _ => panic!("not a variant in the game"),
        };

        Hand { variant }
    }
}

impl From<Variant> for Hand {
    fn from(variant: Variant) -> Self {
        Hand { variant }
    }
}

impl Hand {
    pub fn transform_cheat(char: &str, cheat_code: &str) -> Variant {
        let cheat_code = match cheat_code {
            "X" => CheatCode::Loose,
            "Y" => CheatCode::Draw,
            "Z" => CheatCode::Win,
            _ => panic!("no-code"),
        };

        match char {
            "A" => match cheat_code {
                CheatCode::Loose => Variant::Scissor,
                CheatCode::Draw => Variant::Rock,
                CheatCode::Win => Variant::Paper,
            },
            "B" => match cheat_code {
                CheatCode::Loose => Variant::Rock,
                CheatCode::Draw => Variant::Paper,
                CheatCode::Win => Variant::Scissor,
            },
            "C" => match cheat_code {
                CheatCode::Loose => Variant::Paper,
                CheatCode::Draw => Variant::Scissor,
                CheatCode::Win => Variant::Rock,
            },
            _ => panic!("not a variant in the game"),
        }
    }

    pub fn vs(self, vs: &Hand) -> usize {
        match self.variant {
            Variant::Rock => {
                if Variant::Scissor == vs.variant {
                    Variant::Rock as usize + WIN
                } else if Variant::Paper == vs.variant {
                    Variant::Rock as usize
                } else {
                    Variant::Rock as usize + DRAW
                }
            }
            Variant::Paper => {
                if Variant::Rock == vs.variant {
                    Variant::Paper as usize + WIN
                } else if Variant::Scissor == vs.variant {
                    Variant::Paper as usize
                } else {
                    Variant::Paper as usize + DRAW
                }
            }
            Variant::Scissor => {
                if Variant::Paper == vs.variant {
                    Variant::Scissor as usize + WIN
                } else if Variant::Rock == vs.variant {
                    Variant::Scissor as usize
                } else {
                    Variant::Scissor as usize + DRAW
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution;

    #[test]
    fn d_02_example_a() {
        assert_eq!(solution("./src/demo.txt", false).unwrap(), 15);
    }

    #[test]
    fn d_02_example_b() {
        assert_eq!(solution("./src/demo.txt", true).unwrap(), 12);
    }

    #[test]
    fn d_02a() {
        assert_eq!(solution("./src/input.txt", false).unwrap(), 11873);
    }

    #[test]
    fn d_02b() {
        assert_eq!(solution("./src/input.txt", true).unwrap(), 12014);
    }
}
