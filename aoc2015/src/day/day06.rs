use super::{Day, Input};

pub enum Operation {
    Off,
    On,
    Toggle,
}
pub struct Instruction {
    op: Operation,
    start: Coordinate,
    end: Coordinate,
}
pub struct Coordinate {
    x: usize,
    y: usize,
}
pub fn process(input: &Input) -> Vec<Instruction> {
    let mut instuctions: Vec<Instruction> = vec![];
    for line in input.contents.lines() {
        let mut op = Operation::Off;
        let mut trimmed = "";
        match line {
            l if l.contains("toggle") => {
                op = Operation::Toggle;
                trimmed = &line[7..];
            }
            l if l.contains("off") => {
                op = Operation::Off;
                trimmed = &line[9..];
            }
            l if l.contains("on") => {
                op = Operation::On;
                trimmed = &line[8..];
            }
            _ => (),
        }
        let end_first = trimmed.find("through").unwrap();
        let begin_second = end_first + "through".len() + 1;
        let first = &trimmed[..end_first];
        let second = &trimmed[begin_second..];

        let first_index = first.find(',').unwrap();
        let first1 = &first[..first_index].parse().unwrap();
        let first2 = &first[first_index + 1..first.len() - 1].parse().unwrap();

        let second_index = second.find(',').unwrap();
        let second1 = &second[..second_index].parse().unwrap();
        let second2 = &second[second_index + 1..second.len()].parse().unwrap();

        instuctions.push(Instruction {
            op,
            start: Coordinate {
                x: *first1,
                y: *first2,
            },
            end: Coordinate {
                x: *second1,
                y: *second2,
            },
        });
    }
    instuctions
}

pub fn part_a(input: &Input) -> i32 {
    let mut lights = vec![[false; 1000]; 1000].into_boxed_slice();
    for instruction in process(input) {
        for row in lights
            .iter_mut()
            .take(instruction.end.x + 1)
            .skip(instruction.start.x)
        {
            for light in row
                .iter_mut()
                .take(instruction.end.y + 1)
                .skip(instruction.start.y)
            {
                match instruction.op {
                    Operation::Toggle => *light = !*light,
                    Operation::On => *light = true,
                    Operation::Off => *light = false,
                }
            }
        }
    }
    let mut total = 0;
    for row in lights.iter() {
        for light in row.iter() {
            if *light {
                total += 1;
            }
        }
    }
    total
}

pub fn part_b(input: &Input) -> i32 {
    let mut lights = vec![[0; 1000]; 1000].into_boxed_slice();
    for instruction in process(input) {
        for row in lights
            .iter_mut()
            .take(instruction.end.x + 1)
            .skip(instruction.start.x)
        {
            for light in row
                .iter_mut()
                .take(instruction.end.y + 1)
                .skip(instruction.start.y)
            {
                match instruction.op {
                    Operation::Toggle => *light += 2,
                    Operation::On => *light += 1,
                    Operation::Off => {
                        if *light != 0 {
                            *light -= 1;
                        }
                    }
                }
            }
        }
    }
    let mut total = 0;
    for row in lights.iter() {
        for light in row.iter() {
            total += light;
        }
    }
    total
}

pub static DAY: Day = Day { part_a, part_b };
