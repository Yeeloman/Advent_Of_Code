use crate::file;

const DAY: i64 = 7;
const PATH: &str = "src/inputs/in_7";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let (mut answer_1, mut answer_2) = (0, 0);
    let mut input = file::load_content(PATH)?;

    let calc = process::clean_data(input);

    for (target, nums) in calc {
        answer_1 += part_1::main(target, &nums);
        answer_2 += part_2::main(target, &nums);
    }

    file::print_challenges(DAY, answer_1, answer_2);
    Ok(())
}

mod process {
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn clean_data(input: String) -> HashMap<i64, Vec<i64>> {
        let mut calculation: HashMap<i64, Vec<i64>> = HashMap::new();

        for line in input.lines() {
            let (result, nums) = line.split_once(":").expect("Invalid input");

            let result = result.parse::<i64>().unwrap();
            let nums: Vec<i64> = nums
                .trim()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            calculation.insert(result, nums);
        }
        calculation
    }
}

mod part_1 {
    use itertools::Itertools;

    #[allow(dead_code)]
    pub fn main(target: i64, nums: &Vec<i64>) -> i64 {
        let ops_num = nums.len() - 1;
        let op = vec!['+', '*'];
        let op_comb = (0..ops_num).map(|_| op.clone()).multi_cartesian_product();

        for possible_op in op_comb {
            if eval_exp(&nums, possible_op) as i64 == target {
                return target;
            }
        }
        0
    }

    fn eval_exp(nums: &[i64], operators: Vec<char>) -> i64 {
        let mut res = nums[0];
        for (i, &op) in operators.iter().enumerate() {
            match op {
                '+' => res += nums[i + 1],
                '*' => res *= nums[i + 1],
                _ => panic!("Invalid op"),
            }
        }
        res
    }
}

mod part_2 {
    use itertools::Itertools;

    pub fn main(target: i64, nums: &Vec<i64>) -> i64 {
        let ops_num = nums.len() - 1;
        let op = vec!['+', '*', '|'];
        let op_comb = (0..ops_num).map(|_| op.clone()).multi_cartesian_product();

        for possible_op in op_comb {
            if eval_exp(&nums, possible_op) as i64 == target {
                return target;
            }
        }
        0
    }

    fn eval_exp(nums: &[i64], operators: Vec<char>) -> i64 {
        let mut res = nums[0];
        for (i, &op) in operators.iter().enumerate() {
            match op {
                '+' => res += nums[i + 1],
                '*' => res *= nums[i + 1],
                '|' => {
                    let conc = res.to_string() + &nums[i +1].to_string
                    ();
                    res = conc.parse::<i64>().unwrap();
                },
                _ => panic!("Invalid op"),
            }
        }
        res
    }
}
