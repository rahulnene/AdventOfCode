use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const LINES: &str = include_str!("../../problem_inputs_2022/day_23_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut dir_consider = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    let mut layout = Layout::from_str(LINES);
    for _ in 0..5 {
        layout.consider(&mut dir_consider);
        layout.update();
    }
    dbg!(layout.get_bounds());
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

type Position = (isize, isize);
#[derive(Debug, PartialEq, Eq, Clone)]
struct Layout {
    current: FxHashMap<Position, Option<Elf>>,
    next: FxHashMap<Position, usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Elf {
    direction: Direction,
    to_move: bool,
}

impl Elf {
    fn new(direction: Direction) -> Self {
        Self {
            direction,
            to_move: false,
        }
    }

    fn with_direction(&self, direction: Direction) -> Self {
        Self {
            direction,
            to_move: self.to_move,
        }
    }
    fn will_move(&self) -> Self {
        Self {
            direction: self.direction,
            to_move: true,
        }
    }
}

impl Layout {
    fn new() -> Self {
        Self {
            current: FxHashMap::default(),
            next: FxHashMap::default(),
        }
    }

    fn get_bounds(&self) -> (isize, isize) {
        let max_x = self.current.iter().map(|(a, _)| a.0).max().unwrap();
        let max_y = self.current.iter().map(|(a, _)| a.1).max().unwrap();
        let min_x = self.current.iter().map(|(a, _)| a.0).min().unwrap();
        let min_y = self.current.iter().map(|(a, _)| a.1).min().unwrap();
        (max_x - min_x, max_y - min_y)
    }

    fn from_str(s: &str) -> Self {
        let mut layout = Self::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x as isize, y as isize);
                if c == '#' {
                    layout.current.insert(pos, Some(Elf::default()));
                } else {
                    layout.current.insert(pos, None);
                }
            }
        }
        layout
    }
    fn exists(&self, pos: Position) -> bool {
        self.current.get(&pos).is_some()
    }

    fn any_elves(&self, pos: Position, dir: Direction) -> bool {
        match dir {
            Direction::North => {
                self.exists((pos.0, pos.1 - 1))
                    || self.exists((pos.0 - 1, pos.1 - 1))
                    || self.exists((pos.0 + 1, pos.1 - 1))
            }
            Direction::East => {
                self.exists((pos.0 + 1, pos.1))
                    || self.exists((pos.0 + 1, pos.1 - 1))
                    || self.exists((pos.0 + 1, pos.1 + 1))
            }
            Direction::South => {
                self.exists((pos.0, pos.1 + 1))
                    || self.exists((pos.0 - 1, pos.1 + 1))
                    || self.exists((pos.0 + 1, pos.1 + 1))
            }
            Direction::West => {
                self.exists((pos.0 - 1, pos.1))
                    || self.exists((pos.0 - 1, pos.1 - 1))
                    || self.exists((pos.0 - 1, pos.1 + 1))
            }
        }
    }

    fn consider(&mut self, dir_consider: &mut VecDeque<Direction>) {
        let elf_positions = self
            .current
            .iter()
            .to_owned()
            .filter(|(_, &b)| b.is_some())
            .map(|a| *a.0)
            .to_owned()
            .collect_vec();
        for elf_position in elf_positions.iter() {
            for dir in dir_consider.iter() {
                if !self.any_elves(*elf_position, *dir) {
                    match dir {
                        Direction::North => {
                            self.next
                                .entry((elf_position.0, elf_position.1 - 1))
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                            self.current
                                .entry((elf_position.0, elf_position.1 - 1))
                                .and_modify(|e| *e = Some(Elf::new(*dir)));
                        }
                        Direction::East => {
                            self.next
                                .entry((elf_position.0 + 1, elf_position.1))
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                            self.current
                                .entry((elf_position.0 + 1, elf_position.1))
                                .and_modify(|e| *e = Some(Elf::new(*dir)));
                        }
                        Direction::South => {
                            self.next
                                .entry((elf_position.0, elf_position.1 + 1))
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                            self.current
                                .entry((elf_position.0, elf_position.1 + 1))
                                .and_modify(|e| *e = Some(Elf::new(*dir)));
                        }
                        Direction::West => {
                            self.next
                                .entry((elf_position.0 - 1, elf_position.1))
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                            self.current
                                .entry((elf_position.0 - 1, elf_position.1))
                                .and_modify(|e| *e = Some(Elf::new(*dir)));
                        }
                    }
                }
            }
        }
        dir_consider.rotate_left(1);
    }

    fn update(&mut self) {
        let elf_positions = self
            .current
            .iter()
            .to_owned()
            .filter(|(_, &b)| b.is_some())
            .map(|a| *a.0)
            .to_owned()
            .collect_vec();
        for elf_position in elf_positions.iter() {
            match self.current.get(elf_position).unwrap().unwrap().direction {
                Direction::North => {
                    let considered = (elf_position.0, elf_position.1 - 1);
                    if self.next.get(&considered).unwrap_or(&1) > &1 {
                        continue;
                    } else {
                        self.current
                            .entry(considered)
                            .and_modify(|e| *e = Some(Elf::new(Direction::North)));
                        self.current.entry(*elf_position).and_modify(|e| *e = None);
                    }
                }
                Direction::East => {
                    let considered = (elf_position.0 + 1, elf_position.1);
                    if self.next.get(&considered).unwrap() > &1 {
                        continue;
                    } else {
                        self.current
                            .entry(considered)
                            .and_modify(|e| *e = Some(Elf::new(Direction::East)));
                        self.current.entry(*elf_position).and_modify(|e| *e = None);
                    }
                }
                Direction::South => {
                    let considered = (elf_position.0, elf_position.1 + 1);
                    if self.next.get(&considered).unwrap() > &1 {
                        continue;
                    } else {
                        self.current
                            .entry(considered)
                            .and_modify(|e| *e = Some(Elf::new(Direction::South)));
                        self.current.entry(*elf_position).and_modify(|e| *e = None);
                    }
                }
                Direction::West => {
                    let considered = (elf_position.0 - 1, elf_position.1);
                    if self.next.get(&considered).unwrap() > &1 {
                        continue;
                    } else {
                        self.current
                            .entry(considered)
                            .and_modify(|e| *e = Some(Elf::new(Direction::West)));
                        self.current.entry(*elf_position).and_modify(|e| *e = None);
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}
