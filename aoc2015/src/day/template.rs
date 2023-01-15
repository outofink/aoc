use super::{Day, Input, Types};

pub fn part_a(input: &Input) -> Types {
    Types::Number(input.contents.lines().count())
}
pub fn part_b(input: &Input) -> Types {
    Types::Number(input.contents.lines().count())
}

pub static DAY: Day = Day { part_a, part_b };
