use std::time::{Duration, Instant};
pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2023/day_1_test.txt");
    } else {
        lines = include_str!("../../../AdventOfCodeInputs/problem_inputs_2023/day_1.txt");
    }
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = lines.lines().map(|l| calibration_value(l)).sum();
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = lines
        .lines()
        .map(|l| calibration_value(&replace_numbers(l)))
        .sum();
    (ans, now.elapsed())
}

fn calibration_value(line: &str) -> usize {
    let s: Vec<usize> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as usize)
        .collect();
    match s.as_slice() {
        [single] => 11 * single,
        [tens, .., ones] => 10 * tens + ones,
        _ => 0,
    }
}

fn replace_numbers(s: &str) -> String {
    let to_replace = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    let mut barrel = String::new();
    for c in s.chars() {
        barrel.push(c);
        for (word, num) in to_replace {
            if barrel.ends_with(word) {
                barrel.replace_range(barrel.len() - word.len()..barrel.len(), num);
            }
        }
    }
    barrel
}
