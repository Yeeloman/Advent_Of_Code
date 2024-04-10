use std::io;
mod fnhelp;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    fnhelp::week_1::day_1(&input);

}
