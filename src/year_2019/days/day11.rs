use std::fmt::{Debug, Formatter};

use fxhash::FxHashMap;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_11.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let now = std::time::Instant::now();
    let mut map = Map::new_from_lines(lines);
    for _ in 0..100 {
        map.step();
    }
    println!("Part 1: {:?}", now.elapsed());
    map.flashes
}

fn solve02(lines: &str) -> usize {
    let now = std::time::Instant::now();
    let mut map = Map::new_from_lines(lines);
    while !map.all_flashed {
        map.step();
    }
    println!("Part 2: {:?}", now.elapsed());
    map.step
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position(usize, usize);
impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Map {
    map: FxHashMap<Position, Octopus>,
    flashes: usize,
    all_flashed: bool,
    step: usize,
}

impl Map {
    fn new_from_lines(lines: &str) -> Self {
        let mut map = FxHashMap::default();
        for (y, line) in lines.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                map.insert(
                    Position::new(x, y),
                    Octopus::new(Position::new(x, y), c.to_digit(10).unwrap() as u8),
                );
            }
        }
        Self {
            map,
            flashes: 0,
            all_flashed: false,
            step: 0,
        }
    }

    fn step(&mut self) {
        for octopus in self.map.values_mut() {
            octopus.update();
        }
        self.flash_all();
        self.finalize_step();
    }

    fn oct_at(&mut self, position: &Position) -> &mut Octopus {
        self.map.get_mut(&position).unwrap()
    }

    fn finalize_step(&mut self) {
        self.step += 1;
        if self.map.values().all(|octopus| octopus.flashed) {
            self.all_flashed = true;
        }
        for octopus in self.map.values_mut() {
            if octopus.flashed {
                octopus.energy = 0;
                octopus.flashed = false;
                self.flashes += 1;
            }
        }
    }

    fn flash_all(&mut self) {
        let flashed: Vec<Position> = self
            .map
            .values_mut()
            .filter(|octopus| octopus.is_energetic())
            .map(|octopus| {
                octopus.flash();
                octopus.position
            })
            .collect();
        self.flash_neighbors(&flashed);
    }

    fn flash_neighbors(&mut self, flashed: &[Position]) {
        if flashed.is_empty() {
            return;
        }
        for flasher in flashed {
            for neighbor in get_neighbors(flasher) {
                let flashed_octopus = self.oct_at(&neighbor);
                flashed_octopus.update();
            }
            self.flash_all();
        }
    }
}

fn get_neighbors(position: &Position) -> Vec<Position> {
    let mut neighbors: Vec<Position> = Vec::new();
    let (row, col) = (position.0 as isize, position.1 as isize);
    for k in row - 1..row + 2 {
        for l in col - 1..col + 2 {
            if k == row && l == col || k < 0 || l < 0 || k > 9 || l > 9 {
                continue;
            }
            neighbors.push(Position::new(k as usize, l as usize));
        }
    }
    neighbors
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Octopus {
    position: Position,
    energy: u8,
    flashed: bool,
}

impl Octopus {
    fn new(position: Position, energy: u8) -> Self {
        Self {
            position,
            energy,
            flashed: false,
        }
    }

    fn update(&mut self) {
        self.energy += 1;
    }

    fn flash(&mut self) {
        if !self.flashed {
            self.energy = 0;
        };
        self.flashed = true;
    }

    fn is_energetic(&self) -> bool {
        self.energy > 9
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = String::new();
        map.push('\n');
        for y in 0..10 {
            for x in 0..10 {
                let octopus = self.map.get(&Position::new(x, y)).unwrap();
                map.push_str(&format!("{octopus:?}"));
            }
            map.push('\n');
        }
        map.push_str(&format!("{}", self.flashes));
        write!(f, "{map}")
    }
}

impl Debug for Octopus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.energy)
    }
}
