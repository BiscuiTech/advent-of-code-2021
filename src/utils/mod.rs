use std::fs;

pub fn read_file(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    contents
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
}
