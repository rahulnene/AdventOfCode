use std::time::{Duration, Instant};

pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../problem_inputs_2024/day_5_test.txt");
    } else {
        lines = include_str!("../../problem_inputs_2024/day_5.txt");
    }
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = 0;
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = 0;
    (ans, now.elapsed())
}
