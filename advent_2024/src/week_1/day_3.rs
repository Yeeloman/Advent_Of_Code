use crate::file::load_content;
use regex::Regex;

const PATH: &str = "src/inputs/in_3";

#[allow(unused_variables, unused_mut)]
fn re_mul(exp: &str, re: &Regex) -> Option<i32> {
    let mut res = 0;

    if let Some(captures) = re.captures(exp) {
        let n_1 = captures.get(1)?.as_str().parse::<i32>().ok()?;
        let n_2 = captures.get(2)?.as_str().parse::<i32>().ok()?;
        return Some(n_1 * n_2);
    }
    None
}
#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()>{
    let mut total_product = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input = load_content(PATH)?;


    for line in input.lines() {
        for mat in re.find_iter(line) {
            if let Some(product) = re_mul(mat.as_str(), &re) {
                total_product += product;
            }
            
        }
    }
    println!("total product is: {}", total_product);
    Ok(())
}