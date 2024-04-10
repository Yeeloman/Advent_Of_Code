pub mod week_1 {
    pub fn day_1(input: &str) -> () {
        let mut output = 0;

        for c in input.chars() {
            output += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        }
        println!("day 1 challenge: {}", output)
    }
}
