fn main() {
    let input = include_str!(".test");
    for line in input.lines() {
        let bytes: Vec<u8> = line.bytes().map(|n| n - 48).collect();
    }
}
