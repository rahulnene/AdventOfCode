use std::time::{Duration, Instant};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2023/day_9.txt");

pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    let now = Instant::now();
    let ans = LINES
        .lines()
        .map(parse)
        .map(|nums| interpolate_forwards(&nums))
        .collect_vec();

    (ans.iter().sum(), now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    let ans = LINES
        .lines()
        .map(parse)
        .map(|nums| interpolate_backwards(&nums))
        .collect_vec();

    (ans.iter().sum(), now.elapsed())
}

fn parse(s: &str) -> Vec<isize> {
    s.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_vec()
}

fn interpolate_forwards(original_nums: &[isize]) -> isize {
    let mut sum = *original_nums.last().unwrap();
    let mut nums = original_nums.to_vec();
    loop {
        nums = diff(&nums);
        sum += *nums.last().unwrap();

        if nums.len() == 1 {
            return sum;
        }
    }
}
fn interpolate_backwards(original_nums: &[isize]) -> isize {
    let mut sum = *original_nums.iter().next().unwrap();
    let mut nums = original_nums.to_vec();
    let mut to_add = false;
    loop {
        nums = diff(&nums);
        // let relevant = *nums.iter().next().unwrap();
        // dbg!(relevant);
        if to_add {
            sum += *nums.iter().next().unwrap();
        } else {
            sum -= *nums.iter().next().unwrap();
        }

        if nums.len() == 1 {
            return sum;
        }
        to_add = !to_add;
    }
}

fn diff(nums: &[isize]) -> Vec<isize> {
    nums.iter()
        .tuple_windows()
        .map(|(i, j)| j - i)
        .collect_vec()
}
