use super::{Day, Input, Types};

pub fn part_a(input: &Input) -> Types {
    let mut floor: i32 = 0;
    input.contents.chars().for_each(|c| match c {
        '(' => floor += 1,
        ')' => floor -= 1,
        _ => (),
    });
    Types::Number(floor.try_into().unwrap())
}
pub fn part_b(input: &Input) -> Types {
    let mut floor: i32 = 0;
    for (idx, c) in input.contents.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return Types::Number((idx + 1).try_into().unwrap());
        }
    }
    Types::Number(floor.try_into().unwrap())
}

pub static DAY: Day = Day { part_a, part_b };
