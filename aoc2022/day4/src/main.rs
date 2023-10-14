#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    let input = include_str!("input.txt");
    pt1(input);
    pt2(input);
}

fn pt1(input: &str) {
    let mut total = 0;
    let lines = input.lines();
    for line in lines {
        let mut split = line.split(",").map(|s| {
            s.split("-")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        });
        let v1 = split.next().unwrap();
        let v2 = split.next().unwrap();
        if (v2[0] <= v1[0] && v1[1] <= v2[1]) || (v1[0] <= v2[0] && v2[1] <= v1[1]) {
            total += 1;
        }
    }
    dbg!(total);
}

fn pt2(input: &str) {
    let mut total = 0;
    let lines = input.lines();
    for line in lines {
        let mut split = line.split(",").map(|s| {
            s.split("-")
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        });
        let v1 = split.next().unwrap();
        let v2 = split.next().unwrap();
        let c1 = v1[0] <= v2[0] && v2[0] <= v1[1];
        let c2 = v1[0] <= v2[1] && v2[1] <= v1[1];
        let c3 = v2[0] < v1[0] && v1[1] < v2[1];
        if c1 || c2 || c3 {
            total += 1;
        }
    }
    dbg!(total);
}
