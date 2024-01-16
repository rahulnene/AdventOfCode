use std::time::{Duration, Instant};
pub fn solution() -> ((isize, Duration), (usize, Duration)) {
    let line = include_str!("../../../problem_inputs_2015/day_1.txt");
    (solve01(&line), solve02(&line))
}

fn solve01(line: &str) -> (isize, Duration) {
    let now = Instant::now();
    let ans = line.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });
    dbg!(ans);
    (ans, now.elapsed())
}

fn solve02(line: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut floor = 0;
    let count = 0;
    for char in line.char_indices() {
        match char.1 {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor < 0 {
            return (char.0 + 1, now.elapsed());
        }
    }
    (count, now.elapsed())
}
