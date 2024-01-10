use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2015/day_5.txt");
    (solve(lines, is_nice), solve(lines, is_nice_2))
}

fn solve(lines: &str, nice_checker: fn(&str) -> bool) -> (usize, Duration) {
    let now = Instant::now();
    let ans = lines.lines().filter(|l| nice_checker(l)).count();
    (ans, now.elapsed())
}

fn is_nice(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;
    let mut has_bad = false;
    let mut last_char = ' ';
    for c in s.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            vowel_count += 1;
        }
        if c == last_char {
            has_double = true;
        }
        if (last_char == 'a' && c == 'b')
            || (last_char == 'c' && c == 'd')
            || (last_char == 'p' && c == 'q')
            || (last_char == 'x' && c == 'y')
        {
            has_bad = true;
        }
        last_char = c;
    }
    vowel_count >= 3 && has_double && !has_bad
}

fn is_nice_2(s: &str) -> bool {
    let mut has_pair = false;
    let mut has_repeat = false;
    let mut last_char = ' ';
    let mut last_last_char = ' ';
    for (i, c) in s.chars().enumerate() {
        if i > 1 && c == last_last_char {
            has_repeat = true;
        }
        if i > 0 {
            let pair = format!("{last_char}{c}");
            if s[i + 1..].contains(pair.as_str()) {
                has_pair = true;
            }
        }
        last_last_char = last_char;
        last_char = c;
    }
    has_pair && has_repeat
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_nicep1() {
        assert_eq!(super::is_nice("ugknbfddgicrmopn"), true);
    }
    #[test]
    fn test_is_nicep2() {
        assert_eq!(super::is_nice("jchzalrnumimnmhp"), false);
    }
    #[test]
    fn test_is_nice_2p1() {
        assert_eq!(super::is_nice_2("qjhvhtzxzqqjkmpb"), true);
    }
    #[test]
    fn test_is_nice_2p2() {
        assert_eq!(super::is_nice_2("uurcxstgmygtbstg"), false);
    }
}
