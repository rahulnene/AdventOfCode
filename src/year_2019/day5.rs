use super::intcode::Computer;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2019/day_5.txt");

pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    let now = Instant::now();
    let mut comp = Computer::new(LINES, 1);
    let ans = comp.run_to_halt();
    (ans, now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    let mut comp = Computer::new(LINES, 5);
    let ans = comp.run_to_halt();
    (ans, now.elapsed())
}
