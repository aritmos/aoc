use day02::parse_input;

fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    let parsed_input = parse_input(input).unwrap().1;
    (1..)
        .zip(parsed_input)
        .map(|(i, hands)| {
            hands
                .iter()
                .all(|h| h.red <= 12 && h.green <= 13 && h.blue <= 14) as u32
                * i
        })
        .sum()
}
