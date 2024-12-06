use crate::file;
use regex::Regex;

const DAY: i32 = 5;
const PATH: &str = "src/inputs/in_5";

pub fn main() -> std::io::Result<()> {
    let (mut answer_1, mut answer_2): (i32, i32) = (0, 0);
    let mut incorrect_updates: Vec<Vec<i32>> = Vec::new();
    let input = file::load_content(PATH)?;

    let (rules, updates) = process_file(input);

    for update in updates {
        let (gd_update, mid) = part_1::main(&rules, &update);

        if gd_update {
            answer_1 += mid;
        } else {
            incorrect_updates.push(update);
        }
    }

    for mut update in incorrect_updates {
        answer_2 += part_2::main(&mut update, &rules);
    }

    file::print_challenges(DAY, answer_1, answer_2);
    Ok(())
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
        } else {
            if let Some(cap) = re.captures(line) {
                let n_1 = cap.get(1).unwrap().as_str().parse::<i32>().ok().unwrap();
                let n_2 = cap.get(2).unwrap().as_str().parse::<i32>().ok().unwrap();
                rules.push((n_1, n_2));
            }
        }
    }
    (rules, updates)
}

mod part_1 {
    use std::collections::HashMap;

    pub fn main(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> (bool, i32) {
        let mut update_dict: HashMap<&i32, usize> = HashMap::new();

        for (idx, page) in update.iter().enumerate() {
            update_dict.insert(page, idx);
        }

        for (bf, af) in rules {
            let before_applies: bool = update_dict.contains_key(bf);
            let after_applies: bool = update_dict.contains_key(af);
            if before_applies
                && after_applies
                && !(update_dict.get(bf).unwrap() < update_dict.get(af).unwrap())
            {
                return (false, 0);
            }
        }

        (true, update[update.len() / 2])
    }
}

mod part_2 {

    use std::cmp::Ordering;
    use std::collections::{HashMap, HashSet};
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rules {
        before: HashSet<i32>,
        after: HashSet<i32>,
    }

    fn rules_set_const(rules: &Vec<(i32, i32)>) -> HashMap<i32, Rules> {
        let mut rules_set: HashMap<i32, Rules> = HashMap::new();

        for (left, right) in rules {
            rules_set
                .entry(*right)
                .or_insert_with(|| Rules {
                    before: HashSet::new(),
                    after: HashSet::new(),
                })
                .before
                .insert(*left);
            rules_set
                .entry(*left)
                .or_insert_with(|| Rules {
                    before: HashSet::new(),
                    after: HashSet::new(),
                })
                .after
                .insert(*right);
        }
        rules_set
    }

    pub fn main(update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) -> i32 {
        let rules_set = rules_set_const(rules);
        update.sort_by(|a, b| {
            if let Some(rules_a) = rules_set.get(a) {
                if rules_a.before.contains(b) {
                    return Ordering::Less;
                }
                if rules_a.after.contains(b) {
                    return Ordering::Greater;
                }
            }
            if let Some(rules_b) = rules_set.get(b) {
                if rules_b.before.contains(a) {
                    return Ordering::Greater;
                }
                if rules_b.after.contains(a) {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });

        update[update.len() / 2]
    }
}
