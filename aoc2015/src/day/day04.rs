use super::{Input, Day};

pub fn part_a(input: &Input) -> i32 {
    let mut i = 0;
    loop {
        let digest = md5::compute(input.contents.clone() + &i.to_string());
        let hex = format!("{digest:x}");
        if hex.starts_with("00000") {
            return i;
        }
        i += 1;
    }
}

pub fn part_b(input: &Input) -> i32 {
    let mut i = 0;
    loop {
        let digest = md5::compute(input.contents.clone() + &i.to_string());
        let hex = format!("{digest:x}");
        if hex.starts_with("000000") {
            return i;
        }
        i += 1;
    }
}

pub static DAY: Day = Day { part_a, part_b };
