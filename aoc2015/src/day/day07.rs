use std::collections::HashMap;

use super::{Day, Input, Types};

pub enum Operation {
    And(String, String),
    Or(String, String),
    Not(String),
    LShift(String, String),
    RShift(String, String),
    None(String),
}

pub struct Machine {
    instructions: HashMap<String, Operation>,
    wires: HashMap<String, u16>,
}

impl Machine {
    fn get_output(&mut self, output: &String) -> u16 {
        if let Ok(val) = output.parse::<u16>() {
            return val;
        }
        if let Some(val) = self.wires.get(output) {
            return *val;
        }
        match self.instructions.get(output).unwrap() {
            Operation::None(val) => {
                let val = self.get_output(&val.clone());
                self.wires.insert(output.to_string(), val)
            }
            Operation::And(left, right) => {
                let left = left.clone();
                let right = right.clone();
                let left = self.get_output(&left);
                let right = self.get_output(&right);
                self.wires.insert(output.to_string(), left & right)
            }
            Operation::Or(left, right) => {
                let left = left.clone();
                let right = right.clone();
                let left = self.get_output(&left);
                let right = self.get_output(&right);
                self.wires.insert(output.to_string(), left | right)
            }
            Operation::Not(val) => {
                let val = self.get_output(&val.clone());
                self.wires.insert(output.to_string(), !val)
            }
            Operation::LShift(left, right) => {
                let left = left.clone();
                let right = right.clone();
                let left = self.get_output(&left);
                let right = self.get_output(&right);
                self.wires.insert(output.to_string(), left << right)
            }
            Operation::RShift(left, right) => {
                let left = left.clone();
                let right = right.clone();
                let left = self.get_output(&left);
                let right = self.get_output(&right);
                self.wires.insert(output.to_string(), left >> right)
            }
        };
        *self.wires.get(output).unwrap()
    }
}

fn process(input: &str) -> HashMap<String, Operation> {
    let mut instructions = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        if parts.contains(&"AND") {
            instructions.insert(
                parts[4].to_string(),
                Operation::And(parts[0].to_string(), parts[2].to_string()),
            );
        } else if parts.contains(&"OR") {
            instructions.insert(
                parts[4].to_string(),
                Operation::Or(parts[0].to_string(), parts[2].to_string()),
            );
        } else if parts.contains(&"NOT") {
            instructions.insert(parts[3].to_string(), Operation::Not(parts[1].to_string()));
        } else if parts.contains(&"LSHIFT") {
            instructions.insert(
                parts[4].to_string(),
                Operation::LShift(parts[0].to_string(), parts[2].to_string()),
            );
        } else if parts.contains(&"RSHIFT") {
            instructions.insert(
                parts[4].to_string(),
                Operation::RShift(parts[0].to_string(), parts[2].to_string()),
            );
        } else {
            instructions.insert(parts[2].to_string(), Operation::None(parts[0].to_string()));
        }
    }
    instructions
}

pub fn part_a(input: &Input) -> Types {
    let mut machine = Machine {
        instructions: process(&input.contents),
        wires: HashMap::new(),
    };
    Types::Number(machine.get_output(&"a".to_string()).try_into().unwrap())
}
pub fn part_b(input: &Input) -> Types {
    let mut machine = Machine {
        instructions: process(&input.contents),
        wires: HashMap::new(),
    };
    let a = machine.get_output(&"a".to_string());
    machine.wires = HashMap::new();
    machine.wires.insert("b".to_string(), a);
    Types::Number(machine.get_output(&"a".to_string()).try_into().unwrap())
}

pub static DAY: Day = Day { part_a, part_b };
