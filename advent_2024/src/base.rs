use crate::file;

const PATH: &str = "src/inputs/test";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let mut input = load_content(PATH)?;

    file::print_challenges(0, 0, 0);
    Ok(())
}