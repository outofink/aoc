mod day01;
mod day02;

use std::fs;

pub struct Day {
    pub part_a: fn(&Data) -> i32,
    pub part_b: fn(&Data) -> i32,
}

pub struct Data {
    pub contents: String,
}

impl Data {
    pub fn init(day: u8) -> Data {
        Data {
            contents: fs::read_to_string(format!("input/{}.txt", day))
                .expect("Something went wrong reading the file"),
        }
    }
}
pub fn run(date: u8) {
    let days = &[&day01::DAY, &day02::DAY];
    if let Some(day) = days.get(date as usize - 1) {
        println!("Day {}:", date);
        let data = Data::init(date);
        println!("Part A: {}", (day.part_a)(&data));
        println!("Part B: {}", (day.part_b)(&data));
    } else {
        println!("Day {} not implemented yet", date);
    }
}
