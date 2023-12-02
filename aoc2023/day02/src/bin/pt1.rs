fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", solve(input));
}

fn solve(input: &str) -> u32 {
    (1..)
        .zip(input.lines())
        .map(|(i, line)| {
            // strip the game indicator
            let colon_idx = line.as_bytes().iter().position(|&b| b == b':').unwrap();
            let line = &line[colon_idx + 2..];

            line.split("; ").all(|hand| {
                let (mut r, mut g, mut b) = (0, 0, 0);
                hand.split(", ").for_each(|dice| {
                    let mut dice_iter = dice.split(' ');
                    let count = dice_iter.next().unwrap().parse::<u32>().unwrap();
                    let color = dice_iter.next().unwrap();
                    match color {
                        "red" => r = count,
                        "green" => g = count,
                        "blue" => b = count,
                        _ => unreachable!(),
                    }
                });
                r <= 12 && g <= 13 && b <= 14
            }) as u32
                * i
        })
        .sum()
}
