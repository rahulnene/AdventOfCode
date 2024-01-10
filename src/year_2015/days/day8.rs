use std::time::{Duration, Instant};

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2015/day_8.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = lines
        .lines()
        .map(|l| (get_raw_size(l) - get_actual_size(l)))
        .sum();
    dbg!(ans);
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = lines
        .lines()
        .map(|l| (encode(l).len() - get_raw_size(l)))
        .sum();
    (ans, now.elapsed())
}

fn get_raw_size(s: &str) -> usize {
    s.len()
}

fn get_actual_size(s: &str) -> usize {
    let mut actual_size = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            let next = chars.next().unwrap();
            if next == 'x' {
                chars.next();
                chars.next();
                actual_size += 1;
            } else {
                actual_size += 1;
            }
        } else {
            actual_size += 1;
        }
    }
    actual_size - 2
}

fn encode(s: &str) -> String {
    let mut encoded = String::new();
    encoded.push('"');
    for c in s.chars() {
        if c == '\\' {
            encoded.push('\\');
            encoded.push('\\');
        } else if c == '"' {
            encoded.push('\\');
            encoded.push('"');
        } else {
            encoded.push(c);
        }
    }
    encoded.push('"');
    encoded
}
