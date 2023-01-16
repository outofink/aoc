use super::{Day, Input, Types};
use json::{self, JsonValue};

pub fn part_a(input: &Input) -> Types {
    let mut i = 0;
    let mut total = 0;
    let chars = input.contents.chars().collect::<Vec<char>>();
    while i < chars.len() {
        match chars.get(i).unwrap() {
            '0'..='9' | '-' => {
                let mut num_string = chars[i].to_string();
                loop {
                    i += 1;
                    match chars.get(i).unwrap() {
                        '0'..='9' => num_string.push(chars[i]),
                        _ => break,
                    }
                }
                total += num_string.parse::<i32>().unwrap();
            }
            _ => i += 1,
        }
    }
    Types::Number(total.try_into().unwrap())
}

fn extract_numbers(input: &JsonValue) -> i32 {
    let mut total = 0;
    if input.is_object() {
        for (_, value) in input.entries() {
            if value.is_string() && value.as_str().unwrap() == "red" {
                return 0;
            } else if value.is_number() {
                total += value.as_i32().unwrap();
            } else if value.is_array() || value.is_object() {
                total += extract_numbers(value);
            }
        }
    } else if input.is_array() {
        for value in input.members() {
            if value.is_number() {
                total += value.as_i32().unwrap();
            } else if value.is_array() || value.is_object() {
                total += extract_numbers(value);
            }
        }
    }
    total
}

pub fn part_b(input: &Input) -> Types {
    let json_input = json::parse(&input.contents).unwrap();
    let mut total = 0;
    for (_, value) in json_input.entries() {
        total += extract_numbers(value);
    }
    Types::Number(total.try_into().unwrap())
}

pub static DAY: Day = Day { part_a, part_b };
