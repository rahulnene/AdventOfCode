use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut pairs = include_str!("../../problem_inputs_2016/day_20.txt")
        .lines()
        .map(|line| {
            let v: Vec<_> = line
                .split('-')
                .map(|w| w.parse::<usize>().unwrap())
                .collect();
            (v[0], v[1])
        })
        .collect_vec();
    pairs.sort_unstable();
    (solve01(&pairs), solve02(&pairs))
}

fn solve01(pairs: &[(usize, usize)]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = pairs.iter().fold(0, |h, &(low, high)| {
        if low > h {
            return h;
        }
        std::cmp::max(h, high + 1)
    });
    (ans, now.elapsed())
}

fn solve02(pairs: &[(usize, usize)]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = pairs
        .iter()
        .chain(std::iter::once(&(1 << 32, 1 << 32)))
        .fold((0, 0), |(h, c), &(low, high)| {
            if low > h {
                (high + 1, c + low - h)
            } else {
                (std::cmp::max(h, high + 1), c)
            }
        })
        .1;
    (ans, now.elapsed())
}
