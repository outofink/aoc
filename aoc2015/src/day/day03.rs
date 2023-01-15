use super::{Day, Input, Types};
use itertools::Itertools;

pub fn part_a(input: &Input) -> Types {
    let mut locations = vec![(0, 0)];
    let mut x = 0;
    let mut y = 0;
    for c in input.contents.chars() {
        match c {
            '^' => y -= 1,
            'v' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => panic!("Invalid character"),
        };
        if !locations.contains(&(x, y)) {
            locations.push((x, y));
        }
    }
    Types::Number(locations.len().try_into().unwrap())
}

pub fn part_b(input: &Input) -> Types {
    let mut locations = vec![(0, 0)];
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    for (c1, c2) in input.contents.chars().tuples() {
        match c1 {
            '^' => y1 -= 1,
            'v' => y1 += 1,
            '<' => x1 -= 1,
            '>' => x1 += 1,
            _ => panic!("Invalid character"),
        };
        match c2 {
            '^' => y2 -= 1,
            'v' => y2 += 1,
            '<' => x2 -= 1,
            '>' => x2 += 1,
            _ => panic!("Invalid character"),
        };
        if !locations.contains(&(x1, y1)) {
            locations.push((x1, y1));
        }
        if !locations.contains(&(x2, y2)) {
            locations.push((x2, y2));
        }
    }
    Types::Number(locations.len().try_into().unwrap())
}

pub static DAY: Day = Day { part_a, part_b };
