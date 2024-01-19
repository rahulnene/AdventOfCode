use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2019/day_10_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let field = Field::new(lines);
    let a = field
        .asteroids
        .iter()
        .map(|a| (a, field.count_visible(*a)))
        .max_by_key(|s| s.1)
        .unwrap();
    (a.1, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let field = Field::new(lines);
    let a = field
        .asteroids
        .iter()
        .map(|a| (a, field.count_visible(*a)))
        .max_by_key(|s| s.1)
        .unwrap();
    let station = a.0;
    dbg!(field.get_seen(*station).first());
    dbg!(field.get_seen(*station).last());
    (a.1, now.elapsed())
}

#[derive(Debug, Clone)]
struct Field {
    asteroids: FxHashSet<Position>,
}

impl Field {
    fn new(lines: &str) -> Self {
        let mut asteroids = FxHashSet::default();
        for (y, line) in lines.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroids.insert((OrderedFloat(x as f64), OrderedFloat(y as f64)));
                }
            }
        }
        Self { asteroids }
    }

    fn count_visible(&self, pos: Position) -> usize {
        let mut angles = Vec::default();
        for asteroid in &self.asteroids {
            if *asteroid == pos {
                continue;
            }
            let angle = (asteroid.1 - pos.1).atan2(*asteroid.0 - *pos.0);
            angles.push(OrderedFloat(angle));
        }
        angles.iter().unique().collect_vec().len()
    }

    fn get_seen(&self, pos: Position) -> Vec<(&Position, f64)> {
        let mut asteroids_with_angles = self
            .asteroids
            .iter()
            .map(|a| {
                (a, {
                    let mut angle = (a.1 - pos.1).atan2(*(a.0 - pos.0));
                    angle *= -1.0;
                    angle += std::f64::consts::FRAC_PI_2;
                    if angle > 0.0 {
                        angle -= std::f64::consts::PI * 2.0;
                    }
                    angle
                })
            })
            .collect_vec();
        asteroids_with_angles.sort_by_key(|(a, b)| {
            OrderedFloat(f64::sqrt(
                f64::abs(*(pos.0 - a.0)) + f64::abs(*(pos.1 - a.1)),
            )) * OrderedFloat(*b)
        });
        asteroids_with_angles
    }
}

type Position = (OrderedFloat<f64>, OrderedFloat<f64>);
