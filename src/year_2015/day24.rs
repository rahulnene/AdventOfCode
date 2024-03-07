use std::time::{Duration, Instant};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2015/day_24.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let packages = LINES
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (solve01(&packages), solve02(&packages))
}

fn solve01(packages: &[usize]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = get_min_qe_for_n_parts(packages, 3);
    (ans, now.elapsed())
}

fn solve02(packages: &[usize]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = get_min_qe_for_n_parts(packages, 4);
    (ans, now.elapsed())
}

fn quantum_entanglement(packages: &[&usize]) -> usize {
    packages.into_iter().copied().product()
}

fn get_min_qe_for_n_parts(packages: &[usize], n: usize) -> usize {
    let total_weight: usize = packages.iter().sum();
    let target_weight = total_weight / n;
    let possible_combos = (1..packages.len())
        .map(|combo_lenths| {
            packages
                .iter()
                .combinations(combo_lenths)
                .filter(|combo| combo.into_iter().copied().sum::<usize>() == target_weight)
                .collect::<Vec<_>>()
        })
        .filter(|l| l.len() > 0)
        .next()
        .unwrap();
    let ans = possible_combos
        .iter()
        .map(|v| quantum_entanglement(&v))
        .min()
        .unwrap();
    ans
}
