use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");
    let mut seen = HashSet::new();
    let nums = input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    const TARGET: u32 = 2020;
    let n = nums.len();
    for i in 0..(n - 2) {
        seen.insert(nums[i]);
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {}
        }
    }
}
