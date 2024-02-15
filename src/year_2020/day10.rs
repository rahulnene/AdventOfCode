use itertools::Itertools;
use rustc_hash::FxHashMap;

use std::{
    cmp::min,
    time::{Duration, Instant},
};

const LINES: &str = include_str!("../../problem_inputs_2020/day_10.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut adapters: Vec<usize> = LINES
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    adapters.sort_unstable();
    let diffs = adapters
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<usize>>();

    let ans = (diffs.iter().filter(|&&x| x == 1).count() + 1)
        * (diffs.iter().filter(|&&x| x == 3).count() + 1);

    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut adapters: Vec<usize> = LINES
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    adapters.sort_unstable();
    let mut num_paths = FxHashMap::default();
    let n = adapters.len();
    num_paths.insert(adapters.last().copied().unwrap(), 1);
    for i in (0..(adapters.len() - 1)).into_iter().rev() {
        let i_val = adapters[i];
        let num_neighbors: usize = ((i + 1)..=(i + 3).min(n - 1))
            .filter_map(|j| {
                let j_val = adapters[j];
                let gap = j_val - i_val;
                if (1..=3).contains(&gap) {
                    Some(num_paths.get(&j_val).unwrap())
                } else {
                    None
                }
            })
            .sum();
        num_paths.insert(i_val, num_neighbors);
    }
    let ans = *num_paths.get(&1).unwrap_or(&0)
        + *num_paths.get(&2).unwrap_or(&0)
        + *num_paths.get(&3).unwrap_or(&0);
    (ans, now.elapsed())
}
