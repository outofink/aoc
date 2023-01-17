use super::{Day, Input, Types};

const SECONDS: usize = 2503;

struct Reindeer {
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

fn process(input: &str) -> Vec<Reindeer> {
    let mut reindeer = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let speed = parts.nth(3).unwrap().parse().unwrap();
        let fly_time = parts.nth(2).unwrap().parse().unwrap();
        let rest_time = parts.nth(6).unwrap().parse().unwrap();
        reindeer.push(Reindeer {
            speed,
            fly_time,
            rest_time,
        });
    }
    reindeer
}
pub fn part_a(input: &Input) -> Types {
    let reindeers = process(&input.contents);
    let mut distances = vec![0usize; reindeers.len()];
    for sec in 0..SECONDS {
        for (i, reindeer) in reindeers.iter().enumerate() {
            if sec % (reindeer.fly_time + reindeer.rest_time) < reindeer.fly_time {
                distances[i] += reindeer.speed;
            }
        }
    }
    Types::Number(*distances.iter().max().unwrap())
}
pub fn part_b(input: &Input) -> Types {
    let reindeers = process(&input.contents);
    let mut points = vec![0usize; reindeers.len()];
    let mut distances = vec![0usize; reindeers.len()];
    for sec in 0..SECONDS {
        for (i, reindeer) in reindeers.iter().enumerate() {
            if sec % (reindeer.fly_time + reindeer.rest_time) < reindeer.fly_time {
                distances[i] += reindeer.speed;
            }
        }
        let max = *distances.iter().max().unwrap();
        for i in 0..reindeers.len() {
            if distances[i] == max {
                points[i] += 1;
            }
        }
    }
    Types::Number(*points.iter().max().unwrap())
}

pub static DAY: Day = Day { part_a, part_b };
