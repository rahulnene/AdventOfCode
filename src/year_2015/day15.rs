use std::time::{Duration, Instant};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2015/day_15.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve(false), solve(true))
}

fn solve(part2: bool) -> (usize, Duration) {
    let now = Instant::now();
    let ingredients = LINES.lines().map(Ingredient::from_str).collect_vec();
    let mut max_score = 0;
    get_permutations_of_n_size(ingredients.len() as u8, 100).for_each(|config| {
        let score = score(&ingredients, &config, part2);
        if score > max_score {
            max_score = score;
        }
    });
    (max_score, now.elapsed())
}

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn from_str(s: &str) -> Self {
        let (_, rest) = s.split_once(':').unwrap();
        let (cap_str, dur_str, flav_str, tex_str, cal_str) =
            rest.split(',').map(str::trim).collect_tuple().unwrap();
        let cap = cap_str.split_once(' ').unwrap().1.parse().unwrap();
        let dur = dur_str.split_once(' ').unwrap().1.parse().unwrap();
        let flav = flav_str.split_once(' ').unwrap().1.parse().unwrap();
        let tex = tex_str.split_once(' ').unwrap().1.parse().unwrap();
        let cal = cal_str.split_once(' ').unwrap().1.parse().unwrap();
        Self {
            capacity: cap,
            durability: dur,
            flavor: flav,
            texture: tex,
            calories: cal,
        }
    }
}

fn score(ingredients: &[Ingredient], config: &[i32], part2: bool) -> usize {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;
    for (i, ingredient) in ingredients.iter().enumerate() {
        let amount = config.get(i).unwrap();
        capacity += ingredient.capacity * amount;
        durability += ingredient.durability * amount;
        flavor += ingredient.flavor * amount;
        texture += ingredient.texture * amount;
        calories += ingredient.calories * amount;
    }
    if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 || (part2 && calories != 500) {
        return 0;
    }
    (capacity * durability * flavor * texture) as usize
}

fn get_permutations_of_n_size(n: u8, sum: i32) -> Box<dyn Iterator<Item = Vec<i32>>> {
    if n == 1 {
        return Box::new(vec![vec![sum]].into_iter());
    }
    Box::new((0..=sum).flat_map(move |i| {
        get_permutations_of_n_size(n - 1, sum - i).map(move |mut v| {
            v.push(i);
            v
        })
    }))
}
