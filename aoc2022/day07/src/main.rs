#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let input = include_str!("input.txt").trim();
    pt1(input);
}

#[derive(Debug, Clone)]
struct Dir {
    subdirs: Vec<Rc<Dir>>,
    size: u32,
}

impl Dir {
    fn total_size(&self) -> u32 {
        self.size + self.subdirs.iter().map(|dir| dir.total_size()).sum::<u32>()
    }

    fn add_dir(&mut self, dir: Rc<Dir>) {
        self.subdirs.push(dir)
    }
}

enum TerminalLine {
    Cd(String),
    Ls(),
    Dir(String),
    File(u32),
}

fn parse_terminal_line(line: &str) -> TerminalLine {
    // match on first 4 bytes of line
    match Some(&line[0..4]).unwrap() {
        "$ cd" => TerminalLine::Cd(line.split(' ').nth(2).unwrap().to_owned()),
        "$ ls" => TerminalLine::Ls(),
        "dir " => TerminalLine::Dir(line.split(' ').nth(1).unwrap().to_owned()),
        _ => TerminalLine::File(line.split(' ').next().unwrap().parse::<u32>().unwrap()),
    }
}

struct CurrData {
    dirs: HashMap<String, Dir>,
    path: String,
    size: u32,
    cd_back: u8,
}

impl CurrData {
    fn execute_cd_back(&mut self) {
        let bytes = self.path.as_bytes();
        if !bytes.is_empty() {
            let mut idx = bytes.len();
            let mut n = self.cd_back;
            while n > 0 {
                idx -= 1;
                if bytes[idx] == 47 {
                    n -= 1;
                }
            }
            self.path = std::str::from_utf8(&bytes[0..idx]).unwrap().to_owned()
        }
        self.cd_back = 0;
    }

    // flush current dir data into its dirs hashmap
    fn flush(&mut self) {
        let dir = Dir {
            subdirs: vec![],
            size: self.size,
        };
        self.dirs.insert(self.path.clone(), dir);
        self.size = 0;
    }
}

fn pt1(input: &str) {
    let mut curr = CurrData {
        dirs: HashMap::new(),
        path: "".to_owned(),
        size: 0,
        cd_back: 0,
    };

    for line in input.lines().skip(1) {
        match parse_terminal_line(line) {
            TerminalLine::Cd(dir) => {
                if dir == *".." {
                    curr.cd_back += 1;
                } else {
                    curr.flush();

                    curr.execute_cd_back();
                    curr.path += "/";
                    curr.path += &dir;
                }
            }
            TerminalLine::File(n) => curr.size += n,
            _ => (),
        }
    }

    // flush last directory
    curr.flush();

    // println!("---");
    // for (path, dir) in curr.dirs.iter() {
    //     println!("{} {}", path, dir.size);
    // }
    // println!("---");

    // reset path to start second loop over lines
    curr.path = "".to_owned();

    for line in input.lines().skip(1) {
        match parse_terminal_line(line) {
            TerminalLine::Cd(dir) => {
                if dir == *".." {
                    curr.cd_back += 1;
                } else {
                    curr.execute_cd_back();
                    curr.path += "/";
                    curr.path += &dir;
                }
            }
            TerminalLine::Dir(dir) => {
                let subdir_path = curr.path.to_owned() + "/" + &dir;
                let subdir = curr.dirs.get(&subdir_path).unwrap().clone();
                curr.dirs
                    .get_mut(&curr.path)
                    .unwrap()
                    .add_dir(Rc::new(subdir));
            }
            _ => (),
        }
    }

    let ans: u32 = curr
        .dirs
        .values()
        .map(|dir| dir.total_size())
        .filter(|&n| n <= 100000)
        .sum();

    dbg!(ans);
}
