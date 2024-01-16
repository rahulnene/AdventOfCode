use std::time::{Duration, Instant};

use fxhash::FxHashMap;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let line = include_str!("../../../problem_inputs_2015/day_3.txt");
    let instructions = line.chars().map(Direction::from_char).collect::<Vec<_>>();
    (solve01(&instructions), solve02(&instructions))
}

fn solve01(instrs: &[Direction]) -> (usize, Duration) {
    let now = Instant::now();
    let mut houses: FxHashMap<(isize, isize), usize> = FxHashMap::default();
    let mut position = (0, 0);
    let current_house = houses.entry(position).or_insert(0);
    *current_house += 1;
    for dir in instrs {
        match dir {
            Direction::Up => position.1 += 1,
            Direction::Down => position.1 -= 1,
            Direction::Left => position.0 -= 1,
            Direction::Right => position.0 += 1,
        }
        let current_house = houses.entry(position).or_insert(0);
        *current_house += 1;
    }
    let ans = houses.values().filter(|&&n| n >= 1).count();
    (ans, now.elapsed())
}

fn solve02(line: &[Direction]) -> (usize, Duration) {
    let now = Instant::now();
    let mut houses: FxHashMap<(isize, isize), usize> = FxHashMap::default();
    let mut position = (0, 0);
    let santa_instrs = line.iter().step_by(2).copied().collect::<Vec<_>>();
    let robot_instrs = line.iter().skip(1).step_by(2).copied().collect::<Vec<_>>();
    let current_house = houses.entry(position).or_insert(0);
    *current_house += 1;
    for dir in santa_instrs {
        match dir {
            Direction::Up => position.1 += 1,
            Direction::Down => position.1 -= 1,
            Direction::Left => position.0 -= 1,
            Direction::Right => position.0 += 1,
        }
        let current_house = houses.entry(position).or_insert(0);
        *current_house += 1;
    }
    position = (0, 0);
    for dir in robot_instrs {
        match dir {
            Direction::Up => position.1 += 1,
            Direction::Down => position.1 -= 1,
            Direction::Left => position.0 -= 1,
            Direction::Right => position.0 += 1,
        }
        let current_house = houses.entry(position).or_insert(0);
        *current_house += 1;
    }
    let ans = houses.len();
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
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid direction char: {}", c),
        }
    }
}
