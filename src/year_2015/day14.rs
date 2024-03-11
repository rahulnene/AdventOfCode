use std::{
    cmp::min,
    time::{Duration, Instant},
};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2015/day_14.txt");
const TIME_LIMIT: usize = 2503;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut reindeers = FxHashMap::default();
    for line in LINES.lines() {
        let (speed_str, rest_str) = line.split_once(',').unwrap();
        let (name, _, _, speed, _, _, fly_time, _) =
            speed_str.split_whitespace().collect_tuple().unwrap();
        let (_, _, _, _, _, rest_time, _) = rest_str.split_whitespace().collect_tuple().unwrap();
        reindeers.insert(
            name,
            Reindeer::new(
                speed.parse().unwrap(),
                fly_time.parse().unwrap(),
                rest_time.parse().unwrap(),
            ),
        );
    }
    (solve01(&reindeers), solve02(&reindeers))
}

fn solve01(reindeers: &FxHashMap<&str, Reindeer>) -> (usize, Duration) {
    let now = Instant::now();
    let max_distance = reindeers
        .values()
        .map(|reindeer| reindeer.distance(TIME_LIMIT))
        .max()
        .unwrap();
    (max_distance, now.elapsed())
}

fn solve02(reindeers: &FxHashMap<&str, Reindeer>) -> (usize, Duration) {
    let now = Instant::now();
    let mut reindeers_score = FxHashMap::default();
    for time in 1..=TIME_LIMIT + 1 {
        let leader = get_leader(&reindeers, time);
        dbg!(&leader);
        let _ = *reindeers_score
            .entry(leader)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    dbg!(&reindeers_score);
    let max_score = *reindeers_score.values().max().unwrap();
    (max_score as usize, now.elapsed())
}
#[derive(Debug, Clone, Copy)]
struct Reindeer {
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn new(speed: usize, fly_time: usize, rest_time: usize) -> Self {
        Self {
            speed,
            fly_time,
            rest_time,
        }
    }

    fn distance(&self, time: usize) -> usize {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = time / cycle_time;
        let remaining_time = time % cycle_time;
        let fly_time = min(remaining_time, self.fly_time);
        cycles * self.speed * self.fly_time + fly_time * self.speed
    }
}

fn get_leader(reindeers: &FxHashMap<&str, Reindeer>, time: usize) -> String {
    reindeers
        .iter()
        .map(|(name, reindeer)| (name, reindeer.distance(time)))
        .max_by_key(|v| v.1)
        .unwrap()
        .0
        .to_string()
}
