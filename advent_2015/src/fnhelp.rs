pub mod week_1 {
    pub fn day_1(input: &str) -> () {
        let mut output = 0;
        let mut index = 0;
        let mut output_idx = 0;
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
                output_idx = index;
                first_time = true;
            }

        }

        println!("day 1 challenge: {}", output);
        println!("index: {}", output_idx);
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_2(input: &str) -> () {

    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_3(input: &str) -> () {

    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_4(input: &str) -> () {

    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_5(input: &str) -> () {

    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_6(input: &str) -> () {

    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub fn day_7(input: &str) -> () {

    }
}
