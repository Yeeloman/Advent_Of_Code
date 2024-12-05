use std::fs::File;
use std::io::{self, BufRead, Read};

pub fn load_content(path: &str) -> io::Result<String> {
    let mut input = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut input)?;
    Ok(input)
}

/// Loads lines from a file at the specified path into a vector of strings.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file.
///
/// # Returns
///
/// * `io::Result<Vec<String>>` - A result containing a vector of strings,
///   where each string is a line from the file, or an I/O error.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened or read.
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

pub fn print_challenges(day: i32, part_1: i32, part_2: i32) {

    println!("day {}:", day);
    println!("\t- part 1: {}", part_1);
    println!("\t- part 2: {}", part_2);
}