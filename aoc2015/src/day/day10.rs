use super::{Day, Input, Types};

fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut count: u8 = 1;
    let mut last = ' ';
    for c in input.chars() {
        if last == ' ' {
            last = c;
        } else if last == c {
            count += 1;
        } else {
            output.push((count + 48) as char);
            output.push(last);
            last = c;
            count = 1;
        }
    }
    output.push((count + 48) as char);
    output.push(last);
    output
}
pub fn part_a(input: &Input) -> Types {
    let mut output = input.contents.clone();
    for _ in 0..40 {
        output = look_and_say(&output);
    }
    Types::Number(output.len())
}
pub fn part_b(input: &Input) -> Types {
    let mut output = input.contents.clone();
    for _ in 0..50 {
        output = look_and_say(&output);
    }
    Types::Number(output.len())
}

pub static DAY: Day = Day { part_a, part_b };
