mod day;
use std::env;
fn main() {
    let day = env::args()
        .nth(1)
        .expect("No day number passed")
        .parse::<u8>()
        .expect("Day number must be a number");
    day::run(day);
}
