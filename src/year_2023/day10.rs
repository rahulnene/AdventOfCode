use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2023/day_10_test.txt");

pub fn solution() -> ((usize, Duration), (isize, Duration)) {
    let mut map = FxHashMap::default();
    for (r, line) in LINES.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            map.insert((c as isize, r as isize), Pipe::from_char(ch));
        }
    }
    (solve01(&map), solve02(&map))
}

fn solve01(map: &FxHashMap<Location, Pipe>) -> (usize, Duration) {
    let now = Instant::now();
    let start_loc = map
        .iter()
        .find(|(_, &pipe)| pipe == Pipe::Start)
        .map(|(loc, _)| loc)
        .unwrap();
    let next_locs = vec![
        (Direction::Right, (start_loc.0 + 1, start_loc.1)),
        (Direction::Down, (start_loc.0, start_loc.1 + 1)),
        (Direction::Left, (start_loc.0 - 1, start_loc.1)),
        (Direction::Up, (start_loc.0, start_loc.1 - 1)),
    ];
    let next_locs = next_locs
        .iter()
        .copied()
        .find_or_first(|loc| map.get(&loc.1).is_some())
        .unwrap();
    let mut walker_1 = Walker {
        pos: *start_loc,
        direction: next_locs.0,
    };
    let walker_1_dist = walker_1.walk_till_end(map).0;
    (walker_1_dist / 2, now.elapsed())
}

fn solve02(map: &FxHashMap<Location, Pipe>) -> (isize, Duration) {
    let now = Instant::now();
    let start_loc = map
        .iter()
        .find(|(_, &pipe)| pipe == Pipe::Start)
        .map(|(loc, _)| loc)
        .unwrap();
    let next_locs = vec![
        (Direction::Right, (start_loc.0 + 1, start_loc.1)),
        (Direction::Down, (start_loc.0, start_loc.1 + 1)),
        (Direction::Left, (start_loc.0 - 1, start_loc.1)),
        (Direction::Up, (start_loc.0, start_loc.1 - 1)),
    ];
    let next_locs = next_locs
        .iter()
        .copied()
        .find_or_first(|loc| map.get(&loc.1).is_some())
        .unwrap();
    let mut walker_1 = Walker {
        pos: *start_loc,
        direction: next_locs.0,
    };
    let pipe_locations = walker_1.walk_till_end(map).1;
    // let mut interior_positions = Vec::new();
    let a = get_one_interior_point(map, &pipe_locations);
    dbg!(a);
    (0, now.elapsed())
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

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone, Copy)]
struct Walker {
    pos: Location,
    direction: Direction,
}

impl Walker {
    fn walk_till_end(&mut self, map: &FxHashMap<Location, Pipe>) -> (usize, Vec<Location>) {
        let mut steps = 0;
        let mut pipe_locs = Vec::new();
        loop {
            let current_pipe = *map.get(&self.pos).unwrap();
            pipe_locs.push(self.pos);
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
            if map.get(&self.pos).unwrap() == &Pipe::Start {
                return (steps + 1, pipe_locs);
            }
            steps += 1;
        }
    }
}

type Location = (isize, isize);
fn get_one_interior_point(map: &FxHashMap<Location, Pipe>, pipe_locs: &[Location]) -> Location {
    for point in map.keys() {
        if is_inside(map, point, pipe_locs) {
            return *point;
        }
    }
    unreachable!()
}

fn is_inside(map: &FxHashMap<Location, Pipe>, pos: &Location, pipe_locs: &[Location]) -> bool {
    if pipe_locs.contains(pos) {
        return false;
    }
    let mut boundary = VecDeque::new();
    boundary.push_back(*pos);
    let mut old_size = 1;
    while let Some(pos) = boundary.pop_front() {
        let tile = map.get(&pos);
        if tile.is_none() {
            return false;
        }
        if pipe_locs.contains(&pos) {
            continue;
        }
        boundary.push_back((pos.0 - 1, pos.0));
        boundary.push_back((pos.0 + 1, pos.0));
        boundary.push_back((pos.0, pos.0 - 1));
        boundary.push_back((pos.0, pos.0 + 1));
        boundary = boundary.iter().unique().copied().collect();
        if boundary.len() == old_size {
            break;
        }
        old_size = boundary.len();
    }
    true
}
