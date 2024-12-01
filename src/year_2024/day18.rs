use std::time::{Duration, Instant};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2023/day_18.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve(Instruction::p1_parser), solve(Instruction::p2_parser))
}
fn solve(instr_parser: impl Fn(&str) -> Instruction) -> (usize, Duration) {
    let now = Instant::now();
    let instructions = LINES.lines().map(instr_parser).collect_vec();
    let ans = calculate_area(&instructions);
    (ans, now.elapsed())
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
    fn get_delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    value: isize,
}
impl Instruction {
    fn p1_parser(s: &str) -> Self {
        let (action_str, val_str, _) = s.split_ascii_whitespace().collect_tuple().unwrap();
        Instruction {
            direction: Direction::from_str(action_str),
            value: val_str.parse().unwrap(),
        }
    }
    fn p2_parser(s: &str) -> Self {
        let (_, _, val_str) = s.split_ascii_whitespace().collect_tuple().unwrap();
        let value = from_hex(&val_str[2..7]);
        let direction = match val_str.chars().nth(7).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Invalid direction"),
        };

        Instruction { direction, value }
    }
}

fn calculate_area(instructions: &[Instruction]) -> usize {
    let mut x = 0;
    let mut area = 0;
    let mut perimeter = 0;
    for instr in instructions {
        let dir = instr.direction;
        let val = instr.value;
        let (dx, dy) = dir.get_delta();
        let dx = dx * val;
        let dy = dy * val;
        x += dx;
        perimeter += val as usize;
        area += x * dy;
    }
    area.abs() as usize + perimeter / 2 + 1
}

fn from_hex(s: &str) -> isize {
    isize::from_str_radix(s, 16).unwrap()
}
