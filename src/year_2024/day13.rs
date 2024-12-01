use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2023/day_13.txt");

pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    let now = Instant::now();
    let patterns = LINES
        .split("\r\n\r\n")
        .map(|l| Grid::from_str(l))
        .collect_vec();
    let mut total_verts = 0;
    let mut total_horiz = 0;
    for p in patterns {
        let vert_mirrors: isize = (0..p.bounds.0)
            .filter(|&c| p.check_vertical_reflection(c))
            .sum();
        let hori_mirrors: isize = (0..p.bounds.1)
            .filter(|&r| p.check_horizontal_reflection(r))
            .sum();
        if vert_mirrors > 0 {
            total_verts += vert_mirrors + 1;
        }
        if hori_mirrors > 0 {
            total_horiz += hori_mirrors + 1;
        }
    }

    let ans = total_verts + 100 * total_horiz;
    (ans, now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Content {
    Ash,
    Rock,
}
type Position = (isize, isize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    pattern: FxHashMap<Position, Content>,
    bounds: (isize, isize),
}

impl Grid {
    fn from_str(s: &str) -> Self {
        let mut pattern = FxHashMap::default();
        let mut bounds = (0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;
                let content = match c {
                    '.' => Content::Ash,
                    '#' => Content::Rock,
                    _ => continue,
                };
                pattern.insert((x, y), content);
                bounds.0 = bounds.0.max(x);
                bounds.1 = bounds.1.max(y);
            }
        }
        Self { pattern, bounds }
    }

    fn get_row_vals(&self, row_num: isize) -> Vec<Content> {
        (0..=self.bounds.0)
            .map(|col| self.pattern.get(&(col, row_num)).unwrap())
            .copied()
            .collect_vec()
    }
    fn get_col_vals(&self, col_num: isize) -> Vec<Content> {
        (0..=self.bounds.1)
            .map(|row| self.pattern.get(&(col_num, row)).unwrap())
            .copied()
            .collect_vec()
    }

    fn check_horizontal_reflection(&self, row_num: isize) -> bool {
        let (mut row1, mut row2) = (row_num, row_num + 1);
        loop {
            let row1_vals = self.get_row_vals(row1);
            let row2_vals = self.get_row_vals(row2);
            if row1_vals != row2_vals {
                return false;
            }
            row1 -= 1;
            row2 += 1;
            if row1 < 0 || row2 > self.bounds.1 {
                return true;
            }
        }
    }
    fn check_vertical_reflection(&self, col_num: isize) -> bool {
        let (mut col1, mut col2) = (col_num, col_num + 1);
        loop {
            let col1_vals = self.get_col_vals(col1);
            let col2_vals = self.get_col_vals(col2);
            if col1_vals != col2_vals {
                return false;
            }
            col1 -= 1;
            col2 += 1;
            if col1 < 0 || col2 > self.bounds.0 {
                return true;
            }
        }
    }
}
