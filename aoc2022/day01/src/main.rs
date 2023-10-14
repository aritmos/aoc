#![allow(dead_code)]

use itertools::Itertools;

fn main() {
    pt1(include_str!("input.txt"));
    // pt2(include_str!("input.txt"));
}

fn pt1(input: &str) {
    let max = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max();
    println!("{max:?}");
}

fn pt2(input: &str) {
    let groups = input.split("\r\n\r\n");
    let mut max: Vec<u64> = vec![0, 0, 0];
    for group in groups {
        let mut weight: u64 = 0;
        for line in group.lines() {
            weight += line.parse::<u64>().unwrap();
        }
        max.push(weight);
        max.sort();
        max.remove(0);
    }
    let ans: u64 = max.iter().sum();
    println!("{ans:?}");
}

fn test() -> u32 {
    1
}
