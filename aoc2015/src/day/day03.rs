use super::{Day, Input};

pub fn part_a(input: &Input) -> i32 {
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
    locations.len() as i32
}

pub fn part_b(input: &Input) -> i32 {
    let mut locations = vec![(0, 0)];
    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    for chars in input.contents.chars().collect::<Vec<char>>().chunks(2) {
        let [c1, c2] = chars else {panic!("Odd number of characters")};
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
    locations.len() as i32
}

pub static DAY: Day = Day { part_a, part_b };