use std::{
    ops::Deref,
    time::{Duration, Instant},
};

use fxhash::FxHashMap;
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2023/day_8_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let (instrs, dirs) = lines
        .split("\r\n\r\n")
        .map(str::trim)
        .collect_tuple()
        .unwrap();
    let mut dir_to_next_map = FxHashMap::default();
    for dir in dirs.lines() {
        let frags: Vec<&str> = dir.split_whitespace().collect_vec();
        let current_dir: &str = frags[0].trim();
        let left: String = frags[2].chars().filter(|c| c.is_alphanumeric()).collect();
        let right: String = frags[3].chars().filter(|c| c.is_alphanumeric()).collect();
        dir_to_next_map.insert(current_dir, (left, right));
    }
    let mut position = "AAA";
    for (num, ch) in instrs.chars().cycle().enumerate() {
        let map = dir_to_next_map.get(&position).unwrap();
        match ch {
            'R' => {
                position = map.1.as_str();
            }
            'L' => {
                position = map.0.as_str();
            }
            _ => panic!("Invalid char"),
        }
        if position.contains("ZZZ") {
            println!("Finished P1");
            return (num + 1, now.elapsed());
        }
    }
    dbg!(position);
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let (instrs, dirs) = lines
        .split("\r\n\r\n")
        .map(str::trim)
        .collect_tuple()
        .unwrap();
    let mut dir_to_next_map = FxHashMap::default();
    for dir in dirs.lines() {
        let frags: Vec<&str> = dir.split_whitespace().collect_vec();
        let current_dir: &str = frags[0].trim();
        let left: String = frags[2].chars().filter(|c| c.is_alphabetic()).collect();
        let right: String = frags[3].chars().filter(|c| c.is_alphabetic()).collect();
        dir_to_next_map.insert(current_dir, (left, right));
    }
    let mut start_positions = dir_to_next_map
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| *s)
        .collect_vec();

    for (num, ch) in instrs.chars().cycle().enumerate() {
        let mut next_positions = Vec::new();
        for position in start_positions.iter() {
            let map = dir_to_next_map.get(position).unwrap();
            match ch {
                'R' => {
                    next_positions.push(map.1.as_str());
                }
                'L' => {
                    next_positions.push(map.0.as_str());
                }
                _ => panic!("Invalid char"),
            }
        }
        if next_positions.iter().all(|s| s.ends_with('Z')) {
            return (num+1, now.elapsed());
        }
        start_positions = next_positions;
    }
    (0, now.elapsed())
}
