use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2023/day_10.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut map = FxHashMap::default();
    for (r, line) in LINES.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            map.insert((c as isize, r as isize), Pipe::from_char(ch));
        }
    }
    solve(&map)
}

fn solve(map: &FxHashMap<Location, Pipe>) -> ((usize, Duration), (usize, Duration)) {
    let now = Instant::now();
    let start_loc = map
        .iter()
        .find(|(_, &pipe)| pipe == Pipe::Start)
        .map(|(loc, _)| loc)
        .unwrap();
    let next_locs = [
        (Direction::Right, (start_loc.0 + 1, start_loc.1)),
        (Direction::Down, (start_loc.0, start_loc.1 + 1)),
        (Direction::Left, (start_loc.0 - 1, start_loc.1)),
        (Direction::Up, (start_loc.0, start_loc.1 - 1)),
    ];
    let start_dir = next_locs
        .iter()
        .copied()
        .find_or_first(|loc| map.get(&loc.1).is_some())
        .unwrap()
        .0;
    let mut walker_1 = Walker {
        pos: *start_loc,
        direction: start_dir,
        history: Vec::new(),
    };
    let walker_1_dist = walker_1.walk_till_end(map);
    let ans = calculate_area(&walker_1.history) - walker_1_dist;
    ((walker_1_dist / 2, now.elapsed()), (ans, now.elapsed()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Horizontal,
    Vertical,
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
    Start,
    Blank,
}
impl Pipe {
    fn from_char(c: char) -> Pipe {
        match c {
            '-' => Pipe::Horizontal,
            '|' => Pipe::Vertical,
            '7' => Pipe::RightTop,
            'F' => Pipe::LeftTop,
            'L' => Pipe::LeftBottom,
            'J' => Pipe::RightBottom,
            'S' => Pipe::Start,
            '.' => Pipe::Blank,
            _ => unreachable!("Invalid pipe character: {c}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply_pipe(&self, pipe: Pipe) -> Self {
        match pipe {
            Pipe::Horizontal | Pipe::Vertical | Pipe::Start => *self,

            Pipe::LeftTop => match self {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Down,
                _ => unreachable!(),
            },
            Pipe::RightTop => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Down,
                _ => unreachable!(),
            },
            Pipe::LeftBottom => match self {
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                _ => unreachable!(),
            },
            Pipe::RightBottom => match self {
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Up,
                _ => unreachable!(),
            },
            Pipe::Blank => unreachable!(),
        }
    }
    fn get_delta(self) -> (isize, isize) {
        match self {
            Self::Up => (0, 1),
            Self::Down => (0, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Walker {
    pos: Location,
    direction: Direction,
    history: Vec<Direction>,
}

impl Walker {
    fn walk_till_end(&mut self, map: &FxHashMap<Location, Pipe>) -> usize {
        let mut steps = 0;
        loop {
            let current_pipe = *map.get(&self.pos).unwrap();
            self.direction = self.direction.apply_pipe(current_pipe);
            match self.direction {
                Direction::Up => {
                    self.pos.1 -= 1;
                }
                Direction::Down => {
                    self.pos.1 += 1;
                }
                Direction::Left => {
                    self.pos.0 -= 1;
                }
                Direction::Right => {
                    self.pos.0 += 1;
                }
            }
            self.history.push(self.direction);
            if map.get(&self.pos).unwrap() == &Pipe::Start {
                return steps + 1;
            }
            steps += 1;
        }
    }
}

type Location = (isize, isize);
fn calculate_area(instructions: &[Direction]) -> usize {
    let mut x = 0;
    let mut area = 0;
    let mut perimeter = 0;
    for dir in instructions {
        let (dx, dy) = dir.get_delta();
        x += dx;
        perimeter += 1;
        area += x * dy;
    }
    area.unsigned_abs() + perimeter / 2 + 1
}
