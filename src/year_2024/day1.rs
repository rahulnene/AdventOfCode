use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_1_test.txt");
    } else {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_1.txt");
    }
    let mut left_col = Vec::new();
    let mut right_col = Vec::new();
    for line in lines.lines() {
        let (left, right) = parse_line(line);
        left_col.push(left);
        right_col.push(right);
    }
    left_col.sort_unstable();
    right_col.sort_unstable();
    (
        solve01(&left_col, &right_col),
        solve02(&left_col, &right_col),
    )
}

fn solve01(left_col: &[usize], right_col: &[usize]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = left_col
        .iter()
        .zip(right_col.iter())
        .map(|(l, r)| (l).abs_diff(*r))
        .sum();
    (ans, now.elapsed())
}

fn solve02(left_col: &[usize], right_col: &[usize]) -> (usize, Duration) {
    let now = Instant::now();
    let right_counts = right_col.iter().copied().counts();
    let ans = left_col
        .iter()
        .map(|l| l * right_counts.get(l).unwrap_or(&0))
        .sum();
    (ans, now.elapsed())
}

fn parse_line(line: &str) -> (usize, usize) {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect_tuple()
        .unwrap()
}
