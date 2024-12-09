use crate::file;

const DAY: i32 = 0;
const PATH: &str = "src/inputs/test";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let (mut answer_1, mut answer_2) = (0, 0);
    let mut input = file::load_content(PATH)?;
    let mut grid = process::clean_data(input);

    file::print_challenges(DAY, answer_1, answer_2);
    Ok(())
}

mod process {
    pub fn clean_data(input: String) -> Vec<Vec<char>> {
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect()
    }
}

mod part_1 {

    #[allow(dead_code)]
    pub fn main() {
        todo!()
    }
}

mod part_2 {

    #[allow(dead_code)]
    pub fn main() {
        todo!()
    }
}
