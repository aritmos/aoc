use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/input.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let rows = grid[0].len();

    let mut num_digits = Vec::new(); // buffer for building numbers
    let mut symbols = HashMap::<(usize, usize), Vec<u32>>::new();

    fn add_symbols(
        symbols: &mut HashMap<(usize, usize), Vec<u32>>,
        grid: &[&[u8]],
        coords: (usize, usize),
        num_digits: &mut Vec<u8>,
    ) {
        let num_len = num_digits.len();
        let num = unsafe {
            std::str::from_utf8_unchecked(num_digits)
                .parse::<u32>()
                .unwrap()
        };

        num_digits.clear();

        for coords in adjacent_symbols(grid, coords, num_len) {
            symbols.entry(coords).or_default().push(num)
        }
    }

    for (y, &line) in grid.iter().enumerate() {
        #[allow(clippy::needless_range_loop)] // its much cleaner to have the for loop
        for ptr in 0..rows {
            let b = line[ptr];

            if b.is_ascii_digit() {
                num_digits.push(b);
            } else {
                if num_digits.is_empty() {
                    continue;
                }

                let num_len = num_digits.len();
                let x = ptr - num_len;
                add_symbols(&mut symbols, &grid, (x, y), &mut num_digits);
            }
        }

        if !num_digits.is_empty() {
            let num_len = num_digits.len();
            let x = rows - num_len;
            add_symbols(&mut symbols, &grid, (x, y), &mut num_digits);
        }
    }

    symbols
        .values()
        .filter(|&v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

fn adjacent_symbols(
    grid: &[&[u8]],
    num_coords: (usize, usize),
    num_len: usize,
) -> Vec<(usize, usize)> {
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

    let mut symbols = Vec::new();

    symbols.extend(
        slices
            .into_iter()
            .filter_map(|(ok, y)| ok.then_some(y))
            .flat_map(|y| {
                let slice = &grid[y as usize][x..][..num_len];
                slice
                    .iter()
                    .enumerate()
                    .filter_map(move |(i, b)| is_symbol(b).then_some((x + i, y as usize)))
            }),
    );

    symbols.extend(
        singles
            .into_iter()
            .filter_map(|(ok, coords)| ok.then_some(coords))
            .filter_map(|(x, y)| {
                let byte = &grid[y as usize][x as usize];
                is_symbol(byte).then_some((x as usize, y as usize))
            }),
    );

    symbols
}
