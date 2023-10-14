#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let input = include_str!("input.txt");
    solve(input); // for pt1 use move_crates_9000,
                  // for pt2 use move_crates_9001
}

#[derive(Debug)]
struct Crates {
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn move_crates_9000(&mut self, n: u32, i: usize, j: usize) {
        for _ in 0..n {
            let cr = self.stacks[i].pop().unwrap();
            self.stacks[j].push(cr);
        }
    }

    fn move_crates_9001(&mut self, n: u32, i: usize, j: usize) {
        let mut stack = vec![];
        for _ in 0..n {
            stack.push(self.stacks[i].pop().unwrap());
        }
        stack.reverse();
        self.stacks[j].append(&mut stack);
    }

    fn top(&self) -> String {
        let mut out = String::new();
        for stack in &self.stacks {
            match stack.len() {
                0 => (),
                n => out += &stack[n - 1].to_string(),
            };
        }
        out
    }
}

fn solve(input: &str) {
    let (crate_input, orders) = input.split_once("\r\n\r\n");

    let mut crate_lines = crate_input.lines().rev();
    let n = crate_lines
        .next()
        .unwrap()
        .trim()
        .split(" ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut crates = Crates {
        stacks: vec![vec![]; n],
    };

    for crate_line in crate_lines {
        for (i, c) in itertools::enumerate(crate_line.chars()) {
            if c.is_alphabetic() {
                let idx = (i - 1) / 4;
                crates.stacks[idx].push(c);
            }
        }
    }

    for order in orders.lines() {
        let words = order.split(" ").collect::<Vec<&str>>();
        let n = words[1].parse::<u32>().unwrap();
        let i = words[3].parse::<usize>().unwrap() - 1;
        let j = words[5].parse::<usize>().unwrap() - 1;

        crates.move_crates_9001(n, i, j);
    }
    // dbg!(&crates);
    println!("{}", crates.top());
}
