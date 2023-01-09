use std::fs;

pub struct Data {
    contents: String,
}

impl Data {
    pub fn init() -> Data {
        Data {
            contents: fs::read_to_string("input/1.txt")
                .expect("Something went wrong reading the file"),
        }
    }
}
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
