const INPUT: &str = include_str!(".input");
fn main() {
    solve(INPUT);
}

struct Device {
    cycle: i32,
    screen: [[char; 40]; 6],
    signal_strenghts: Vec<i32>,
    x: i32,
}

impl Device {
    fn signal_strength(&mut self) {
        if (self.cycle - 20) % 40 == 0 {
            let signal_strength = self.cycle * self.x;
            self.signal_strenghts.push(signal_strength);
        }
    }

    fn noop(&mut self) {
        self.signal_strength();
        self.add_pixel();
        self.cycle += 1;
    }

    fn addx(&mut self, v: i32) {
        self.noop();
        self.noop();
        self.x += v;
    }

    fn add_pixel(&mut self) {
        let row = self.cycle / 40; // sprite's centre pixel position
        let col = (self.cycle - 1) % 40;
        if (self.x - col).abs() <= 1 {
            self.screen[row as usize][col as usize] = '#';
        }
    }

    fn print_screen(&self) {
        for line in self.screen {
            let out = line.iter().collect::<String>();
            println!("{}", out);
        }
    }
}

fn solve(input: &str) {
    let mut device = Device {
        cycle: 1,
        x: 1,
        signal_strenghts: vec![],
        screen: [['.'; 40]; 6],
    };

    for line in input.lines() {
        let mut split = line.split(' ');
        match split.next().unwrap() {
            "noop" => device.noop(),
            "addx" => {
                let v = split.next().unwrap().parse::<i32>().unwrap();
                device.addx(v);
            }
            _ => panic!(),
        }
    }
    // dbg!(device.signal_strenghts.iter().sum::<i32>());
    // dbg!(device.cycle);
    device.print_screen();
}
