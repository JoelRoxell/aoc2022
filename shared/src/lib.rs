use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::{str, vec};

pub fn read_lines(filepath: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let line_buff = create_file_buffer(filepath).lines();

    Ok(line_buff.fold(vec![], |mut acc, v| {
        match v {
            Ok(s) => acc.push(s),
            Err(err) => println!("read_lines::failed to read entry {}", err),
        }

        acc
    }))
}

pub fn read_symbol_separated_items(
    filepath: &str,
    sep: char,
) -> Result<Vec<String>, Box<dyn Error>> {
    let entires = create_file_buffer(filepath).split(sep as u8);

    // TODO: proper err checks...
    let res = entires.fold(vec![], |mut acc, section| {
        let r = str::from_utf8(&section.unwrap())
            .unwrap()
            .trim()
            .to_string();

        acc.push(r);

        acc
    });

    Ok(res)
}

pub fn create_file_buffer(filepath: &str) -> io::BufReader<File> {
    let f = File::open(filepath).unwrap_or_else(|_| panic!("couldn't find file {}", filepath));

    io::BufReader::new(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_lines_test() {
        let list = read_lines("./res/data-test-1.txt").unwrap();

        assert_eq!(list.len(), 5)
    }

    #[test]
    fn read_symbol_separated_items_test() {
        let list = read_symbol_separated_items("./res/data-test-2.txt", ',').unwrap();

        println!("{:?}", list);

        assert_eq!(list.len(), 7)
    }
}
