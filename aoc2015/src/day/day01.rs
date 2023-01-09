use super::{Data, Day};

pub fn part_a(data: &Data) -> i32 {
    let mut floor = 0;
    data.contents.chars().for_each(|c| match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => (),
    });
    floor
}
pub fn part_b(data: &Data) -> i32 {
    let mut floor = 0;
    for (idx, c) in data.contents.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return idx as i32 + 1;
        }
    }
    floor
}

pub static DAY: Day = Day { part_a, part_b };
