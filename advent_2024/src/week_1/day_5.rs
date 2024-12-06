use crate::file;
use regex::Regex;
use std::collections::HashMap;
use topological_sort::TopologicalSort;

const DAY: i32 = 5;
const PATH: &str = "src/inputs/in_5";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let (mut answer_1, mut answer_2): (i32, i32) = (0, 0);
    let mut incorrect_updates: Vec<Vec<i32>> = Vec::new();
    let mut input = file::load_content(PATH)?;

    let (rules, updates) = process_file(input);

    for update in updates {
        let (gd_update, mid) = follows_rule(&rules, &update);

        if gd_update {
            answer_1 += mid;
        } else {
            incorrect_updates.push(update);
        }
    }

    for update in incorrect_updates {
        answer_2 += force_rules(&rules, update);
    }

    file::print_challenges(DAY, answer_1, answer_2);
    Ok(())
}

#[allow(unused_variables, unused_mut)]
fn force_rules(rules: &Vec<(i32, i32)>, update: Vec<i32>) -> i32 {
    let mut sorted_vec: Vec<i32> = Vec::new();
    let mut ts = top_sort(&rules);

    while let Some(node) = ts.pop() {
        if update.contains(&node) {
            sorted_vec.push(node);
        }
    }
    if sorted_vec.is_empty() {
        return 0;
    }
    sorted_vec[sorted_vec.len() / 2]
}

fn top_sort(rules: &Vec<(i32, i32)>) -> TopologicalSort<i32> {
    let mut ts: TopologicalSort<i32> = TopologicalSort::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let mut all_nodes: HashMap<i32, bool> = HashMap::new();

    for &(left, right) in rules {
        ts.add_dependency(left, right);
        *in_degree.entry(right).or_insert(0) += 1;
        all_nodes.insert(left, true);
        all_nodes.insert(right, true);
    }

    let mut zero_in_degree_found = false;
    for node in all_nodes.keys() {
        if !in_degree.contains_key(node) {
            ts.add_dependency(0, *node);
            zero_in_degree_found = true;
            break;
        }
    }

    if !zero_in_degree_found {
        ts.add_dependency(0, 1);
    }

    ts
}
// fn top_sort(rules: &Vec<(i32, i32)>) -> TopologicalSort<i32> {
//     let mut ts: TopologicalSort<i32> = TopologicalSort::new();

//     for &(left, right) in rules {
//         ts.add_dependency(left, right);
//     }
//     // println!("{:?}", ts);
//     ts
// }

fn follows_rule(rules: &Vec<(i32, i32)>, update: &Vec<i32>) -> (bool, i32) {
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
