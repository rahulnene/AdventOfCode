use std::{
    task::Wake,
    time::{Duration, Instant},
};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2022/day_16_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut valves: FxHashMap<String, Valve> = FxHashMap::default();
    for line in LINES.lines() {
        let (valve_details, connection_details) = line.split_once(';').unwrap();
        let (valve_name_str, flow_rate) = valve_details.split_once('=').unwrap();
        let flow_rate: usize = flow_rate.parse().unwrap();
        let valve_name = valve_name_str
            .split_ascii_whitespace()
            .skip(1)
            .next()
            .unwrap();
        let connects_to: Vec<String> = connection_details
            .trim()
            .split_ascii_whitespace()
            .skip(4)
            .map(str::to_string)
            .collect_vec();
        let valve = Valve {
            flow_rate,
            connects_to,
        };
        valves.insert(valve_name.to_string(), valve);
    }
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: usize,
    connects_to: Vec<String>,
}

struct Walker {
    current_pos: String,
    opened_with_time: Vec<(String, usize)>,
    timer: usize,
}

impl Walker {
    fn new() -> Self {
        Self {
            current_pos: "AA".to_string(),
            opened_with_time: vec![],
            timer: 0,
        }
    }
}
