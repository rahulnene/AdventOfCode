use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2023/day_21.txt");

pub fn solution() -> ((usize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let (maze, start) = Maze::from_str(LINES);
    let mut current = FxHashSet::default();
    current.insert(start);
    for _ in 1..=64 {
        current = current
            .iter()
            .map(|pos| maze.get_next_positions(pos))
            .flatten()
            .unique()
            .collect();
    }
    (current.len(), now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

type Position = (isize, isize);

#[derive(Debug, Clone)]
struct Maze {
    cells: FxHashMap<Position, bool>,
}

impl Maze {
    fn get_neighbors(&self, pos: &Position) -> Vec<Position> {
        let mut result = Vec::new();
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .map(|neighbor| (pos.0 + neighbor.0, pos.1 + neighbor.1))
            .iter()
            .filter(|new_pos| *self.cells.get(&new_pos).unwrap_or(&false))
            .for_each(|new_pos| result.push(*new_pos));
        assert!(result.len() <= 4, "Too many neighbors");
        result
    }

    fn get_next_positions(&self, start: &Position) -> FxHashSet<Position> {
        let mut visited = FxHashSet::default();
        let neighbors = self.get_neighbors(start);
        for n in &neighbors {
            visited.insert(*n);
        }
        visited
    }

    fn from_str(s: &str) -> (Self, Position) {
        let mut cells = FxHashMap::default();
        let mut s_pos = (0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;
                if c == 'S' {
                    s_pos = (x, y);
                }
                cells.insert((x, y), c != '#');
            }
        }
        (Self { cells }, s_pos)
    }
}
