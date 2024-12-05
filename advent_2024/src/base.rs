use crate::file::load_content;

const PATH: &str = "src/inputs/test";

#[allow(unused_variables, unused_mut)]
pub fn main() -> std::io::Result<()> {
    let mut input = load_content(PATH)?;

    Ok(())
}