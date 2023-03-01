use itertools::Itertools;

use super::{Day, Input, Types};

pub struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}
pub fn process(data: &str) -> Vec<Ingredient> {
    let mut ingredients = Vec::new();
    for line in data.lines() {
        let mut parts = line.split_whitespace();
        let capacity = parts.nth(2).unwrap().trim_end_matches(',').parse().unwrap();
        let durability = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let flavor = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let texture = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        let calories = parts.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        ingredients.push(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        });
    }
    ingredients
}
pub fn sum(counts: &[&isize], ingredients: &Vec<Ingredient>) -> usize {
    let mut total_capacity = 0;
    let mut total_durability = 0;
    let mut total_flavor = 0;
    let mut total_texture = 0;
    for (count, ingredient) in counts.iter().zip(ingredients) {
        total_capacity += *count * ingredient.capacity;
        total_durability += *count * ingredient.durability;
        total_flavor += *count * ingredient.flavor;
        total_texture += *count * ingredient.texture;
    }
    if total_capacity < 0 || total_durability < 0 || total_flavor < 0 || total_texture < 0 {
        0
    } else {
        (total_capacity * total_durability * total_flavor * total_texture)
            .try_into()
            .unwrap()
    }
}
pub fn sum_500_calories(counts: &[&isize], ingredients: &Vec<Ingredient>) -> usize {
    let mut total_capacity = 0;
    let mut total_durability = 0;
    let mut total_flavor = 0;
    let mut total_texture = 0;
    let mut total_calories = 0;
    for (count, ingredient) in counts.iter().zip(ingredients) {
        total_capacity += *count * ingredient.capacity;
        total_durability += *count * ingredient.durability;
        total_flavor += *count * ingredient.flavor;
        total_texture += *count * ingredient.texture;
        total_calories += *count * ingredient.calories;
    }
    if total_calories != 500
        || total_capacity < 0
        || total_durability < 0
        || total_flavor < 0
        || total_texture < 0
    {
        0
    } else {
        (total_capacity * total_durability * total_flavor * total_texture)
            .try_into()
            .unwrap()
    }
}
pub fn part_a(input: &Input) -> Types {
    let ingredients = process(&input.contents);
    let mut max = 0;
    for counts in (0..100).combinations_with_replacement(ingredients.len()) {
        if counts.iter().sum::<isize>() == 100 {
            for permutation in counts.iter().permutations(ingredients.len()) {
                let score = sum(&permutation, &ingredients);
                if score > max {
                    max = score;
                }
            }
        }
    }
    Types::Number(max)
}

pub fn part_b(input: &Input) -> Types {
    let ingredients = process(&input.contents);
    let mut max = 0;
    for counts in (0..100).combinations_with_replacement(ingredients.len()) {
        if counts.iter().sum::<isize>() == 100 {
            for permutation in counts.iter().permutations(ingredients.len()) {
                let score = sum_500_calories(&permutation, &ingredients);
                if score > max {
                    max = score;
                }
            }
        }
    }
    Types::Number(max)
}

pub static DAY: Day = Day { part_a, part_b };
