use std::time::{Duration, Instant};

use rustc_hash::FxHashMap;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_13_test.txt");
    let mut firewall = FxHashMap::default();
    let mut max_wall = 0;
    for line in lines.lines() {
        let mut parts = line.split(": ");
        max_wall = parts.next().unwrap().parse::<usize>().unwrap();
        let range = parts.next().unwrap().parse::<usize>().unwrap();
        firewall.insert(max_wall, range);
    }
    (
        calc_severity(&firewall, max_wall, 0).0,
        calc_best_delay(&firewall, max_wall),
    )
}

fn calc_severity(
    firewall: &FxHashMap<usize, usize>,
    max_wall: usize,
    delay: usize,
) -> ((usize, Duration), usize) {
    let now = Instant::now();
    let ans = (0..=max_wall).filter_map(|t| {
        let t = t + delay;
        let p = firewall.get(&t).unwrap_or_else(|| &1);
        if calc_scanner_position(t, *p) == 0 {
            Some(t * p)
        } else {
            None
        }
    });
    ((ans.clone().sum(), now.elapsed()), ans.count())
}

fn calc_best_delay(firewall: &FxHashMap<usize, usize>, max_wall: usize) -> (usize, Duration) {
    let now = Instant::now();
    for delay in 0.. {
        let severity = calc_severity(firewall, max_wall, delay).1;
        if severity == 0 {
            return (delay, now.elapsed());
        }
    }
    return (0, now.elapsed());
}

fn calc_scanner_position(t: usize, range: usize) -> usize {
    let mut pos: isize = 0;
    let mut dir: isize = 1;
    for _ in 0..t {
        if pos == 0 {
            dir = 1;
        } else if pos == range as isize - 1 {
            dir = -1;
        }
        pos += dir;
    }
    pos as usize
}
