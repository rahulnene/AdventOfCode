use std::time::{Instant, Duration};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2019/day_18.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}