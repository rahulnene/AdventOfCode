use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2019/day_4.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let range: (usize, usize) = lines
        .trim()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let ans = (range.0..range.1)
        .map(|x| x.to_string())
        .filter(|x| is_valid_password(x))
        .count();
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let range: (usize, usize) = lines
        .trim()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let ans = (range.0..range.1)
        .map(|x| x.to_string())
        .filter(|x| is_valid_password2(x))
        .count();
    (ans, now.elapsed())
}

fn is_valid_password(password: &str) -> bool {
    let mut has_double = false;
    let mut has_decreasing = false;
    let mut prev = 0;
    for c in password.chars() {
        let digit = c.to_digit(10).unwrap();
        if digit == prev {
            has_double = true;
        }
        if digit < prev {
            has_decreasing = true;
        }
        prev = digit;
    }
    has_double && !has_decreasing
}

fn is_valid_password2(password: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(is_valid_password("111111"), true);
        assert_eq!(is_valid_password("223450"), false);
        assert_eq!(is_valid_password("123789"), false);
    }
    #[test]
    fn test2() {
        assert_eq!(is_valid_password2("112233"), true);
        assert_eq!(is_valid_password2("123444"), false);
        assert_eq!(is_valid_password2("111122"), true);
    }
}
