use super::{Day, Input, Types};

fn increment_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len() - 1;
    loop {
        if chars[i] == 'z' {
            chars[i] = 'a';
            i -= 1;
        } else {
            chars[i] = (chars[i] as u8 + 1) as char;
            break;
        }
    }
    chars.into_iter().collect()
}

fn get_next_password(mut current_string: String) -> String {
    loop {
        current_string = increment_string(&current_string);
        let password_chars: Vec<char> = current_string.chars().collect();
        if password_chars.contains(&'i')
            || password_chars.contains(&'o')
            || password_chars.contains(&'l')
        {
            continue;
        }
        let mut pairs = 0;
        let mut i = 0;
        while i < password_chars.len() - 1 {
            if password_chars[i] == password_chars[i + 1] {
                pairs += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        if pairs < 2 {
            continue;
        }
        i = 0;
        let mut straight = false;
        while i < password_chars.len() - 2 {
            if password_chars[i] as u8 + 1 == password_chars[i + 1] as u8
                && password_chars[i + 1] as u8 + 1 == password_chars[i + 2] as u8
            {
                straight = true;
                break;
            }
            i += 1;
        }
        if straight {
            break;
        }
    }
    current_string
}

pub fn part_a(input: &Input) -> Types {
    Types::String(get_next_password(input.contents.clone()))
}

pub fn part_b(input: &Input) -> Types {
    Types::String(get_next_password(get_next_password(input.contents.clone())))
}

pub static DAY: Day = Day { part_a, part_b };
