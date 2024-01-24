use std::time::{Duration, Instant};

use rustc_hash::FxHashMap;
pub fn solution() -> ((String, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_19.txt");
    solve(lines)
}

fn solve(lines: &str) -> ((String, Duration), (usize, Duration)) {
    let now = Instant::now();
    let mut world = PacketWorld::new(lines);
    let ans = world.walk();
    ((ans.0, now.elapsed()), (ans.1, now.elapsed()))
}

#[derive(Debug, Clone)]
struct PacketWorld {
    map: FxHashMap<(isize, isize), MapContent>,
    path: Vec<char>,
    position: (isize, isize),
    direction: Direction,
    total_letters: usize,
    total_steps: usize,
}

impl PacketWorld {
    fn new(lines: &str) -> Self {
        let mut map = FxHashMap::default();
        let path = Vec::new();
        for (y, line) in lines.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let content = MapContent::from_char(c);
                map.insert((x as isize, y as isize), content);
            }
        }
        let max_x = map.keys().map(|(x, _)| x).max().unwrap();
        let max_y = map.keys().map(|(_, y)| y).max().unwrap();
        let start = map
            .iter()
            .find(|(pos, &content)| {
                content == MapContent::Path && {
                    pos.0 == 0 || pos.1 == 0 || pos.0 == *max_x || pos.1 == *max_y
                }
            })
            .map(|(pos, _)| *pos)
            .unwrap();
        let direction = match start {
            (0, _) => Direction::Right,
            (_, 0) => Direction::Down,
            (x, _) if x == *max_x => Direction::Left,
            (_, y) if y == *max_y => Direction::Up,
            _ => unreachable!(),
        };
        let total_letters = map
            .iter()
            .filter(|(_, &content)| content.is_letter())
            .count();
        Self {
            map,
            path,
            position: start,
            direction,
            total_letters,
            total_steps: 1,
        }
    }

    fn walk(&mut self) -> (String, usize) {
        while self.path.len() < self.total_letters {
            self.step();
            self.total_steps += 1;
        }
        (self.path.iter().collect(), self.total_steps)
    }

    fn step(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
        let content = self.map.get(&self.position).unwrap();
        match content {
            MapContent::Empty => unreachable!(),
            MapContent::Letter(c) => self.path.push(*c),
            MapContent::Path => {}
            MapContent::Corner => {
                self.direction = self.get_next_direction();
            }
        }
    }

    fn get_next_direction(&self) -> Direction {
        let (x, y) = self.position;
        let up = self.map.get(&(x, y - 1)).unwrap_or(&MapContent::Empty);
        let left = self.map.get(&(x - 1, y)).unwrap_or(&MapContent::Empty);
        match self.direction {
            Direction::Up | Direction::Down => {
                if left.is_letter() || *left == MapContent::Path {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Left | Direction::Right => {
                if up.is_letter() || *up == MapContent::Path {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapContent {
    Empty,
    Letter(char),
    Path,
    Corner,
}

impl MapContent {
    const fn from_char(c: char) -> Self {
        match c {
            ' ' => MapContent::Empty,
            '|' | '-' => MapContent::Path,
            '+' => MapContent::Corner,
            c => MapContent::Letter(c),
        }
    }

    const fn is_letter(self) -> bool {
        matches!(self, MapContent::Letter(_))
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
