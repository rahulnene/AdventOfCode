use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2015/day_6.txt");

lazy_static! {
    static ref INSTRUCTIONS: Vec<Instruction> = LINES.lines().map(Instruction::from_str).collect();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut grid = LightGrid::new();
    for instruction in INSTRUCTIONS.iter() {
        grid.apply_p1(instruction);
    }
    let ans = grid.lights.values().filter(|&&light| light == 1).count();
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut grid = LightGrid::new();
    for instruction in INSTRUCTIONS.iter() {
        grid.apply_p2(instruction);
    }
    let ans = grid.lights.values().sum();
    (ans, now.elapsed())
}

type Position = (usize, usize);
#[derive(Debug, Clone)]
struct LightGrid {
    lights: FxHashMap<Position, usize>,
}

impl LightGrid {
    fn new() -> Self {
        Self {
            lights: FxHashMap::default(),
        }
    }

    fn apply_p1(&mut self, instruction: &Instruction) {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                let pos = (x, y);
                let light = self.lights.entry(pos).or_insert(0);
                match instruction.action {
                    Action::TurnOn => *light = 1,
                    Action::TurnOff => *light = 0,
                    Action::Toggle => *light = light.abs_diff(1),
                }
            }
        }
    }
    fn apply_p2(&mut self, instruction: &Instruction) {
        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                let pos = (x, y);
                let light = self.lights.entry(pos).or_insert(0);
                match instruction.action {
                    Action::TurnOn => *light += 1,
                    Action::TurnOff => *light = light.saturating_sub(1),
                    Action::Toggle => *light += 2,
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone)]
struct Instruction {
    action: Action,
    start: Position,
    end: Position,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut words = s.split_whitespace();
        let action = match words.next().unwrap() {
            "toggle" => Action::Toggle,
            "turn" => match words.next().unwrap() {
                "on" => Action::TurnOn,
                "off" => Action::TurnOff,
                _ => panic!("Invalid action"),
            },
            _ => panic!("Invalid action"),
        };
        let start = {
            let mut nums = words.next().unwrap().split(',');
            (
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            )
        };
        words.next();
        let end = {
            let mut nums = words.next().unwrap().split(',');
            (
                nums.next().unwrap().parse().unwrap(),
                nums.next().unwrap().parse().unwrap(),
            )
        };
        Self { action, start, end }
    }
}
