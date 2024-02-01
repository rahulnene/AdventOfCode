use itertools::Itertools;
use lazy_static::lazy_static;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2015/day_17.txt");
const TARGET_VOLUME: usize = 150;
lazy_static! {
    static ref CONTAINER_SIZE_LIST: Vec<usize> = LINES
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();

    let ans = (0..(2usize.pow(CONTAINER_SIZE_LIST.len() as u32)))
        .map(|num| {
            let bitmask: Vec<bool> = (0..CONTAINER_SIZE_LIST.len())
                .map(|i| (num & (1 << i)) != 0)
                .collect_vec();
            // (bitmask.clone(), calc_volume_of_combo(&bitmask))
            calc_volume_of_combo(&bitmask)
        })
        .filter(|v| *v == TARGET_VOLUME)
        .count();

    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let vols = (0..(2usize.pow(CONTAINER_SIZE_LIST.len() as u32)))
        .map(|num| {
            let bitmask: Vec<bool> = (0..CONTAINER_SIZE_LIST.len())
                .map(|i| (num & (1 << i)) != 0)
                .collect_vec();
            // (bitmask.clone(), calc_volume_of_combo(&bitmask))
            (
                bitmask.iter().filter(|b| **b).count(),
                calc_volume_of_combo(&bitmask),
            )
        })
        .filter(|v| v.1 == TARGET_VOLUME)
        .counts();
    let ans = vols.iter().min().unwrap().clone();
    (*ans.1, now.elapsed())
}

fn calc_volume_of_combo(combo: &[bool]) -> usize {
    combo
        .iter()
        .enumerate()
        .filter(|j| *j.1)
        .map(|a| CONTAINER_SIZE_LIST[a.0])
        .sum()
}
