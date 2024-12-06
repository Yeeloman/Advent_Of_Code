use crate::file;
use regex::Regex;
use std::collections::HashMap;

const PATH: &str = "src/inputs/in_5";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let (mut answer_1, mut answer_2): (i32, i32) = (0, 0);
    let mut input = file::load_content(PATH)?;

    let (rules, updates) = process_file(input);

    for update in updates {
        let (gd_update, mid) = follows_rule(&rules, &update);

        if gd_update {
            answer_1 += mid;
        }
    }

    file::print_challenges(5, answer_1, answer_2);
    Ok(())
}

fn follows_rule(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> (bool, i32) {
    let mut update_dict: HashMap<&i32, usize> = HashMap::new();

    for (idx, page) in update.iter().enumerate() {
        update_dict.insert(page, idx);
    }

    for (bf, af) in rules {
        let before_applies: bool = update_dict.contains_key(bf);
        let after_applies: bool = update_dict.contains_key(af);
        if  before_applies && after_applies && !(update_dict.get(bf).unwrap() < update_dict.get(af).unwrap()) {
            return (false, 0);
        }
    }

    (true, update[update.len() / 2])
}
fn process_file(input: String) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
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
            let update = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
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
