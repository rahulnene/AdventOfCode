use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::{Duration, Instant};

lazy_static! {
    static ref RE: Regex = Regex::new(r"\[([^\]]*)\]").expect("Bad regex pattern");
    static ref LINES: String = include_str!("../../problem_inputs_2016/day_7.txt").to_owned();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve_01(&LINES), solve_02(&LINES))
}

fn solve_01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = lines.lines().filter(|ip| supports_tls(ip)).count();
    (ans, now.elapsed())
}

fn solve_02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let ans = lines.lines().filter(|ip| supports_ssl(ip)).count();
    (ans, now.elapsed())
}

fn supports_tls(ip: &str) -> bool {
    let (in_brackets, rest) = extract_and_remainder(ip);
    !in_brackets.iter().any(|s| has_abba(s.as_bytes()))
        && rest.iter().any(|s| has_abba(s.as_bytes()))
}

fn supports_ssl(ip: &str) -> bool {
    let (in_brackets, rest) = extract_and_remainder(ip);
    let abs = rest
        .iter()
        .flat_map(|s| get_abs(s.as_bytes()))
        .collect_vec();
    let bas = in_brackets
        .iter()
        .flat_map(|s| get_abs(s.as_bytes()))
        .map(|(c1, c2)| (c2, c1))
        .collect_vec();
    abs.iter().any(|j| bas.iter().any(|k| j == k))
}

fn has_abba(s: &[u8]) -> bool {
    s.iter()
        .tuple_windows()
        .any(|(c1, c2, c3, c4)| c1 == c4 && c2 == c3 && c1 != c2)
}

fn get_abs(s: &[u8]) -> Vec<(u8, u8)> {
    s.iter()
        .tuple_windows()
        .filter(|(c1, c2, c3)| c1 == c3 && c1 != c2)
        .map(|(c1, c2, _)| (*c1, *c2))
        .collect_vec()
}

fn extract_and_remainder(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut extracted_strings = Vec::new();
    let mut remainder_strings = Vec::new();
    let mut last_end = 0;

    for capture in RE.find_iter(input) {
        let start = capture.start();
        let end = capture.end();

        extracted_strings.push(&input[start + 1..end - 1]);
        remainder_strings.push(&input[last_end..start]);
        last_end = end;
    }

    remainder_strings.push(&input[last_end..]);

    (extracted_strings, remainder_strings)
}
