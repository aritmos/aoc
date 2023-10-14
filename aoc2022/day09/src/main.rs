#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashSet;

const INPUT: &str = include_str!(".input");

fn main() {
    solve(INPUT);
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Default)]
struct Rope {
    knots: Vec<Point>,
}

impl Rope {
    fn move_head(&mut self, direction: char) {
        let head = &mut self.knots[0];
        match direction {
            'L' => head.x -= 1,
            'R' => head.x += 1,
            'U' => head.y -= 1,
            'D' => head.y += 1,
            _ => panic!("Direction must be L/R/U/P"),
        }
    }

    fn move_knot(&mut self, idx: usize) {
        let prev_knot = self.knots[idx - 1];
        let curr_knot = &mut self.knots[idx];

        let x_diff = prev_knot.x - curr_knot.x;
        let y_diff = prev_knot.y - curr_knot.y;

        // if the previous knot isnt touching the current knot, move the current knot
        if x_diff * x_diff + y_diff * y_diff >= 4 {
            // add (diff / 2) rounded up if +ve or down if -ve
            let x_add = x_diff as f32 / 2.;
            let y_add = y_diff as f32 / 2.;
            curr_knot.x += match x_diff.is_positive() {
                true => x_add.ceil() as i32,
                false => x_add.floor() as i32,
            };
            curr_knot.y += match y_diff.is_positive() {
                true => y_add.ceil() as i32,
                false => y_add.floor() as i32,
            };
        }
    }

    fn move_rope(&mut self, direction: char) {
        self.move_head(direction);
        for idx in 1..self.knots.len() {
            self.move_knot(idx);
        }
    }
}

fn solve(input: &str) {
    const KNOT_NUM: usize = 10;
    let mut squares: HashSet<Point> = HashSet::new();
    let mut rope = Rope {
        knots: vec![Point::default(); KNOT_NUM],
    };

    for line in input.lines() {
        let mut split = line.split(' ');
        let direction = split.next().unwrap().chars().next().unwrap();
        let count = split.next().unwrap().parse::<u8>().unwrap();
        for _ in 0..count {
            rope.move_rope(direction);
            let tail = *rope.knots.last().unwrap();
            squares.insert(tail);
        }
    }
    println!("{}", squares.len());
}
