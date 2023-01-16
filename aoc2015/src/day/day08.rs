use super::{Day, Input, Types};

pub fn part_a(input: &Input) -> Types {
    let mut total = 0;
    let mut total_raw = 0;
    for line in input.contents.lines() {
        let chars: Vec<char> = line.chars().collect();
        total_raw += line.len();
        total += line.len() - 2;
        let mut i = 0;
        while i < line.len() {
            if chars[i] == '\\' {
                if chars[i + 1] == 'x' {
                    total -= 3;
                    i += 3;
                } else {
                    total -= 1;
                    i += 1;
                }
            }
            i += 1;
        }
    }
    Types::Number(total_raw - total)
}
pub fn part_b(input: &Input) -> Types {
    let mut total = 0;
    let mut total_raw = 0;
    for line in input.contents.lines() {
        let chars: Vec<char> = line.chars().collect();
        total_raw += line.len();
        total += line.len() + 2;
        for char in chars {
            match char {
                '\\' | '\"' => total += 1,
                _ => (),
            }
        }
    }
    Types::Number(total - total_raw)
}

pub static DAY: Day = Day { part_a, part_b };
