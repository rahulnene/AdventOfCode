use lazy_static::lazy_static;
use regex::Regex;
use std::time::{Duration, Instant};

lazy_static! {
    static ref BASE_MULS: Regex =
        Regex::new(r"mul\(\s*(-?\d{1,3})\s*,\s*(-?\d{1,3})\s*\)").unwrap();
    static ref MULS_WITH_DISABLES: Regex =
        Regex::new(r"(?:mul\(\s*(-?\d{1,3})\s*,\s*(-?\d{1,3})\s*\)|do\(\)|don't\(\))").unwrap();
}

pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let line: &str;
    if test {
        line = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_3_test.txt");
    } else {
        line = include_str!("../../../AdventOfCodeInputs/problem_inputs_2024/day_3.txt");
    }
    (solve01(&line), solve02(&line))
}

fn solve01(line: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = get_sum_of_muls(line);
    (ans, now.elapsed())
}

fn solve02(line: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = get_sum_of_muls_with_disables(line);
    (ans, now.elapsed())
}

fn get_sum_of_muls(line: &str) -> usize {
    let mut ans: usize = 0;
    for cap in BASE_MULS.captures_iter(line) {
        let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        ans += (a * b) as usize;
    }
    ans
}

fn get_sum_of_muls_with_disables(line: &str) -> usize {
    let mut ans: usize = 0;
    let mut should_count = true;
    for cap in MULS_WITH_DISABLES.captures_iter(line) {
        let instr_str = cap.get(0).unwrap().as_str().split('(').next().unwrap();
        match instr_str {
            "mul" => {
                let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                if should_count {
                    ans += (a * b) as usize;
                }
            }
            "do" => {
                should_count = true;
            }
            "don't" => {
                should_count = false;
            }
            _ => {}
        }
    }
    ans
}
