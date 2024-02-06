use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2021/day_8.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut count = 0;
    for line in LINES.lines() {
        let output = line.split_once(" | ").unwrap().1;
        let a = output
            .trim()
            .split(' ')
            .map(|f| f.to_string())
            .collect_vec();
        for i in 0..4 {
            match a[i].len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            };
        }
    }
    (count, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut sum = 0;
    for line in LINES.lines() {
        let (wire_pattern, output_wires) = line.split_once(" | ").unwrap();
        let wire_count_to_chars: FxHashMap<usize, FxHashSet<char>> = wire_pattern
            .split_whitespace()
            .map(|s| (s.len(), s.chars().collect()))
            .collect();
        let mut output_str = String::new();
        for output_char_set in output_wires
            .split_whitespace()
            .map(|s| s.chars().collect::<FxHashSet<_>>())
        {
            match (
                output_char_set.len(),
                output_char_set
                    .intersection(&wire_count_to_chars[&4])
                    .count(),
                output_char_set
                    .intersection(&wire_count_to_chars[&2])
                    .count(),
            ) {
                (2, _, _) => output_str.push('1'),
                (3, _, _) => output_str.push('7'),
                (4, _, _) => output_str.push('4'),
                (7, _, _) => output_str.push('8'),
                (5, 2, _) => output_str.push('2'),
                (5, 3, 1) => output_str.push('5'),
                (5, 3, 2) => output_str.push('3'),
                (6, 4, _) => output_str.push('9'),
                (6, 3, 1) => output_str.push('6'),
                (6, 3, 2) => output_str.push('0'),
                _ => (),
            }
        }
        sum += output_str.parse::<usize>().unwrap();
    }
    (sum, now.elapsed())
}
