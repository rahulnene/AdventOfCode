use std::time::{Instant, Duration};

use fxhash::FxHashMap;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2023/day_3.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    // let mut grid = FxHashMap::default();
    // for (row, line) in lines.lines().enumerate() {
    //     for (col, c) in line.chars().enumerate() {
    //         grid.insert(match c {
    //             '.' => Contents::Empty,
    //             '#' => Contents::,
    //             _ => panic!("Invalid character at row {}, col {}", row, col),
    //         }, (row,col));
    //     }
    // }
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Contents {
    Empty,
    Symbol,
    Part(usize)
}