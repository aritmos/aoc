use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Cell {
    Start,
    End,
    Square(u8),
}

impl Cell {
    fn elevation(self) -> u8 {
        match self {
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Square(x) => x,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
    visited: HashSet<GridCoord>,
    current: HashSet<GridCoord>,
    num_steps: usize,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let first_line = input.lines().next().unwrap();
        let width = first_line.len();
        let height = input.lines().count();
        let mut data = Vec::new();
        for c in input.chars() {
            let cell = match c {
                'S' => Cell::Start,
                'E' => Cell::End,
                'a'..='z' => Cell::Square(c as u8 - b'a'),
                '\r' | '\n' => continue,
                _ => panic!("Invalid character: <{c}>"),
            };
            data.push(cell);
        }
        Self {
            width,
            height,
            data,
            current: Default::default(),
            visited: Default::default(),
            num_steps: 0,
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn cell(&self, coord: GridCoord) -> Option<&Cell> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    fn walkable_neighbors(&self, coord: GridCoord) -> impl Iterator<Item = GridCoord> + '_ {
        let curr_elev = self.cell(coord).unwrap().elevation();
        let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        deltas.into_iter().filter_map(move |(dx, dy)| {
            Some(GridCoord {
                x: coord.x.checked_add_signed(dx)?,
                y: coord.y.checked_add_signed(dy)?,
            })
            .filter(|&coord| self.in_bounds(coord))
            .filter(|&coord| {
                let other_elev = self.cell(coord).unwrap().elevation();
                other_elev <= curr_elev + 1
            })
        })
    }

    pub fn step(&mut self) -> bool {
        if self.current.is_empty() {
            // find start coordinate
            let mut start_coord: Option<GridCoord> = None;
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord: GridCoord = (x, y).into();
                    if let Cell::Start = self.cell(coord).unwrap() {
                        start_coord = Some(coord);
                        break;
                    }
                }
            }
            let start_coord = start_coord.unwrap();
            self.current.insert(start_coord);
            self.visited.insert(start_coord);
            return false;
        }

        let current = std::mem::take(&mut self.current);
        let mut next = HashSet::new();
        let mut visited = std::mem::take(&mut self.visited);

        self.num_steps += 1;

        for curr in current {
            for ncoord in self.walkable_neighbors(curr) {
                if visited.contains(&ncoord) {
                    // don't visit it again!
                    continue;
                }
                // check if we visited the End node
                let GridCoord { x, y } = ncoord;
                if let Cell::End = self.data[y * self.width + x] {
                    return true;
                }

                visited.insert(ncoord);
                next.insert(ncoord);
            }
        }
        self.current = next;
        self.visited = visited;

        false
    }

    pub fn num_steps(&self) -> usize {
        self.num_steps
    }

    pub fn start_at_all_lowest(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.data[y * self.width + x].elevation() == 0 {
                    self.current.insert((x, y).into());
                }
            }
        }
    }
}
