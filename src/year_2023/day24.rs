use derive_more::*;
use itertools::Itertools;
use nalgebra::{Vector2, Vector3};
use pest::pratt_parser::Op;
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2023/day_24_test.txt");
const MIN: f64 = 7.0;
const MAX: f64 = 27.0;
// const LINES: &str = include_str!("../../problem_inputs_2023/day_24.txt");
// const MIN: f64 = 200000000000000.0;
// const MAX: f64 = 400000000000000.0;

pub fn solution() -> ((usize, Duration), (isize, Duration)) {
    let hailstones = LINES.lines().map(Hailstone::from_str).collect_vec();
    (solve01(&hailstones), solve02(&hailstones))
}

fn solve01(hailstones: &[Hailstone]) -> (usize, Duration) {
    let now = Instant::now();
    // dbg!(line_intersect_2d(&hailstones[0], &hailstones[4]));
    let mut ans = 0;
    let combinations: Vec<_> = hailstones.iter().tuple_combinations().collect();
    let ans = combinations
        .par_iter()
        .filter(|(h1, h2)| {
            if let Some(intersect) = intersection_2d(h1, h2) {
                // dbg!(intersect);
                !(intersect.x < MIN || intersect.x > MAX || intersect.y < MIN || intersect.y > MAX)
            } else {
                false
            }
        })
        .count();

    (ans, now.elapsed())
}

fn solve02(hailstones: &[Hailstone]) -> (isize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

type Vec3 = Vector3<f64>;
type Vec2 = Vector2<f64>;

#[derive(Clone, Debug)]
struct Hailstone {
    pos: Vec3,
    vel: Vec3,
}

impl Hailstone {
    fn new(pos: Vec3, vel: Vec3) -> Self {
        Self { pos, vel }
    }

    fn from_str(str: &str) -> Self {
        let (pos_str, vel_str) = str.split_once('@').unwrap();
        let pos_nums = pos_str
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap());
        let vel_nums = vel_str
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap());
        Self::new(Vec3::from_iterator(pos_nums), Vec3::from_iterator(vel_nums))
    }
}

fn convert_to_2d(hailstone: &Hailstone) -> (Vec2, Vec2) {
    (
        Vec2::new(hailstone.pos.x, hailstone.pos.y),
        Vec2::new(hailstone.vel.x, hailstone.vel.y),
    )
}

fn hailstone_to_2_points(hailstone: &Hailstone) -> (Vec2, Vec2) {
    let pos = hailstone.pos;
    let vel = hailstone.vel;
    (
        Vec2::new(pos.x, pos.y),
        Vec2::new(pos.x + vel.x * 10.0, pos.y + vel.y * 10.0),
    )
}

fn intersection_3d(h1: &Hailstone, h2: &Hailstone) -> Option<Vec2> {
    let (p1, v1) = (h1.pos, h1.vel);
    let (p2, v2) = (h2.pos, h2.vel);
    let v1xv2 = v1.cross(&v2);
    let t = (p2 - p1).cross(&v2).dot(&v1xv2) / v1xv2.norm_squared();
    let s = (p2 - p1).cross(&v1).dot(&v1xv2) / v1xv2.norm_squared();
    if t >= 0.0 && t <= 1.0 && s >= 0.0 && s <= 1.0 {
        return Some(Vec2::new(p1.x + t * v1.x, p1.y + t * v1.y));
    }
    None
}

fn intersection_2d(h1: &Hailstone, h2: &Hailstone) -> Option<Vec2> {
    let (p1, p2) = hailstone_to_2_points(h1);
    let (p3, p4) = hailstone_to_2_points(h2);
    let s1 = p2 - p1;
    let s2 = p4 - p3;
    let s = (-s1.y * (p1.x - p3.x) + s1.x * (p1.y - p3.y)) / (-s2.x * s1.y + s1.x * s2.y);
    let t = (s2.x * (p1.y - p3.y) - s2.y * (p1.x - p3.x)) / (-s2.x * s1.y + s1.x * s2.y);
    if s >= 0.0 && t >= 0.0 {
        return Some(Vec2::new(p1.x + (t * s1.x), p1.y + (t * s1.y)));
    }
    None
}
