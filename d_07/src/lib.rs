use std::{collections::HashMap, io::BufRead, vec};

use regex::Regex;

pub fn solution(path: &str) -> Vec<(String, usize)> {
    let mut mem: HashMap<String, DirStruct> = HashMap::new();
    let mut cwd: Vec<String> = vec![];
    let mut output_buff = vec![];

    shared::create_file_buffer(path).lines().for_each(|line| {
        let output = line.expect("read line failed");
        let cmd = parse_cmd(&output);

        if !output_buff.is_empty() && cmd.is_some() {
            let path = cwd.join("/");

            match parse_cmd_output(&mut output_buff, &path) {
                Some(level) => {
                    mem.insert(level.path.clone(), level);
                }
                None => {
                    eprintln!("Couldn't parse cmd output buffer");
                }
            }
        }

        if let Some(cmd) = cmd {
            match cmd {
                Cmd::Ls | Cmd::Exit => {}
                Cmd::Dir(dir) => {
                    if dir.eq("..") {
                        cwd.pop();
                    } else {
                        cwd.push(dir)
                    };
                }
            }
        } else {
            output_buff.push(output)
        }
    });

    let mut memo = HashMap::new();
    let folders = mem
        .keys()
        .map(|s| s.to_string())
        .map(|folder| (folder.clone(), sum_folder(&folder, &mem, &mut memo)))
        .collect::<Vec<_>>();

    folders
}

pub fn sum_folder(
    folder: &str,
    input: &HashMap<String, DirStruct>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if let Some(res) = memo.get(folder) {
        *res
    } else {
        let target = input
            .get(folder)
            .expect("folder does not exist in result map");
        let size = target.flat_size
            + target
                .children
                .iter()
                .map(|c| sum_folder(c, input, memo))
                .sum::<usize>();

        memo.insert(folder.to_string(), size);

        size
    }
}

pub fn parse_cmd(cmd: &str) -> Option<Cmd<String>> {
    let dir = Regex::new(r"\$ cd (.*)").unwrap();
    let ls = Regex::new(r"\$ ls").unwrap();
    let exit = Regex::new(r"\$ exit").unwrap(); // w/e

    if let Some(res) = dir.captures(cmd) {
        let target = res[1].parse::<String>().unwrap();

        Some(Cmd::Dir(target))
    } else if ls.captures(cmd).is_some() {
        Some(Cmd::Ls)
    } else if exit.captures(cmd).is_some() {
        Some(Cmd::Exit)
    } else {
        None
    }
}

pub fn parse_cmd_output(output: &mut Vec<String>, cwd: &str) -> Option<DirStruct> {
    let mut level = DirStruct {
        flat_size: 0,
        children: vec![],
        path: cwd.to_string(),
    };

    for data in output.clone() {
        let data = data.split(' ').collect::<Vec<&str>>();

        match data[0] {
            "dir" => level.children.push(format!("{}/{}", level.path, data[1])),
            ls_output => match ls_output.parse::<usize>() {
                Ok(v) => level.flat_size += v,
                Err(_) => {
                    eprintln!("Failed to parse file size")
                }
            },
        }
    }

    output.clear();

    Some(level)
}

pub enum Cmd<V> {
    Ls,
    Dir(V),
    Exit,
}

#[derive(Debug, Clone)]
pub struct DirStruct {
    flat_size: usize,
    children: Vec<String>,
    path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_07_example_a() {
        let folders = solution("src/demo.txt");

        let res = folders
            .iter()
            .filter(|(_, v)| *v <= 100000)
            .map(|v| v.1)
            .sum::<usize>();

        assert_eq!(95437, res)
    }

    #[test]
    fn d_07_a() {
        let folders = solution("src/input.txt");

        let res = folders
            .iter()
            .filter(|(_, v)| *v <= 100000)
            .map(|v| v.1)
            .sum::<usize>();

        assert_eq!(1454188, res)
    }

    #[test]
    fn d_07_b() {
        let folders = solution("src/input.txt");

        let root_size = folders.iter().map(|v| v.1).max().unwrap();
        let unused_space = 70000000 - root_size;
        let required_space = 30000000;
        let missing_space = required_space - unused_space;

        let res = folders
            .iter()
            .filter(|(_, v)| *v >= missing_space)
            .map(|v| v.1)
            .min()
            .unwrap();

        assert_eq!(4183246, res)
    }
}

// 1315877, too low
// 95437
