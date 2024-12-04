use crate::file::load_content;
use regex::Regex;

const PATH: &str = "src/inputs/in_3";

fn re_mul(exp: &str, re: &Regex) -> Option<i32> {
    if let Some(captures) = re.captures(exp) {
        let n_1 = captures.get(1)?.as_str().parse::<i32>().ok()?;
        let n_2 = captures.get(2)?.as_str().parse::<i32>().ok()?;
        return Some(n_1 * n_2);
    }
    None
}

fn loop_re_part_1(line: &str, re: &Regex) -> i32 {
    let mut total = 0;
    for mat in re.find_iter(line) {
        if let Some(prdct) = re_mul(mat.as_str(), &re) {
            total += prdct;
        }
    }
    total
}

fn loop_re_part_2(file: String, re: &Regex) -> i32 {
    let mut total = 0;
    let mut enabled = true;

    for cap in re.captures_iter(&file) {
        if let Some(instr) = cap.get(0).map(|x| x.as_str()) {
            match instr {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => {
                    if enabled {
                        if let (Some(n_1), Some(n_2)) = (cap.get(1), cap.get(2)) {
                            total += n_1.as_str().parse::<i32>().unwrap()
                                * n_2.as_str().parse::<i32>().unwrap();
                        }
                    }
                }
            }
        }
    }
    total
}

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let input = load_content(PATH)?;
    let mut total_1 = 0;
    let mut total_2;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)").unwrap();

    for line in input.clone().lines() {
        total_1 = loop_re_part_1(line, &re);
    }

    total_2 = loop_re_part_2(input, &re);
    println!("Total product part 1 is: {}", total_1);
    println!("Total product part 2 is: {}", total_2);
    Ok(())
}
