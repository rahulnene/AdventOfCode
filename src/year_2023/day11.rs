use grid::Grid;
use itertools::Itertools;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2023/day_11_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut galaxy = Galaxy {
        map: Grid::new(0, 0),
        empty_rows: Vec::new(),
        empty_cols: Vec::new(),
    };
    for line in LINES.lines() {
        galaxy.map.push_row(
            line.chars()
                .map(|ch| match ch {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Invalid character in input"),
                })
                .collect(),
        );
    }
    galaxy.populate_empties();
    for row_num in galaxy.empty_rows.iter() {
        galaxy
            .map
            .insert_row(*row_num, vec![false; galaxy.map.cols()]);
    }
    for col_num in galaxy.empty_cols.iter() {
        galaxy
            .map
            .insert_col(*col_num, vec![false; galaxy.map.rows()]);
    }
    let mut star_locs = Vec::new();
    for r in 0..galaxy.map.rows() {
        for c in 0..galaxy.map.cols() {
            if *galaxy.map.get(r, c).unwrap() {
                star_locs.push((r, c));
            }
        }
    }
    dbg!(&star_locs);
    dbg!(galaxy.empty_cols, galaxy.empty_rows);
    let ans: usize = star_locs
        .iter()
        .combinations(2)
        .map(|v| v[0].0.abs_diff(v[1].0) + v[0].1.abs_diff(v[1].1))
        .sum();
    dbg!(ans);
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

type Position = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Galaxy {
    map: Grid<bool>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Galaxy {
    fn populate_empty_rows(&mut self) {
        for row_num in 0..self.map.rows() {
            if self.map.iter_row(row_num).all(|&x| !x) {
                self.empty_rows.push(row_num)
            }
        }
    }
    fn populate_empty_cols(&mut self) {
        for col_num in 0..self.map.cols() {
            if self.map.iter_col(col_num).all(|&x| !x) {
                self.empty_cols.push(col_num + 1)
            }
        }
    }
    fn populate_empties(&mut self) {
        self.populate_empty_rows();
        self.populate_empty_cols();
    }
}
