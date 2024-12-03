use std::fs::File;
use std::io::{self, Read};

pub fn load_content(path: &str) -> io::Result<String> {
    // const PATH: &str = "src/input/in_3";
    let mut input = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut input)?;
    Ok(input)
}