use day02::{hand::Hand, parse::parse_input};

fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let parsed_input = parse_input(input).unwrap().1;
    parsed_input
        .into_iter()
        .map(|hands| {
            hands
                .into_iter()
                .fold(Hand::default(), |a, b| a.max(b))
                .power()
        })
        .sum()
}
