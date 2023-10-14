use day12::Grid;

const INPUT: &str = include_str!(".input");

fn main() {
    pt1(INPUT);
}

fn pt1(input: &str) {
    let mut grid: Grid = Grid::parse(input);

    //pt2
    grid.start_at_all_lowest();

    while !grid.step() {}
    println!("{}", grid.num_steps());
}
