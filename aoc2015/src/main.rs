mod day;
use std::env;
fn main() {
    let day = env::args().nth(1).unwrap().parse().unwrap();
    day::run(day);
}
