use crate::file;

const PATH: &str = "src/inputs/test";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let mut input = file::load_content(PATH)?;

    let (rules, updates) = process_file(&input);
    file::print_challenges(5, 0, 0);
    Ok(())
}

fn process_file<'a>(input: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut empty_line: bool = false;
    let mut rules: Vec<&'a str> = Vec::new();
    let mut updates: Vec<&'a str> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            empty_line = true;
            continue;
        }
        if empty_line {
            updates.push(line);
        } else {
            rules.push(line);
        }
    }
    (rules, updates)
}
