use crate::file;
use regex::Regex;

const PATH: &str = "src/inputs/test";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let mut input = file::load_content(PATH)?;

    let (rules, updates) = process_file(&input);

    file::print_challenges(5, 0, 0);
    Ok(())
}

fn process_file<'a>(input: &'a str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut empty_line: bool = false;
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let re: Regex = Regex::new(r"(\d+)\|(\d+)").unwrap();

    for line in input.lines() {
        if line.trim().is_empty() {
            empty_line = true;
            continue;
        }
        if empty_line {
            let nums = line.split(",").map(|x| x.parse::<i32>().unwrap());
            let mut update = Vec::new();
            for n in nums {
                update.push(n);
            }
            updates.push(update);
            // println!("{:?}", updates)
        } else {
            if let Some(cap) = re.captures(line) {
                let n_1 = cap.get(1).unwrap().as_str().parse::<i32>().ok().unwrap();
                let n_2 = cap.get(2).unwrap().as_str().parse::<i32>().ok().unwrap();
                // println!("{:?}", (n_1, n_2));
                rules.push((n_1, n_2));
            }
        }
    }
    (rules, updates)
}
