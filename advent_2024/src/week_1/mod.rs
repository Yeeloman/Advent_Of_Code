mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

#[allow(unused_must_use)]
pub fn challenges(done: bool) {
    if !done {
        day_1::main();
        day_2::main();
        day_3::main();
        day_4::main();
        day_5::main();
        day_6::main();
        day_7::main();
    }
}
