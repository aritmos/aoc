use day05::parse;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", pt2(input));
}

fn pt2(input: &str) -> u64 {
    let (seed_info, maps) = parse(input);
    let seed_ranges = seed_info
        .iter()
        .tuples::<(_, _)>()
        .map(|(&s, &l)| s..s + l)
        .collect::<Vec<_>>();

    (0..)
        .map(|l| {
            let s = maps.iter().rev().fold(l, |n, m| {
                m.iter()
                    .find_map(|&range_condition| {
                        // reverse the mapping order
                        let (range_start, dest_start, range_len) = range_condition;

                        // lazy evaluation IS required to not accidentally overflow the subtraction
                        #[allow(clippy::unnecessary_lazy_evaluations)]
                        (range_start <= n && n < range_start + range_len)
                            .then(|| n + dest_start - range_start)
                    })
                    .unwrap_or(n)
            });
            (s, l)
        })
        .find_map(|(s, l)| {
            seed_ranges
                .iter()
                .any(|seed_range| seed_range.contains(&s))
                .then_some(l)
        })
        .unwrap()
}
