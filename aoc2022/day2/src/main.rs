#![allow(dead_code)]

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    pt2(input);
}

fn pt1(input: &str) {
    let total_score: u32 = input
        .lines()
        .map(|l| {
            let (p1, p2) = l.split(" ").tuples().next().unwrap();
            score1(p1, p2)
        })
        .sum();
    println!("{total_score}")
}

fn score1(p1: &str, p2: &str) -> u32 {
    let mut score = 0;
    let p1 = p1.chars().next().unwrap();
    let p2 = p2
        .chars()
        .map(|c| match c {
            'X' => {
                score += 1;
                'A'
            }
            'Y' => {
                score += 2;
                'B'
            }
            'Z' => {
                score += 3;
                'C'
            }
            _ => unreachable!(),
        })
        .next()
        .unwrap();
    match p2 as i64 - p1 as i64 {
        1 | -2 => score += 6,
        0 => score += 3,
        -1 | 2 => (),
        _ => unreachable!(),
    }
    score
}

fn pt2(input: &str) {
    let total_score: u32 = input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let p1 = chars.next().unwrap();
            chars.next();
            let p2 = chars.next().unwrap();
            score2(p1, p2)
        })
        .sum();
    println!("{total_score}")
}

fn score2(p1: char, p2: char) -> u32 {
    let mut score = 0;

    let n1: i32 = match p1 {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => unreachable!(),
    };
    let n2: i32 = match p2 {
        'X' => -1,
        'Y' => 0,
        'Z' => 1,
        _ => unreachable!(),
    };

    // winning points
    score += 3 * (n2 + 1);

    // playing hand points
    let idx = ((n1 + n2 + 3) % 3) as usize;
    score += [1, 2, 3][idx];

    score as u32
}
