use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::{Day, Input, Types};

type Pairs = HashMap<(String, String), i32>;
type People = HashSet<String>;

fn process(input: &str) -> (Pairs, People) {
    let mut matchings: Pairs = HashMap::new();
    let mut people: People = HashSet::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split_ascii_whitespace().collect();
        let (person1, person2) = (
            split[0].to_string(),
            split[10][..split[10].len() - 1].to_string(),
        );
        let happiness: i32 = match *split.get(2).unwrap() {
            "gain" => split[3].parse::<i32>().unwrap(),
            "lose" => -split[3].parse::<i32>().unwrap(),
            _ => panic!("Invalid happiness value"),
        };
        matchings.insert((person1.clone(), person2.clone()), happiness);
        people.insert(person1);
        people.insert(person2);
    }
    (matchings, people)
}

fn max_happiness(matchings: &Pairs, people: &People) -> i32 {
    let mut max_happiness = i32::MIN;
    for permutation in people.iter().permutations(people.len()) {
        let mut happiness = 0;
        for i in 0..permutation.len() {
            let person1 = &permutation[i].clone();
            let person2 = &permutation[(i + 1) % permutation.len()].clone();
            happiness += matchings[&(person1.clone(), person2.clone())];
            happiness += matchings[&(person2.clone(), person1.clone())];
        }
        if happiness > max_happiness {
            max_happiness = happiness;
        }
    }
    max_happiness
}
pub fn part_a(input: &Input) -> Types {
    let (matchings, people) = process(&input.contents);
    let max_happiness = max_happiness(&matchings, &people);
    Types::Number(max_happiness.try_into().unwrap())
}
pub fn part_b(input: &Input) -> Types {
    let (mut matchings, mut people) = process(&input.contents);
    people.insert("Me".to_string());
    for person in &people {
        matchings.insert(("Me".to_string(), person.clone()), 0);
        matchings.insert((person.clone(), "Me".to_string()), 0);
    }
    let max_happiness = max_happiness(&matchings, &people);
    Types::Number(max_happiness.try_into().unwrap())
}

pub static DAY: Day = Day { part_a, part_b };
