use crate::file;

const DAY: i32 = 0;
const PATH: &str = "src/inputs/test";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let (mut answer_1, mut answer_2) = (0, 0);
    let mut input = load_content(PATH)?;

    file::print_challenges(DAY, answer_1, answer_2);
    Ok(())
}

mod part_1 {
    pub fn main() {
        todo!()
    }
}

mod part_2 {
    pub fn main() {
        todo!()
    }
}
