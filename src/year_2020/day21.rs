use std::time::{Duration, Instant};

use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2020/day_21.txt");

pub fn solution() -> ((usize, Duration), (String, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let map = parse_allergen();
    let information = LINES
        .lines()
        .map(|line| {
            let mut parts = line.split(" (contains ");
            let ingredients = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let allergens = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect::<String>())
                .collect::<Vec<String>>();
            AllergyInformation {
                ingredients,
                allergens,
            }
        })
        .collect::<Vec<AllergyInformation>>();

    let unsafe_ingredients = map
        .values()
        .flat_map(|v| v.iter())
        .collect::<Vec<&String>>();

    let safe_ingredients = information
        .iter()
        .flat_map(|info| info.ingredients.iter())
        .filter(|i| !unsafe_ingredients.contains(i))
        .count();

    (safe_ingredients, now.elapsed())
}

fn solve02() -> (String, Duration) {
    let now = Instant::now();

    let mut map = parse_allergen();
    let mut known_allergens: Vec<(String, String)> = Vec::new();
    for (allergen, ingredients) in map.iter() {
        if ingredients.len() == 1 {
            known_allergens.push((allergen.clone(), ingredients[0].clone()));
        }
    }
    while known_allergens.len() < map.len() {
        for (allergen, ingredients) in map.iter_mut() {
            for (known_allergen, known_ingredient) in known_allergens.iter() {
                if allergen != known_allergen {
                    ingredients.retain(|i| i != known_ingredient);
                }
            }
            if ingredients.len() == 1 {
                known_allergens.push((allergen.clone(), ingredients[0].clone()));
            }
        }
    }
    known_allergens.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    known_allergens.dedup();
    dbg!(known_allergens.clone());
    let ans = known_allergens
        .iter()
        .map(|(_, ingredient)| ingredient.clone())
        .collect::<Vec<String>>()
        .join(",");
    (ans, now.elapsed())
}

fn parse_allergen() -> FxHashMap<String, Vec<String>> {
    let mut allergen_map = FxHashMap::default();
    for food in LINES.lines() {
        let mut parts = food.split(" (contains ");
        let ingredients = parts
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let allergens = parts
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect::<String>())
            .collect::<Vec<String>>();
        for allergen in allergens {
            if allergen_map.contains_key(&allergen) {
                let allergen_ingredients: &mut Vec<String> =
                    allergen_map.get_mut(&allergen).unwrap();
                allergen_ingredients.retain(|i| ingredients.contains(i));
            } else {
                allergen_map.insert(allergen, ingredients.clone());
            }
        }
    }
    allergen_map
}

#[derive(Debug, Clone)]
struct AllergyInformation {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}
