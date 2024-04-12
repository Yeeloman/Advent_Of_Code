pub mod week_1 {

    pub fn challenges() {
        day_1();
        day_2();
        day_3();
        day_4();
        day_5();
        day_6();
        day_7();
    }

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};

    fn day_1() -> () {
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

    fn day_2() -> () {
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

    fn day_3() -> () {
        if let Ok(mut file) = File::open("src/inputs/in_3") {
            let content = &mut String::new();
            let mut visited_houses = vec![(0,0)];
            let mut current_house: (i32, i32) = (0, 0);

            let mut directions = HashMap::new();
            directions.insert('<', (-1, 0));
            directions.insert('>', (1, 0));
            directions.insert('^', (0, 1));
            directions.insert('v', (0, -1));

            file.read_to_string( content).unwrap();

            for c in content.chars() {

                //calculate the position of the current house
                if "<>^v".contains(c) {
                    if let Some(pos) = directions.get(&c) {
                        current_house = (
                            current_house.0 + pos.0,
                            current_house.1 + pos.1,
                        )
                    }
                } else {
                    continue;
                }

                //enters this if the current house is new
                if !found(
                    current_house,
                    &visited_houses) {
                        visited_houses.push(current_house);

                }
            }
            println!("Challenge Day 3 Part 1: {}", visited_houses.len());
        }
    }

    //helper function for day 3
    //returns true if found
    fn found(array_to_find: (i32, i32),
            array_to_search: &[(i32, i32)]) -> bool {
        let mut found: bool = false;
        for row in array_to_search {
            if array_to_find == *row {
                found = true
            }
        }
        found
    }

    fn day_4() -> () {}

    fn day_5() -> () {}

    fn day_6() -> () {}

    fn day_7() -> () {}
}
