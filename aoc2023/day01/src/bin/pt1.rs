fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut num_iter = line.chars().filter_map(|c| char::to_digit(c, 10));
        let first_digit = num_iter.next().unwrap();
        sum += first_digit * 10;
        sum += num_iter.last().unwrap_or(first_digit);
    }
    sum
}
