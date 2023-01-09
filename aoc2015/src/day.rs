mod day01;
mod day02;
mod day03;

use std::fs;

pub struct Day {
    pub part_a: fn(&Input) -> i32,
    pub part_b: fn(&Input) -> i32,
}

pub struct Input {
    pub contents: String,
}

impl Input {
    pub fn init(day: u8) -> Input {
        Input {
            contents: fs::read_to_string(format!("input/{}.txt", day))
                .expect("Something went wrong reading the file"),
        }
    }
}
pub fn run(date: u8) {
    let days = &[&day01::DAY, &day02::DAY, &day03::DAY];
    if let Some(day) = days.get(date as usize - 1) {
        println!("Day {}:", date);
        let input = Input::init(date);
        println!("Part A: {}", (day.part_a)(&input));
        println!("Part B: {}", (day.part_b)(&input));
    } else {
        println!("Day {} not implemented yet", date);
    }
}
