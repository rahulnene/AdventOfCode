use std::time::{Duration, Instant};

use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2022/day_8.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let forest = Forest::from_str(LINES);
    (solve01(&forest), solve02(&forest))
}

fn solve01(forest: &Forest) -> (usize, Duration) {
    let now = Instant::now();
    let ans = forest
        .trees
        .iter()
        .filter(|(pos, _)| forest.ray_cast(pos))
        .count();
    (ans, now.elapsed())
}

fn solve02(forest: &Forest) -> (usize, Duration) {
    let now = Instant::now();
    let ans = forest
        .trees
        .iter()
        .map(|(pos, _)| forest.scenic_score(pos))
        .max();
    (ans.unwrap() as usize, now.elapsed())
}

type Position = (isize, isize);

#[derive(Debug, Clone)]
struct Forest {
    trees: FxHashMap<Position, u8>,
    size: usize,
}

impl Forest {
    fn from_str(input: &str) -> Self {
        let mut trees = FxHashMap::default();
        let mut size = 0;
        for (y, line) in input.lines().enumerate() {
            size = line.len();
            for (x, c) in line.chars().enumerate() {
                trees.insert((x as isize, y as isize), c.to_digit(10).unwrap() as u8);
            }
        }
        Self { trees, size }
    }

    fn trees_seen_above(&self, position: &Position) -> u8 {
        let mut pos = *position;
        pos.1 -= 1;
        let current_height = self.trees.get(position).unwrap();
        let mut counter = 0;
        while pos.1 >= 0 {
            let h = self.trees.get(&pos).unwrap();
            counter += 1;
            if h >= current_height {
                break;
            }
            pos.1 -= 1;
        }

        counter
    }
    fn trees_seen_below(&self, position: &Position) -> u8 {
        let mut pos = *position;
        pos.1 += 1;
        let current_height = self.trees.get(position).unwrap();
        let mut counter = 0;
        while pos.1 < self.size as isize {
            let h = self.trees.get(&pos).unwrap();
            counter += 1;
            if h >= current_height {
                break;
            }
            pos.1 += 1;
        }

        counter
    }
    fn trees_seen_left(&self, position: &Position) -> u8 {
        let mut pos = *position;
        pos.0 -= 1;
        let current_height = self.trees.get(position).unwrap();
        let mut counter = 0;
        while pos.0 >= 0 {
            let h = self.trees.get(&pos).unwrap();
            counter += 1;
            if h >= current_height {
                break;
            }
            pos.0 -= 1;
        }

        counter
    }
    fn trees_seen_right(&self, position: &Position) -> u8 {
        let mut pos = *position;
        pos.0 += 1;
        let current_height = self.trees.get(position).unwrap();
        let mut counter = 0;
        while pos.0 < self.size as isize {
            counter += 1;
            if self.trees.get(&pos).unwrap() >= current_height {
                return counter;
            }
            pos.0 += 1;
        }

        counter
    }

    fn scenic_score(&self, position: &Position) -> u32 {
        self.trees_seen_above(position) as u32
            * self.trees_seen_below(position) as u32
            * self.trees_seen_left(position) as u32
            * self.trees_seen_right(position) as u32
    }

    fn ray_cast(&self, position: &Position) -> bool {
        if position.0 == 0
            || position.0 == self.size as isize - 1
            || position.1 == 0
            || position.1 == self.size as isize - 1
        {
            return true;
        }
        let current_height = self.trees.get(position).unwrap();
        let trees_above = !self
            .trees
            .iter()
            .filter(|(pos, _)| pos.0 == position.0 && pos.1 < position.1)
            .any(|(_, h)| h >= current_height);
        let trees_below = !self
            .trees
            .iter()
            .filter(|(pos, _)| pos.0 == position.0 && pos.1 > position.1)
            .any(|(_, h)| h >= current_height);
        let trees_left = !self
            .trees
            .iter()
            .filter(|(pos, _)| pos.1 == position.1 && pos.0 < position.0)
            .any(|(_, h)| h >= current_height);
        let trees_right = !self
            .trees
            .iter()
            .filter(|(pos, _)| pos.1 == position.1 && pos.0 > position.0)
            .any(|(_, h)| h >= current_height);
        trees_above || trees_below || trees_left || trees_right
    }
}

fn xor(a: bool, b: bool) -> bool {
    (a || b) && !(a && b)
}
