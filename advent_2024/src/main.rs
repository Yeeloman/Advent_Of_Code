use week_1::challenges;

pub mod week_1 {

    pub fn challenges() {
        day_1();
        // day_2_approach_A();
        let _ = day_2_approach_B();
    }
    fn day_1() {
        use std::collections::HashMap;
        use std::fs::File;
        use std::io::Read;

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

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    fn day_2_approach_A() {
        use std::fs::File;
        use std::io::Read;

        fn is_increasing(nums: &[i32]) -> bool {
            nums.windows(2).all(|pair| pair[0] < pair[1])
        }

        fn is_decreasing(nums: &[i32]) -> bool {
            nums.windows(2).all(|pair| pair[0] > pair[1])
        }

        // const PATH: &str = "src/inputs/test";
        const PATH: &str = "src/inputs/in_2";
        if let Ok(mut file) = File::open(PATH) {
            let mut input: String = String::new();
            let mut safe_reports = 0;

            file.read_to_string(&mut input).unwrap();

            for line in input.lines() {
                let report: Vec<&str> = line.split_whitespace().collect();

                let report_numbers: Result<Vec<i32>, _> =
                    report.iter().map(|&value| value.parse::<i32>()).collect();
                match report_numbers {
                    Ok(nums) => {
                        let dec = is_decreasing(&nums);
                        let inc = is_increasing(&nums);
                        if dec || inc {
                            let mut safe = true;
                            for (crnt, nxt) in nums.iter().zip(nums.iter().skip(1)) {
                                let diff = (crnt - nxt).abs();
                                if !(1 <= diff && diff <= 3) {
                                    safe = false;
                                    break;
                                }
                            }
                            if safe {
                                safe_reports += 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing numbers: {}", e);
                    }
                }
            }
            println!("the number of safe reports is {}", safe_reports)
        }
    }

    use std::io;
    #[allow(non_snake_case)]
    fn day_2_approach_B() -> io::Result<()> {
        use std::fs::File;
        use std::io::Read;

        const PATH: &str = "src/inputs/in_2";

        fn is_safe_report(report: &[i32]) -> bool {
            if is_valid_sequence(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut modified_report = report.to_vec();
                modified_report.remove(i);
                if is_valid_sequence(&modified_report) {
                    return true;
                }
            }

            false
        }
        fn is_valid_sequence(report: &[i32]) -> bool {
            if report.len() < 2 {
                return true;
            }

            let mut inc: bool = true;
            let mut dec: bool = true;

            for win in report.windows(2) {
                let diff = (win[0] - win[1]).abs();

                if diff < 1 || diff > 3 {
                    return false;
                }
                if win[1] > win[0] {
                    dec = false
                } else if win[1] < win[0] {
                    inc = false
                }
            }
            inc || dec
        }

        let mut file = File::open(PATH)?;
        let mut input = String::new();
        let mut safe_reports_nrml = 0;
        let mut safe_reports_extra = 0;
        file.read_to_string(&mut input).unwrap();

        for line in input.lines() {
            let report_numbers: Result<Vec<i32>, _> =
                line.split_whitespace().map(|x| x.parse::<i32>()).collect();

            match report_numbers {
                Ok(report) => {
                    if is_safe_report(&report) {
                        safe_reports_extra += 1
                    }
                    if is_valid_sequence(&report) {
                        safe_reports_nrml += 1
                    }
                }
                Err(e) => {
                    eprintln!("Failed to parse line '{}': {}", line, e);
                }
            }
        }
        println!("The number of safe reports is {}", safe_reports_nrml);
        println!("The number of safe reports is {}", safe_reports_extra);
        Ok(())
    }
}

fn main() {
    challenges();
}
