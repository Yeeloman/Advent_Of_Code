use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn main() {
    if let Ok(mut file) = File::open("src/inputs/in_1") {
        let input = &mut String::new();
        let mut first_list: Vec<i32> = Vec::new();
        let mut second_list: Vec<i32> = Vec::new();
        let mut output_list: Vec<i32> = Vec::new();
        let mut total = 0;
        let mut total_2 = 0;
        let mut dict: HashMap<i32, i32> = HashMap::new();

        file.read_to_string(input).unwrap();
        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let (Some(first), Some(second)) = (parts.get(0), parts.get(1)) {
                let num_1 = first.parse().unwrap_or(0);
                let num_2 = second.parse().unwrap_or(0);
                first_list.push(num_1);
                second_list.push(num_2);
            }
        }
        first_list.sort();
        second_list.sort();

        for (a, b) in first_list.iter().zip(second_list.iter()) {
            output_list.push((a - b).abs())
        }
        for num in output_list {
            total += num;
        }
        println!("the total is {}", total);

        for num in second_list {
            if !dict.contains_key(&num) {
                dict.insert(num, 1);
            } else {
                if let Some(value) = dict.get_mut(&num) {
                    *value += 1;
                }
            }
        }
        for num in first_list {
            match dict.get(&num) {
                Some(value) => total_2 += num * value,
                None => total_2 += 0,
            }
        }

        println!("{}", total_2)
    }
}
