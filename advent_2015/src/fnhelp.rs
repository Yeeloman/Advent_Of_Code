pub mod week_1 {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};
    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_1() -> () {
        if let Ok(mut file) = File::open("src/inputs/in_1") {
            let input = &mut String::new();
            let mut output = 0;
            let mut index = 0;
            let mut output_idx = 0;
            let mut first_time = false;

            file.read_to_string(input).unwrap();
            for c in input.chars() {
                output += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                index += 1;
                if output < 0 && !first_time {
                    output_idx = index;
                    first_time = true;
                }
            }
            println!("Challenge Day 1 part 1: {}", output);
            println!("Challenge Day 1 part 2: {}", output_idx);
        }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_2() -> () {
        if let Ok(file) = File::open("src/inputs/in_2") {
            let reader = BufReader::new(file);
            let mut total_paper: u32 = 0;
            let mut total_ribbon: u32 = 0;

            for line in reader.lines() {
                if let Ok(line) = line {
                    let dim: Vec<u32> = line
                        .split('x')
                        .filter_map(|x| x.parse::<u32>().ok())
                        .collect();
                    if dim.len() == 3 {
                        let (l, w, h) = (dim[0], dim[1], dim[2]);
                        let lw = l * w;
                        let wh = w * h;
                        let hl = h * l;
                        let min_side = [lw, wh, hl].iter().min().unwrap().clone();
                        total_paper += 2 * (lw + wh + hl) + min_side;
                        total_ribbon +=
                            2 * [l + w, w + h, h + l].iter().min().unwrap() + (l * w * h);
                    }
                }
            }
            println!("Challenge day 2 Part 1: {}", total_paper);
            println!("Challenge day 2 Part 2: {}", total_ribbon);
        }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_3() -> () {}

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_4() -> () {}

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_5() -> () {}

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_6() -> () {}

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_7() -> () {}
}
