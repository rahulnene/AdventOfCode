use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

const LINES: &str = include_str!("../../problem_inputs_2023/day_21_test.txt");

pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    let now = Instant::now();
    let (maze, start) = Maze::from_str(LINES);
    for i in (2..=64).step_by(2) {
        println!("{i}, {:?}", maze.get_positions_at_distance(start, i).len());
        println!("{:?}", now.elapsed());
    }
    (0, now.elapsed())
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
        // for &neighbor in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        //     let new_pos = (pos.0 + neighbor.0, pos.1 + neighbor.1);
        //     if *self.cells.get(&new_pos).unwrap_or(&false) {
        //         result.push(new_pos);
        //     }
        // }
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .map(|neighbor| (pos.0 + neighbor.0, pos.1 + neighbor.1))
            .iter()
            .filter(|new_pos| *self.cells.get(&new_pos).unwrap_or(&false))
            .for_each(|new_pos| result.push(*new_pos));
        assert!(result.len() <= 4, "Too many neighbors");
        result
    }

    fn get_positions_at_distance(&self, start: Position, distance: isize) -> FxHashSet<Position> {
        let mut visited = FxHashSet::default();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some((pos, d)) = queue.pop_front() {
            if d > distance {
                continue;
            }
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);
            for neighbor in self.get_neighbors(&pos) {
                queue.push_back((neighbor, d + 1));
            }
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
