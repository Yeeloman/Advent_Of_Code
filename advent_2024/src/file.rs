use std::fs::File;
use std::io::{self, BufRead, Read};

pub fn load_content(path: &str) -> io::Result<String> {
    let mut input = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut input)?;
    Ok(input)
}

#[allow(dead_code)]
pub fn load_lines(path: &str) -> io::Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}