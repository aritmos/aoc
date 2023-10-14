#![allow(dead_code)]

use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

fn main() {
    let input = include_str!("input.txt").trim();
    pt2(input);
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn pt1(input: &str) {
    let mut letters = input.bytes();
    let mut buf: VecDeque<u8> = VecDeque::from_iter((&mut letters).take(4));
    for i in 4.. {
        if has_unique_elements(&buf) {
            println!("{i}");
            break;
        }
        buf.pop_front();
        buf.push_back(letters.next().unwrap());
    }
}

fn pt2(input: &str) {
    let mut letters = input.bytes();
    let mut buf: VecDeque<u8> = VecDeque::from_iter((&mut letters).take(14));
    for i in 14.. {
        if has_unique_elements(&buf) {
            println!("{i}");
            break;
        }
        buf.pop_front();
        buf.push_back(letters.next().unwrap());
    }
}
