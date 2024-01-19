use std::time::{Duration, Instant};

use fxhash::FxHashMap;
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2023/day_11_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut uni = Universe::new(lines);
    uni.expand();
    dbg!(&uni.get_sum_all_distances());
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Universe {
    stars: FxHashMap<(usize, usize), Contents>,
    expansion_rows: Vec<usize>,
    expansion_cols: Vec<usize>,
}

impl Universe {
    fn new(s: &str) -> Self {
        let mut starts = FxHashMap::default();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                starts.insert(
                    (x, y),
                    match c {
                        '#' => Contents::Star,
                        '.' => Contents::Empty,
                        _ => panic!("Invalid input"),
                    },
                );
            }
        }
        Self {
            stars: starts,
            expansion_rows: Vec::new(),
            expansion_cols: Vec::new(),
        }
    }
    fn expand(&mut self) {
        let max_x = *self.stars.keys().map(|(x, _)| x).max().unwrap();
        let max_y = *self.stars.keys().map(|(_, y)| y).max().unwrap();
        let bounds = (max_x, max_y);
        for x in 0..=max_x {
            let col = (0..=max_y)
                .map(|y| self.stars.get(&(x, y)).unwrap_or(&Contents::Empty))
                .collect::<Vec<_>>();
            if col.iter().all(|c| c == &&Contents::Empty) {
                self.expansion_cols.push(x);
            }
        }
        for y in 0..=max_y {
            let row = (0..=max_x)
                .map(|x| self.stars.get(&(x, y)).unwrap_or(&Contents::Empty))
                .collect::<Vec<_>>();
            if row.iter().all(|c| c == &&Contents::Empty) {
                self.expansion_rows.push(y);
            }
        }
        self.expansion_rows.sort_unstable();
        self.expansion_cols.sort_unstable();
    }

    fn get_sum_all_distances(&self) -> usize {
        let mut total = 0;
        for (s1, s2) in self
            .stars
            .iter()
            .filter(|s| s.1 == &Contents::Star)
            .tuple_combinations()
        {
            let (x1, y1) = *s1.0;
            let (x2, y2) = *s2.0;
            let rows_in_between = self
                .expansion_rows
                .iter()
                .filter(|r| (x1 < **r && **r < x2) || (x2 < **r && **r < x1))
                .count();
            let cols_in_between = self
                .expansion_rows
                .iter()
                .filter(|r| (y1 < **r && **r < y2) || (y2 < **r && **r < y1))
                .count();
            let dist = x1.abs_diff(x2) + y1.abs_diff(y2);
            println!("Distance between stars at ({x1},{y1}) and ({x2},{y2}) is {dist} as well as {rows_in_between} rows and {cols_in_between} cols");
            total += dist + rows_in_between + cols_in_between;
        }
        total as usize
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Contents {
    Star,
    Empty,
}
