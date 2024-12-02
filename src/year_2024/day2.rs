use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../problem_inputs_2024/day_2_test.txt");
    } else {
        lines = include_str!("../../problem_inputs_2024/day_2.txt");
    }
    let reports = lines.lines().map(|line| Report::parse_line(line)).collect_vec();
    let ans1 = solve01(&reports);
    let ans2 = solve02(&reports);
    (ans1, ans2)
}

fn solve01(reports: &[Report]) -> (usize, Duration) {
    let now = Instant::now();
    // let reports = reports.iter().collect_vec();
    // dbg!(&reports);
    let ans = reports.iter().filter(|x| x.is_safe_strict()).count();
    (ans, now.elapsed())
}

fn solve02(reports: &[Report]) -> (usize, Duration) {
    let now = Instant::now();
    // let reports = reports.iter().collect_vec();
    // dbg!(&reports);
    let ans = reports.iter().filter(|x| x.is_safe_loose()).count();
    (ans, now.elapsed())
}

type Level = usize;

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<Level>,
}

impl Report {
    fn parse_line(line: &str) -> Self {
        let levels = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect_vec();
        Self { levels }
    }
    fn is_safe_strict(&self) -> bool {
        if self.levels.len() < 3 {
            return true;
        }
        let mut increasing = true;
        let mut decreasing = true;

        for window in self.levels.windows(2) {
            let a = window[0];
            let b = window[1];
            // dbg!(a,b);
            if a.abs_diff(b) > 3 || a == b {
                return false;
            }
            if a >= b {
                increasing = false;
            }
            if a <= b {
                decreasing = false;
            }
        }
        increasing || decreasing
    }

    fn is_safe_loose(&self) -> bool {
        if self.levels.len() < 4 {
            return true;
        }
        if self.is_safe_strict() {
            return true;
        }
        for removal_index in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(removal_index);
            let report_trimmed = Report { levels };
            if report_trimmed.is_safe_strict() {
                return true;
            }
        }
        false
    }
}
