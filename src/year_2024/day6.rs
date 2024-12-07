use std::time::{Duration, Instant};

use rayon::prelude::*;
use rustc_hash::FxHashSet;

pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_6_test.txt");
    } else {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_6.txt");
    }
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut map = Map::from_str(lines);
    while map.step_p1() == GuardState::InMap {}

    let ans = map.distinct_positions.len();
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let map = Map::from_str(lines);

    let ans = (0..map.bounds.0)
        .into_par_iter()
        .map(|row_index| {
            (0..map.bounds.1)
                .into_par_iter()
                .filter_map(|col_index| {
                    let mut modified_map = map.clone();
                    modified_map.obstacles.push((row_index, col_index));
                    let mut map_state = GuardState::InMap;
                    while map_state == GuardState::InMap {
                        map_state = modified_map.step_p2();
                    }
                    if map_state == GuardState::Looped {
                        Some(1)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum();
    (ans, now.elapsed())
}

type RowIndex = isize;
type ColIndex = isize;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Map {
    obstacles: Vec<(RowIndex, ColIndex)>,
    current_position: (RowIndex, ColIndex),
    facing: Direction,
    bounds: (RowIndex, ColIndex),
    distinct_positions: FxHashSet<((RowIndex, ColIndex), Direction)>,
}

impl Map {
    fn from_str(lines: &str) -> Self {
        let mut obstacles = Vec::new();
        let mut current_position = (0, 0);
        for (row, line) in lines.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    obstacles.push((row as RowIndex, col as ColIndex));
                } else if c == '^' {
                    current_position = (row as RowIndex, col as ColIndex);
                }
            }
        }
        let bounds = (
            lines.lines().count() as RowIndex,
            lines.lines().next().unwrap().chars().count() as ColIndex,
        );
        let mut distinct_positions = FxHashSet::default();
        distinct_positions.insert((current_position, Direction::Up));
        Self {
            obstacles,
            current_position,
            facing: Direction::Up,
            bounds,
            distinct_positions,
        }
    }

    fn step_p1(&mut self) -> GuardState {
        if let Some(value) = self.move_guard() {
            return value;
        }
        self.distinct_positions
            .insert((self.current_position, Direction::Up));
        return GuardState::InMap;
    }

    fn step_p2(&mut self) -> GuardState {
        if let Some(value) = self.move_guard() {
            return value;
        }
        let looping = !self
            .distinct_positions
            .insert((self.current_position, self.facing));
        if looping {
            return GuardState::Looped;
        } else {
            return GuardState::InMap;
        }
    }

    fn move_guard(&mut self) -> Option<GuardState> {
        let (current_row, current_col) = self.current_position;
        let (next_row, next_col) = self.facing.get_index_in_direction(current_row, current_col);
        if next_row < 0 || next_row >= self.bounds.0 || next_col < 0 || next_col >= self.bounds.1 {
            return Some(GuardState::Exited);
        }
        if self.obstacles.contains(&(next_row, next_col)) {
            self.facing = self.facing.rotate_cw();
        } else {
            self.current_position = (next_row, next_col);
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_cw(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn get_index_in_direction(&self, row_ind: RowIndex, col_ind: ColIndex) -> (RowIndex, ColIndex) {
        match self {
            Direction::Up => (row_ind - 1, col_ind),
            Direction::Down => (row_ind + 1, col_ind),
            Direction::Left => (row_ind, col_ind - 1),
            Direction::Right => (row_ind, col_ind + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuardState {
    InMap,
    Exited,
    Looped,
}
