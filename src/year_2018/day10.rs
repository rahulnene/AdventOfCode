use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2018/day_10_test.txt");
lazy_static! {
    static ref RE: Regex =
        Regex::new(r"position=<\s?(-?\d+),\s+(-?\d+)> velocity=<\s?(-?\d+),\s+(-?\d+)>").unwrap();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut points: FxHashMap<(isize, isize), Point> = FxHashMap::default();
    for line in LINES.lines() {
        let caps = RE.captures(line).unwrap();
        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let vx = caps[3].parse().unwrap();
        let vy = caps[4].parse().unwrap();
        points.insert((x, y), Point { v_x: vx, v_y: vy });
    }
    while !check(&points) {
        step(&mut points);
    }
    dbg!(&points);
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy)]
struct Point {
    v_x: isize,
    v_y: isize,
}

fn step(points: &mut FxHashMap<(isize, isize), Point>) {
    let mut new_map = FxHashMap::default();
    for (loc, point) in points.iter() {
        new_map.insert((loc.0 + point.v_x, loc.1 + point.v_y), *point);
    }
    *points = new_map;
}

fn check(map: &FxHashMap<(isize, isize), Point>) -> bool {
    map.iter().all(|p| is_alone(map, *p.0))
}

fn is_alone(points: &FxHashMap<(isize, isize), Point>, to_check: (isize, isize)) -> bool {
    let mut count = 0;
    for (loc, _) in points.iter() {
        if loc.0 == to_check.0 + 1 || loc.0 == to_check.0 - 1 {
            count += 1;
        }
        if loc.1 == to_check.1 + 1 || loc.1 == to_check.1 - 1 {
            count += 1;
        }
    }
    count == 0
}
