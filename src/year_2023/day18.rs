use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::{cmp, fmt::Debug, str::FromStr};

use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2023/day_18_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut trench = Trench::new();
    for line in LINES.lines() {
        let direction = DigInstruction::from_str(line).unwrap();
        trench.dig(direction);
    }
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err("bad direction".to_owned()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct DigInstruction {
    dir: Direction,
    distance: usize,
}

impl FromStr for DigInstruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_str, dist_str, color_str) = s.split(' ').collect_tuple().unwrap();
        let dir = Direction::from_str(dir_str)?;
        let distance = usize::from_str(dist_str).map_err(|_| "bad distance")?;
        Ok(DigInstruction { dir, distance })
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Position {
    fn from(x: isize, y: isize) -> Position {
        Position { x, y }
    }
}

#[derive(Debug, Clone)]
struct Trench {
    contents: FxHashSet<Position>,
    total_area: usize,
    last_three_verticies: [Position; 3],
}

impl Trench {
    fn new() -> Trench {
        Trench {
            contents: FxHashSet::default(),
            total_area: 0,
            last_three_verticies: [Position::default(); 3],
        }
    }

    fn print(&self) {
        let bounds = self.find_bounds();
        let mut repr: Vec<Vec<char>> = Vec::new();
        for y in bounds.1 .0..=bounds.1 .1 {
            let mut row = Vec::new();

            for x in bounds.0 .0..=bounds.0 .1 {
                let current_cube = Position::from(x, y);

                if self.contents.contains(&current_cube) {
                    row.push('#')
                } else {
                    row.push('.')
                }
            }
            repr.push(row);
        }
        repr.reverse();
        fancy_print(repr);
    }

    fn dig(&mut self, instr: DigInstruction, from_pos: Position) {
        for _ in 0..instr.distance {
            match instr.dir {
                Direction::Left => current_pos.x -= 1,
                Direction::Right => current_pos.x += 1,
                Direction::Up => current_pos.y += 1,
                Direction::Down => current_pos.y -= 1,
            }
            self.contents.insert(current_pos);
        }
    }

    fn find_bounds(&self) -> ((isize, isize), (isize, isize)) {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for pos in &self.contents {
            min_x = cmp::min(min_x, pos.x);
            max_x = cmp::max(max_x, pos.x);
            min_y = cmp::min(min_y, pos.y);
            max_y = cmp::max(max_y, pos.y);
        }
        ((min_x, max_x), (min_y, max_y))
    }
}

fn fancy_print(repr: Vec<Vec<char>>) {
    for row in repr {
        println!("{:?}", row)
    }
}
