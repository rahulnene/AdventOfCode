use rustc_hash::{FxHashMap, FxHashSet};

use std::{
    hash::Hasher,
    time::{Duration, Instant},
};

const LINES: &str = include_str!("../../problem_inputs_2023/day_14_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

const TOTAL_SPINS: usize = 1_0_000_000;

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut platform = Platform::from_str(LINES);
    platform.tilt(Direction::Up);
    let result = platform.calculate_load();
    (result, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut platform = Platform::from_str(LINES);
    let mut platform_to_cycle = FxHashMap::default();
    let mut seen = FxHashSet::default();
    let remaining_spins =
        TOTAL_SPINS - platform.spin(TOTAL_SPINS, &mut seen, &mut platform_to_cycle);
    dbg!(remaining_spins);
    let result = platform.calculate_load();
    (result, now.elapsed())
}

type Position = (isize, isize);
#[derive(Debug, Clone, PartialEq, Eq)]
struct Platform {
    rocks: FxHashMap<Position, RockType>,
    bounds: (usize, usize),
}

impl Platform {
    fn new() -> Self {
        Self {
            rocks: FxHashMap::default(),
            bounds: (0, 0),
        }
    }

    fn calculate_load(&self) -> usize {
        let load = self
            .rocks
            .iter()
            .filter(|(_, rock)| *rock == &RockType::Round)
            .map(|(pos, _)| self.bounds.1 - pos.1 as usize)
            .sum();

        load
    }

    fn spin(
        &mut self,
        times: usize,
        seen: &mut FxHashSet<u64>,
        weights: &mut FxHashMap<u64, usize>,
    ) -> usize {
        let mut step = 0;
        while step < times {
            let hash = self.hash();
            if seen.contains(&hash) {
                step += weights.get(&hash).unwrap();
            } else {
                seen.insert(hash);
                weights.insert(hash, step);
                self.spin_once();
                step += 1;
            }
        }
        step
    }

    fn spin_once(&mut self) {
        self.tilt(Direction::Up);
        self.tilt(Direction::Left);
        self.tilt(Direction::Down);
        self.tilt(Direction::Right);
    }

    fn from_str(s: &str) -> Self {
        let mut platform = Self::new();
        let mut bounds = (0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x as isize, y as isize);
                match c {
                    '#' => {
                        platform.rocks.insert(pos, RockType::Hash);
                    }
                    'O' => {
                        platform.rocks.insert(pos, RockType::Round);
                    }
                    _ => {
                        platform.rocks.insert(pos, RockType::Empty);
                    }
                }
                bounds.0 = x.max(bounds.0);
            }
            bounds.1 = y.max(bounds.1);
        }
        platform.bounds = (bounds.0 + 1, bounds.1 + 1);
        platform
    }

    fn pprint(&self) {
        println!();
        for y in 0..self.bounds.1 {
            for x in 0..self.bounds.0 {
                let pos = (x as isize, y as isize);
                let rock = self.rocks.get(&pos);
                match rock {
                    Some(RockType::Round) => print!("O"),
                    Some(RockType::Hash) => print!("#"),
                    _ => print!("."),
                }
            }
            println!();
        }
    }

    fn hash(&self) -> u64 {
        let mut hasher = rustc_hash::FxHasher::default();
        for (pos, rock) in &self.rocks {
            hasher.write_i8(*rock as i8);
            hasher.write_i8(pos.0 as i8);
            hasher.write_i8(pos.1 as i8);
        }
        hasher.finish()
    }

    fn calculate_final_position(&self, pos: (isize, isize), dir: Direction) -> (isize, isize) {
        let mut new_pos = pos;
        loop {
            let next_pos = match dir {
                Direction::Up => (new_pos.0, new_pos.1 - 1),
                Direction::Down => (new_pos.0, new_pos.1 + 1),
                Direction::Left => (new_pos.0 - 1, new_pos.1),
                Direction::Right => (new_pos.0 + 1, new_pos.1),
            };
            match self.rocks.get(&next_pos) {
                Some(RockType::Empty) => new_pos = next_pos,
                _ => break,
            }
        }
        new_pos
    }
    fn tilt(&mut self, dir: Direction) {
        let mut changed = true;
        while changed {
            changed = self.tilt_once(dir);
        }
    }

    fn tilt_once(&mut self, dir: Direction) -> bool {
        let mut new_platform = self.clone();
        let mut changed = false;
        for (pos, rock) in &self.rocks {
            if rock != &RockType::Round {
                continue;
            }
            let new_pos = match dir {
                Direction::Up => (pos.0, pos.1 - 1),
                Direction::Down => (pos.0, pos.1 + 1),
                Direction::Left => (pos.0 - 1, pos.1),
                Direction::Right => (pos.0 + 1, pos.1),
            };
            let rock_at_new_pos = self.rocks.get(&new_pos);
            if let Some(RockType::Empty) = rock_at_new_pos {
                new_platform.rocks.insert(*pos, RockType::Empty);
                new_platform.rocks.insert(new_pos, *rock);
                changed = true;
            }
        }
        *self = new_platform;
        changed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RockType {
    Round,
    Hash,
    Empty,
}
