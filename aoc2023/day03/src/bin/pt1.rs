fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let rows = grid[0].len();

    let mut sum = 0;
    let mut num_digits = Vec::new(); // buffer for building numbers

    for (y, &line) in grid.iter().enumerate() {
        // println!("{sum}");
        #[allow(clippy::needless_range_loop)] // its much cleaner to have the for loop
        for ptr in 0..rows {
            // println!("> {}", std::str::from_utf8(&num_digits).unwrap());
            let b = line[ptr];

            if b.is_ascii_digit() {
                num_digits.push(b);
            } else {
                if num_digits.is_empty() {
                    continue;
                }

                let num_len = num_digits.len();
                let x = ptr - num_len;
                if has_adjacent_symbol(&grid, (x, y), num_len) {
                    sum += unsafe {
                        std::str::from_utf8_unchecked(&num_digits)
                            .parse::<u32>()
                            .unwrap()
                    }
                }
                num_digits.clear();
            }
        }

        if !num_digits.is_empty() {
            let num_len = num_digits.len();
            let x = rows - num_len;
            if has_adjacent_symbol(&grid, (x, y), num_len) {
                sum += unsafe {
                    std::str::from_utf8_unchecked(&num_digits)
                        .parse::<u32>()
                        .unwrap()
                };
            }
        }
        num_digits.clear();
    }
    sum
}

fn has_adjacent_symbol(grid: &[&[u8]], num_coords: (usize, usize), num_len: usize) -> bool {
    let (x, y) = num_coords;
    let (cols, rows) = (grid.len(), grid[0].len()); // matrix size (0 indexed)
    let (top, left, bottom, right) = (y != 0, x != 0, y != rows - 1, x + num_len < cols);

    let is_symbol = |&b: &u8| (b != b'.') & !b.is_ascii_digit();

    let slices = [(top, y as isize - 1), (bottom, y as isize + 1)];
    let singles = [
        (left, (x as isize - 1, y as isize)),
        (right, ((x + num_len) as isize, y as isize)),
        (left & top, (x as isize - 1, y as isize - 1)),
        (left & bottom, (x as isize - 1, y as isize + 1)),
        (right & top, ((x + num_len) as isize, y as isize - 1)),
        (right & bottom, ((x + num_len) as isize, y as isize + 1)),
    ];

    let any_slices = slices
        .into_iter()
        .filter_map(|(ok, y)| ok.then_some(y))
        .any(|y| {
            let slice = &grid[y as usize][x..][..num_len];
            slice.iter().any(is_symbol)
        });

    let any_singles = singles
        .into_iter()
        .filter_map(|(ok, coords)| ok.then_some(coords))
        .any(|(x, y)| {
            let byte = &grid[y as usize][x as usize];
            is_symbol(byte)
        });

    any_slices | any_singles
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../inputs/sample.txt");

    #[test]
    fn check() {
        let grid = INPUT.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
        assert!(has_adjacent_symbol(&grid, (0, 0), 3));
    }
}
