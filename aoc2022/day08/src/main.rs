#![allow(dead_code)]
#![allow(unused_variables)]

const INPUT: &str = include_str!(".input");
const N: usize = 99; // # rows
const M: usize = N; // # cols

fn main() {
    pt2(INPUT);
}

#[derive(Debug, Default, Clone, Copy)]
struct Tree {
    // instantiated height is +=1 of what is given in input,
    // done so we can "set the ground level" at height = 0
    // this way i can exclusively use unsigned ints all throughout
    height: u8,
    visible: bool,
}

struct Forest {
    trees: [[Tree; M]; N],
}

impl Forest {
    fn from_lines(lines: std::str::Lines) -> Forest {
        let mut forest = Forest {
            trees: [[Tree::default(); M]; N],
        };

        let mut row_idx: usize = 0;
        for line in lines {
            let mut nums = line.bytes().map(|n| n - 48);
            for col_idx in 0..N {
                forest.trees[row_idx][col_idx].height = nums.next().unwrap() + 1;
            }
            row_idx += 1;
        }
        forest
    }

    fn pass<X, Y>(&mut self, outer_iter: X, inner_iter: Y, transpose: bool)
    where
        X: Iterator<Item = usize>,
        Y: Iterator<Item = usize> + Clone,
    {
        for x in outer_iter {
            let mut local_max = 0;
            for y in inner_iter.clone() {
                let (i, j) = if !transpose { (x, y) } else { (y, x) };
                if self.trees[i][j].height > local_max {
                    local_max = self.trees[i][j].height;
                    self.trees[i][j].visible = true;
                }
            }
        }
    }

    fn visible(&self) -> u32 {
        let mut total_visible = 0;
        for i in 0..N {
            for j in 0..M {
                if self.trees[i][j].visible {
                    total_visible += 1;
                }
            }
        }
        total_visible
    }

    fn scenic_score(&self, i: usize, j: usize) -> usize {
        let mut scenic = 1;
        let tree_height = self.trees[i][j].height;

        scenic *= match (0..i)
            .rev()
            .find(|&x| self.trees[x][j].height >= tree_height)
        {
            Some(x) => i - x,
            None => i,
        };
        scenic *= match ((i + 1)..N).find(|&x| self.trees[x][j].height >= tree_height) {
            Some(x) => x - i,
            None => (N - 1) - i,
        };
        scenic *= match (0..j)
            .rev()
            .find(|&y| self.trees[i][y].height >= tree_height)
        {
            Some(y) => j - y,
            None => j,
        };
        scenic *= match ((j + 1)..M).find(|&y| self.trees[i][y].height >= tree_height) {
            Some(y) => y - j,
            None => (M - 1) - j,
        };

        scenic
    }

    fn scenic_max(&self) -> usize {
        let mut scenic_max = 1;
        for i in 0..N {
            for j in 0..M {
                scenic_max = std::cmp::max(self.scenic_score(i, j), scenic_max);
            }
        }
        scenic_max
    }
}

fn pt1(input: &str) {
    let mut forest = Forest::from_lines(input.lines());

    forest.pass(0..N, 0..M, false); // pass from west
    forest.pass(0..N, (0..M).rev(), false); // pass from east
    forest.pass(0..M, 0..N, true); // pass from north
    forest.pass(0..M, (0..N).rev(), true); // pass from south

    dbg!(forest.visible());
}

fn pt2(input: &str) {
    let forest = Forest::from_lines(input.lines());

    // dbg!(forest.scenic_score(1, 2));
    dbg!(forest.scenic_max());
}
