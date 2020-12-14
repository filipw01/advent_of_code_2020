use std::fs::read_to_string;

pub fn run() -> usize {
    let file_content = read_to_string("day16/data.txt").unwrap();
    let mut lines: Vec<_> = file_content.split("\n").collect();
    lines.pop();
    lines.len()
}
