fn main() {
    // dbg!(u32::MAX); // 4294967295
    // dbg!("/".as_bytes()); // 47
    dbg!(parent_dir_path("/a/b".to_owned(), 2));
}

fn parent_dir_path(path: String, cd_back: u8) -> String {
    let bytes = path.as_bytes();
    if bytes.len() != 0 {
        let mut idx = bytes.len();
        let mut n = cd_back;
        while n > 0 {
            idx -= 1;
            if bytes[idx] == 47 {
                n -= 1;
            }
        }
        return std::str::from_utf8(&bytes[0..idx]).unwrap().to_owned();
    }
    path
}
