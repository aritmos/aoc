use day04::parse_input;

fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", pt1(input));
}

fn pt1(input: &str) -> u32 {
    let input = parse_input(input).unwrap().1;
    let mut counts = vec![1; input.len()];
    input
        .into_iter()
        .enumerate()
        .for_each(|(i, (winners, nums))| {
            let win_number = nums
                .iter()
                .filter(|&n| winners.iter().any(|w| w == n))
                .count();
            let multiplier = counts[i];
            counts[i + 1..][..win_number]
                .iter_mut()
                .for_each(|n| *n += multiplier);
        });
    counts.iter().sum()
}
