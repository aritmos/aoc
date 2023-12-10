use day05::parse;

fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", pt1(input));
}

fn pt1(input: &str) -> u64 {
    let (seeds, maps) = parse(input);
    seeds
        .into_iter()
        .map(|s| {
            maps.iter().fold(s, |n, m| {
                m.iter()
                    .find_map(|&range_condition| {
                        let (dest_start, range_start, range_len) = range_condition;

                        // lazy evaluation IS required to not accidentally overflow the subtraction
                        #[allow(clippy::unnecessary_lazy_evaluations)]
                        (range_start <= n && n < range_start + range_len)
                            .then(|| n + dest_start - range_start)
                    })
                    .unwrap_or(n)
            })
        })
        .min()
        .unwrap()
}
