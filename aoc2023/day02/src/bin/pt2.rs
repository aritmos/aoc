fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // strip the game indicator
            let colon_idx = line.as_bytes().iter().position(|&b| b == b':').unwrap();
            let line = &line[colon_idx + 2..];

            let (mut r, mut g, mut b) = (0, 0, 0);
            line.split("; ").for_each(|hand| {
                hand.split(", ").for_each(|dice| {
                    let mut dice_iter = dice.split(' ');
                    let count = dice_iter.next().unwrap().parse::<u32>().unwrap();
                    let color = dice_iter.next().unwrap();
                    match color {
                        "red" => r = r.max(count),
                        "green" => g = g.max(count),
                        "blue" => b = b.max(count),
                        _ => unreachable!(),
                    }
                });
            });
            r * g * b
        })
        .sum()
}
