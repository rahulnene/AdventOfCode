use itertools::Itertools;
use lazy_static::lazy_static;
use std::time::{Duration, Instant};
lazy_static! {
    static ref LINES: &'static str = include_str!("../../problem_inputs_2016/day_3.txt");
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}
fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let ans = LINES
        .lines()
        .map(|tri_str| {
            tri_str
                .split_ascii_whitespace()
                .collect_tuple()
                .map(check_triangle)
                .unwrap()
        })
        .filter(|b| *b)
        .count();
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let lines: Vec<Vec<&str>> = LINES
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let ans = (0..lines[0].len())
        .map(|i| {
            let mut ans = Vec::new();
            for grouping in &lines.iter().chunks(3) {
                let batch = grouping.map(|line| line[i]).collect::<Vec<_>>();
                ans.push((batch[0], batch[1], batch[2]));
            }
            ans
        })
        .flatten()
        .map(check_triangle)
        .filter(|b| *b)
        .count();
    (ans, now.elapsed())
}

fn check_triangle((a, b, c): (&str, &str, &str)) -> bool {
    let a = a.parse::<usize>().unwrap();
    let b = b.parse::<usize>().unwrap();
    let c = c.parse::<usize>().unwrap();
    a + b > c && a + c > b && b + c > a
}
