use std::time::{Duration, Instant};
use itertools::Itertools;

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_11.txt");
    let dirs = lines.split(',').map(dir_to_axial).collect_vec();
    let now = Instant::now();
    let mut x = 0;
    let mut y = 0;
    let mut max = 0;
    for dir in dirs {
        x += dir.0;
        y += dir.1;
        let dist = axial_dist(x, y);
        if dist > max {
            max = dist;
        }
    }

    ((axial_dist(x, y), now.elapsed()), (max, now.elapsed()))
}

fn dir_to_axial(dir: &str) -> (isize, isize) {
    match dir {
        "n" => (1, -1),
        "ne" => (1, 0),
        "se" => (0, 1),
        "s" => (-1, 1),
        "sw" => (-1, 0),
        "nw" => (0, -1),
        _ => panic!("Invalid direction"),
    }
}

fn axial_dist(x: isize, y: isize) -> usize {
    ((x.abs() + y.abs() + (x + y).abs()) / 2) as usize
}
