mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

use std::fmt;
use std::fs;

pub struct Day {
    pub part_a: fn(&Input) -> Types,
    pub part_b: fn(&Input) -> Types,
}

pub struct Input {
    pub contents: String,
}

pub enum Types {
    String(String),
    Number(usize),
}
impl fmt::Display for Types {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}
impl Input {
    pub fn init(day: u8) -> Self {
        Self {
            contents: fs::read_to_string(format!("input/{day}.txt"))
                .expect("Something went wrong reading the file"),
        }
    }
}
pub fn run(date: u8) {
    let days = &[
        &day01::DAY,
        &day02::DAY,
        &day03::DAY,
        &day04::DAY,
        &day05::DAY,
        &day06::DAY,
        &day07::DAY,
        &day08::DAY,
        &day09::DAY,
        &day10::DAY,
        &day11::DAY,
        &day12::DAY,
    ];
    days.get(date as usize - 1).map_or_else(
        || {
            println!("Day {date} not implemented yet");
        },
        |day| {
            println!("Day {date}:");
            let input = Input::init(date);
            println!("Part A: {}", (day.part_a)(&input));
            println!("Part B: {}", (day.part_b)(&input));
        },
    );
}
