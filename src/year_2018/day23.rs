use itertools::Itertools;
use rustc_hash::FxHashMap;
use scan_fmt::scan_fmt;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2018/day_23_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut bots = LINES.lines().map(NanoBot::parse).collect::<Vec<NanoBot>>();
    let strongest = bots
        .iter()
        .max_by_key(|bot| bot.radius)
        .expect("No bots found");
    let strongest_bot = *strongest;
    bots.retain(|bot| strongest_bot.in_range_of(bot));
    (bots.len(), now.elapsed())
}
fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let bots = LINES.lines().map(NanoBot::parse).collect::<Vec<NanoBot>>();
    let mut coord_to_count = FxHashMap::default();
    for x in -100..100 {
        for y in -100..100 {
            for z in -100..100 {
                let coord = (x, y, z);
                let count = get_nanobots_in_range(coord, &bots);
                coord_to_count.insert(coord, count);
            }
        }
    }
    let max_count = coord_to_count.values().max().unwrap();
    let ans = coord_to_count
        .iter()
        .filter(|(_, &count)| count == *max_count)
        .map(|c| c.0 .0.abs() + c.0 .1.abs() + c.0 .2.abs())
        .min()
        .unwrap();
    (ans as usize, now.elapsed())
}

type Coordinate = (isize, isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NanoBot {
    position: Coordinate,
    radius: isize,
}

fn get_nanobots_in_range(coord: Coordinate, bots: &[NanoBot]) -> usize {
    bots.iter().filter(|bot| bot.in_range_of(*bot)).count()
}

impl NanoBot {
    fn new(position: Coordinate, radius: isize) -> Self {
        NanoBot { position, radius }
    }

    fn parse(line: &str) -> Self {
        let (x, y, z, radius) =
            scan_fmt!(line, "pos=<{},{},{}>, r={}", isize, isize, isize, isize).unwrap();
        NanoBot::new((x, y, z), radius)
    }

    fn distance_to(&self, other: &NanoBot) -> isize {
        let (x1, y1, z1) = self.position;
        let (x2, y2, z2) = other.position;
        (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
    }

    fn in_range_of(&self, other: &NanoBot) -> bool {
        self.distance_to(other) <= self.radius
    }
}
