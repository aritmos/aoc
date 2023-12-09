use day04::parse_input;

fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", pt1(input));
}

fn pt1(input: &str) -> u32 {
    let input = parse_input(input).unwrap().1;
    input
        .into_iter()
        .map(|(winners, nums)| {
            let count = nums
                .iter()
                .filter(|&n| winners.iter().any(|w| w == n))
                .count();
            match count {
                0 => 0,
                1 => 1,
                x => 2 << (x - 2),
            }
        })
        .sum()
}
