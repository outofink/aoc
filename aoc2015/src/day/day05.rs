use super::{Day, Input};

pub fn part_a(input: &Input) -> i32 {
    let mut nice = 0;
    for line in input.contents.lines() {
        let mut vowels = 0;
        let mut double = false;
        if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")
        {
            continue;
        }
        for (i, c) in line.chars().enumerate() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                _ => (),
            }
            if i > 0 && line.chars().nth(i - 1) == Some(c) {
                double = true;
            }
        }
        if vowels >= 3 && double {
            nice += 1;
        }
    }
    nice
}
pub fn part_b(input: &Input) -> i32 {
    let mut nice = 0;
    for line in input.contents.lines() {
        let mut repeat = false;
        let mut sandwich = false;
        for (i, pair) in line.chars().collect::<Vec<char>>().windows(2).enumerate() {
            let str_pair: String = pair.iter().collect();
            if line[(i + 2)..].contains(&str_pair) {
                repeat = true;
                break;
            }
        }
        for threesome in line.chars().collect::<Vec<char>>().windows(3) {
            if threesome[0] == threesome[2] {
                sandwich = true;
                break;
            }
        }
        if repeat && sandwich {
            nice += 1;
        }
    }
    nice
}

pub static DAY: Day = Day { part_a, part_b };
