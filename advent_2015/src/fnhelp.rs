pub mod week_1 {
    pub fn day_1(input: &str) -> () {
        let mut output = 0;
        let mut index = 0;
        let mut output_idex = 0;
        let mut first_time = false;

        for c in input.chars() {
            output += match c {
                '(' => 1,
                ')' => -1,
                _ => {
                    0
                },
            };
            index += 1;
            if output < 0 && !first_time {
                output_idex = index;
                first_time = true;
            }

        }

        println!("day 1 challenge: {}", output);
        println!("index: {}", output_idex);
    }
}
