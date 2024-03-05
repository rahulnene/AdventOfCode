use itertools::Itertools;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2019/day_4.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let range: (usize, usize) = LINES
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

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let range: (usize, usize) = LINES
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
    if !is_valid_password(password) {
        return false;
    }
    let mut prev = 0;
    let mut count = 1;
    for c in password.chars() {
        let digit = c.to_digit(10).unwrap();
        if digit == prev {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            count = 1;
        }
        prev = digit;
    }
    count == 2
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
