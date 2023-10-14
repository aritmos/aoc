#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    pt2(input);
}

fn pt1(input: &str) {
    let mut total: u32 = 0;
    let lines = input.lines();
    for line in lines {
        let bytes = line.as_bytes();
        let n = bytes.len();
        let container1 = &bytes[0..n / 2];
        let container2 = &bytes[n / 2..];

        let repeated = container1.iter().find(|&i| container2.contains(i)).unwrap();

        match *repeated {
            r if r < 97 => total += (r - 38) as u32, // uppercase
            r => total += (r - 96) as u32,           // lowercase
        };
    }
    dbg!(total);
}

fn pt2(input: &str) {
    let rucksacks = input.lines().map(|line| {
        let bytes = line.as_bytes();
        let mut set: HashSet<u8> = HashSet::new();
        for &byte in bytes {
            set.insert(byte);
        }
        set
    });

    let sum = rucksacks
        .tuples()
        .map(|(a, b, c)| a.iter().copied().find(|i| b.contains(i) && c.contains(i)))
        .map(|i| score(i.unwrap()))
        .sum::<u32>();
    dbg!(sum);
}

fn score(i: u8) -> u32 {
    return if i < 97 {
        (i - 38) as u32
    } else {
        (i - 96) as u32
    };
}
