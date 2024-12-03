pub mod week_1 {

    pub fn challenges() {
        day_1();
        day_2();
        day_3::part_1();
        day_3::part_2();
        day_4();
        day_5();
        day_6();
        day_7();
    }

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

    mod day_3 {

        use std::collections::HashMap;
        use std::fs::File;
        use std::io::Read;

        struct Directions {
            directions: HashMap<char, (i32, i32)>,
        }
        impl Directions {
            fn new() -> Self {
                let mut directions = HashMap::new();
                directions.insert('<', (-1, 0));
                directions.insert('>', (1, 0));
                directions.insert('^', (0, 1));
                directions.insert('v', (0, -1));

                Directions { directions }
            }
            fn get(&self, c: &char) -> Option<(i32, i32)> {
                self.directions.get(&c).cloned()
            }
        }

        pub fn part_1() -> () {
            if let Ok(mut file) = File::open("src/inputs/in_3") {
                let content = &mut String::new();
                let mut visited_houses = vec![(0,0)];
                let mut current_house: (i32, i32) = (0, 0);


                file.read_to_string( content).unwrap();

                for c in content.chars() {

                    //calculate the position of the current house
                    if "<>^v".contains(c) {
                        current_house = calculate_pos(c, current_house);
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

        pub fn part_2() -> () {
            if let Ok(mut f) = File::open("src/inputs/in_3") {
                let content = &mut String::new();
                let mut visited_houses = vec![(0, 0)];
                let mut santa_turn = true;
                let mut santa_pos = (0,0);
                let mut robot_pos = (0,0);
                f.read_to_string(content).unwrap();

                for c in content.chars() {
                    if "<>^v".contains(c) {
                        if santa_turn {
                            santa_pos = calculate_pos(c, santa_pos);
                            if !found(santa_pos, &visited_houses) {
                                    visited_houses.push(santa_pos);
                            }
                            santa_turn = false;
                        } else {
                            robot_pos = calculate_pos(c, robot_pos);
                            if !found(robot_pos, &visited_houses) {
                                visited_houses.push(robot_pos);
                            }
                            santa_turn = true;
                        }
                    } else {
                        continue;
                    }
                }
                println!("Challenge Day 3 Part 2: {}", visited_houses.len());
            }
        }

        fn calculate_pos(c: char, mut crnt_pos: (i32, i32)) -> (i32, i32) {
            let directions = Directions::new();
            //calculate the position of the current house
                if let Some(pos) = directions.get(&c) {
                    crnt_pos = (
                        crnt_pos.0 + pos.0,
                        crnt_pos.1 + pos.1,
                    )
                }
                crnt_pos
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
    }


    fn day_4() -> () {

        if let Ok(mut file) = File::open("src/inputs/in_4") {
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            input = input.trim_end_matches('\n').to_string();
            let mut num = 1;
            loop {
                let hashed_input = format!("{}{}", input, num);
                let return_val = md5::compute(hashed_input.as_bytes());
                let return_val = format!("{:x}", return_val);
                // to find the answer for part 2 add a 0 below
                if return_val.starts_with("00000") {
                    break;
                }
                num += 1;
            }
            println!("Challenge 4 Part 1/2: {}", num);
        }
    }

    mod hp_day_5 {

        pub fn vowel_finder(line: &str) -> bool {
            let mut vowels_found: u8 = 0;
            let vowels = "aeiou";
            for char in line.chars() {
                if vowels.contains(char) {
                    vowels_found += 1;
                }
            }
            if vowels_found >= 3 {
                return true
            }
            false
        }

        pub fn consecutive_chars(line: &str) -> bool {
            let mut chars = line.chars();
            if let Some(mut prev_char) = chars.next() {
                while let Some(crnt_char) = chars.next() {
                    if prev_char == crnt_char {
                        return true;
                    }
                    prev_char = crnt_char;
                }
            }
            false
        }

        pub fn disallowed_strings(line: &str) -> bool {
            let disallowed = vec!["ab", "cd", "pq", "xy"];
            for s in disallowed {
                if line.contains(s) {
                    return false;
                }
            }
            true
        }

        pub fn pair_of_two(line: &str) -> bool {
            for i in 0..line.len() - 1 {
                let pair = &line[i..i + 2];
                if line[i + 2..].contains(pair) {
                    return true;
                }
            }
            false
        }

        pub fn one_letter_between(line: &str) -> bool {
            for i in 0..line.len() - 2 {
                if line.chars().nth(i) == line.chars().nth(i + 2) {
                    return true
                }
            }
            false
        }
    }

    fn day_5() -> () {
        if let Ok(mut file) = File::open("src/inputs/in_5") {

            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            let mut nice_strings_part_1 = 0;
            let mut nice_strings_part_2 = 0;

            for line in input.lines() {
                if hp_day_5::vowel_finder(line) &&
                    hp_day_5::consecutive_chars(line) &&
                    hp_day_5::disallowed_strings(line) {
                        nice_strings_part_1 += 1;
                }

                if hp_day_5::pair_of_two(line) &&
                    hp_day_5::one_letter_between(line) {
                    nice_strings_part_2 += 1;
                }
            }
            println!("Challenge 5 part 1: {}", nice_strings_part_1);
            println!("Challenge 5 part 2: {}", nice_strings_part_2);
        }
    }

    fn day_6() -> () {}

    fn day_7() -> () {}
}
