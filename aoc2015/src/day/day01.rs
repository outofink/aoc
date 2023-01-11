use super::{Input, Day};

pub fn part_a(input: &Input) -> i32 {
    let mut floor = 0;
    input.contents.chars().for_each(|c| match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => (),
    });
    floor
}
pub fn part_b(input: &Input) -> i32 {
    let mut floor = 0;
    for (idx, c) in input.contents.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return (idx + 1).try_into().unwrap();
        }
    }
    floor
}

pub static DAY: Day = Day { part_a, part_b };
