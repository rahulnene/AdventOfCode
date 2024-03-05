use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2018/day_6.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let points: Vec<Coord> = LINES
        .lines()
        .map(|l| {
            let mut split = l.split(", ");
            (
                split.next().unwrap().parse::<isize>().unwrap(),
                split.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let min_x = points.iter().map(|p| p.0).min().unwrap();
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let min_y = points.iter().map(|p| p.1).min().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();
    (
        solve01(&points, min_x, max_x, min_y, max_y),
        solve02(&points, min_x, max_x, min_y, max_y),
    )
}

fn solve01(
    points: &[Coord],
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
) -> (usize, Duration) {
    let now = Instant::now();
    let mut first_pass = FxHashMap::default();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let target = (x, y);
            if let Some(closest) = get_coord_closest_to(&target, &points) {
                first_pass
                    .entry(closest)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }
    let mut second_pass = FxHashMap::default();
    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            let target = (x, y);
            if let Some(closest) = get_coord_closest_to(&target, &points) {
                second_pass
                    .entry(closest)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }
    let common = first_pass
        .into_iter()
        .filter(|(k, v)| second_pass.get(k) == Some(v));
    let ans = common.map(|(_, v)| v).max().unwrap();
    (ans as usize, now.elapsed())
}

fn solve02(
    points: &[Coord],
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
) -> (usize, Duration) {
    let now = Instant::now();
    let mut safe = Vec::new();
    for x in min_x - 2..max_x + 3 {
        for y in min_y - 2..=max_y + 2 {
            safe.push(total_distance(&(x, y), &points));
        }
    }
    let ans = safe.into_iter().filter(|d| *d < 10000).count();
    (ans, now.elapsed())
}

type Coord = (isize, isize);
fn dist(a: &Coord, b: &Coord) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as usize
}

fn total_distance(target: &Coord, points: &[Coord]) -> usize {
    points.iter().map(|p| dist(target, p)).sum()
}

fn get_coord_closest_to(target: &Coord, points: &[Coord]) -> Option<usize> {
    let distances = points
        .iter()
        .enumerate()
        .map(|(i, p)| (i, dist(target, p)))
        .collect::<Vec<_>>();
    let min_distance = distances
        .iter()
        .sorted_by_key(|(_, d)| *d)
        .next()
        .unwrap()
        .1;
    let closest = distances
        .iter()
        .filter(|(_, d)| *d == min_distance)
        .collect_vec();
    if closest.len() == 1 {
        Some(closest.get(0).unwrap().0)
    } else {
        None
    }
}
