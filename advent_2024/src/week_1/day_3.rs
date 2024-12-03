use crate::file::load_content;
use regex::Regex;

const PATH: &str = "src/inputs/in_3";

#[allow(unused_variables, unused_mut)]
fn re_mul(exp: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})").unwrap();
    let mut res = 0;

    if let Some(captures) = re.captures(exp) {
        let n_1 = captures.get(1).map(|x| x.as_str()).unwrap().parse::<i32>().unwrap();
        let n_2 = captures.get(2).map(|x| x.as_str()).unwrap().parse::<i32>().unwrap();
        res = n_1 * n_2;

    }
    res
}
#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()>{
    let mut result = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let input = load_content(PATH)?;

    for line in input.lines() {
        for mat in re.find_iter(line) {
            let exp = mat.as_str();
            let num = re_mul(exp);
            result += num;
        }
    }
    println!("{}", result);
    Ok(())
}