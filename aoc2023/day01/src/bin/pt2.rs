fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", solver(input));
}

fn solver(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let n = line.len();
        sum += find_digit(line, 0..n) as u32 * 10;
        sum += find_digit(line, (0..n).rev()) as u32;
    }
    sum
}

fn find_digit<I>(s: &str, iter: I) -> u8
where
    I: Iterator<Item = usize>,
{
    const WRITTEN_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let bytes = s.as_bytes();
    for i in iter {
        if bytes[i].is_ascii_digit() {
            return bytes[i] - b'0';
        }

        if let Some(n) = WRITTEN_DIGITS
            .iter()
            .zip(1..)
            .find_map(|(d, j)| s[i..].starts_with(d).then_some(j))
        {
            return n;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find() {
        let s = "threethreetwothree";
        let n = s.len();
        assert_eq!(find_digit(s, 0..n), 3);
        assert_eq!(find_digit(s, (0..n).rev()), 3);
    }
}
