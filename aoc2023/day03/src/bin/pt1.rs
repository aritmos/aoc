fn main() {
    let input = include_str!("../../inputs/sample.txt");
    println!("{}", solve(input))
}

fn solve(input: &str) -> u32 {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
    let mut sum = 0;
    let mut num_digits = Vec::new(); // buffer for building numbers
    for (y, &line) in grid.iter().enumerate() {
        for ptr in 0..rows {
            let b = line[ptr];
            if b.is_ascii_digit() {
                num_digits.push(b);
            } else {
                if num_digits.is_empty() {
                    continue;
                }

                let num_len = num_digits.len();
                let x = ptr - num_len + 1;
                if has_adjacent_symbol(&grid, x, y, num_len) {
                    sum += unsafe {
                        std::str::from_utf8_unchecked(&num_digits)
                            .parse::<u32>()
                            .unwrap()
                    };
                }
                num_digits.clear();
            }
        }
        if !num_digits.is_empty() {
            let num_len = num_digits.len();
            let rows = grid[0].len();
            let x = rows - num_len;
            if has_adjacent_symbol(&grid, x, y, num_len) {
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

fn has_adjacent_symbol(grid: &[&[u8]], x: usize, y: usize, len: usize) -> bool {
    let (cols, rows) = (grid.len(), grid[0].len()); // matrix size (0 indexed)
    // top row
    if 
}

fn coord_is_ok(x: usize, y: usize, rows: usize, cols: usize) -> bool {
    0 <= x && x < rows && 0 <= y && y < cols
}
