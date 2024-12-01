use std::time::{Duration, Instant};

use itertools::Itertools;

// const LINES: &str = include_str!("../../problem_inputs_2023/day_22.txt");
const LINES: &str = include_str!("../../problem_inputs_2023/day_22_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let bricks = LINES.lines().map(Brick::from_str).collect_vec();
    let mut sim = Simulation::new(bricks);
    println!("{:?}", &sim.bricks);
    sim.settle();
    println!("{:?}", &sim.bricks);
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

type Vec3 = (usize, usize, usize);

#[derive(Clone, Debug)]
struct Simulation {
    bricks: Vec<Brick>,
}

impl Simulation {
    fn new(bricks: Vec<Brick>) -> Self {
        Self { bricks }
    }

    fn settle(&mut self) -> bool {
        let mut settled = false;
        
        settled
    }
}

#[derive(Clone, Copy, Debug)]
struct Brick {
    start: Vec3,
    end: Vec3,
    length: usize,
}

impl Brick {
    fn from_str(s: &str) -> Self {
        let (from, to) = s.split_once('~').unwrap();
        let start: Vec3 = from
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let end: Vec3 = to
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let length = 1 + end.0 + end.1 + end.2 - start.0 - start.1 - start.2;
        Self { start, end, length }
    }
}
