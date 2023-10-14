const INPUT: &str = include_str!(".input");

use std::collections::VecDeque;

fn main() {
    solve(INPUT);
}

enum WorryVal {
    Number(i64),
    Item,
}

struct Monkey {
    items: VecDeque<i64>,     // items held
    inspect_op: char,         // what type of iteration it performs, either '*' or '+'
    inspect_var: WorryVal,    // what value to use in the RHS of operation, either a number or 'old'
    divisor: i64,             // what number to use to test divisibility
    throw_to: (usize, usize), // if test = true -> throw to .0, else .1
    inspections: u32,         // # of items inspected
}

impl Monkey {
    // monkey throws its first item
    // returns tuple of monkey index to throw to, and the item
    fn throw(&mut self, modulo: i64) -> Option<(usize, i64)> {
        // get first item (removing it from inventory),
        // if there are none return None
        let mut item = self.items.pop_front()?;

        // set {var} to the item itself, or a predetermined value
        // `old * {var}
        let inspect_var = match self.inspect_var {
            WorryVal::Number(x) => x,
            WorryVal::Item => item,
        };

        item = match self.inspect_op {
            '*' => item * inspect_var,
            '+' => item + inspect_var,
            x => panic!("Inspect opearation for '{x}' is not implemented"),
        };

        // bound the number
        item %= modulo;

        // pick monkey to throw to
        let monkey_index = if (item % self.divisor) == 0 {
            self.throw_to.0
        } else {
            self.throw_to.1
        };

        // increment inspections
        self.inspections += 1;

        // return index and item,
        // the item movement will be handed by the parent function
        Some((monkey_index, item))
    }

    // - IGNORE -
    // turning input into structs
    fn from_chunk(chunk: &str) -> Monkey {
        let mut lines = chunk.lines().skip(1);
        let items: VecDeque<i64> = lines.next().unwrap()[18..]
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let operation_line = lines.next().unwrap();
        let inspect_op: char = operation_line.chars().nth(23).unwrap();
        let inspect_var: WorryVal = match &operation_line[25..] {
            "old" => WorryVal::Item,
            n => WorryVal::Number(n.parse::<i64>().unwrap()),
        };
        let divisor: i64 = lines.next().unwrap()[21..].parse().unwrap();
        let idx_true: usize = lines.next().unwrap()[29..].parse().unwrap();
        let idx_false: usize = lines.next().unwrap()[30..].parse().unwrap();

        // return monkey
        Monkey {
            items,
            inspect_op,
            inspect_var,
            divisor,
            throw_to: (idx_true, idx_false),
            inspections: 0,
        }
    }
}

struct KeepAway {
    monkeys: Vec<Monkey>,
    modulo: i64,
}

impl KeepAway {
    // simulates a round within the "game"
    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            // in the order given by the input (same order as in the list),
            // each monkey goes through and throws their entire inventory
            for _ in 0..self.monkeys[i].items.len() {
                // if we didn't get None, then move the item into the corresponding monkey
                if let Some((i, x)) = self.monkeys[i].throw(self.modulo) {
                    self.monkeys[i].items.push_back(x);
                }
            }
        }
    }
}

fn solve(input: &str) {
    // instantiate main struct
    let mut keep_away = KeepAway {
        monkeys: Vec::new(),
        modulo: 1,
    };

    // populate struct from input
    for chunk in input.split("\r\n\r\n") {
        let monkey = Monkey::from_chunk(chunk);
        let div = monkey.divisor;

        keep_away.monkeys.push(monkey); // add monkey to list
        keep_away.modulo *= div; // update modulo
    }

    // simulate game
    for _ in 0..10000 {
        keep_away.round();
    }

    println!(
        "{:?}",
        keep_away
            .monkeys
            .iter()
            .map(|m| m.inspections)
            .collect::<Vec<u32>>()
    );

    // for the solution we want the highest 2 inspection numbers multiplied together
    let mut vec = keep_away
        .monkeys
        .iter()
        .map(|monkey| monkey.inspections as u64)
        .collect::<Vec<u64>>();
    vec.sort_unstable();

    println!("{}", vec[vec.len() - 1] * vec[vec.len() - 2]);
}
