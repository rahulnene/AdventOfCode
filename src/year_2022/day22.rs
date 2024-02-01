use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::{
    ops::{Add, Sub},
    time::{Duration, Instant},
};

const LINES: &str = include_str!("../../problem_inputs_2022/day_22_test.txt");
lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\d+|\D)").unwrap();
}
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let (map_str, instrs) = LINES.split_once("\r\n\r\n").unwrap();
    let instrs = Instruction::from_str(instrs);
    let mut adventure = Adventure::new(map_str);
    for instr in instrs {
        adventure.follow_instruction(&instr);
        dbg!(instr, adventure.direction);
    }
    let ans = score(&adventure);

    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn score(adv: &Adventure) -> usize {
    let x = adv.position.0.val + 1;
    let y = adv.position.1.val + 1;
    let dir = match adv.direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    dbg!(x, y, dir);
    1000 * y + 4 * x + dir
}

type Position = (BoundedCoordinate, BoundedCoordinate);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct BoundedCoordinate {
    val: usize,
    max: usize,
}

impl BoundedCoordinate {
    fn new(val: usize, max: usize) -> Self {
        Self { val, max }
    }
}
impl Add for BoundedCoordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new((self.val + rhs.val) % self.max, self.max)
    }
}
impl Add<usize> for BoundedCoordinate {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new((self.val + rhs) % self.max, self.max)
    }
}
impl Sub for BoundedCoordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new((self.val + self.max - rhs.val) % self.max, self.max)
    }
}
impl Sub<usize> for BoundedCoordinate {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self::new((self.val + self.max - rhs) % self.max, self.max)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Adventure {
    map: FxHashMap<Position, MapContents>,
    position: Position,
    direction: Direction,
    bounds: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Adventure {
    fn new(map_str: &str) -> Self {
        let max_x = map_str.lines().next().unwrap().len();
        let max_y = map_str.lines().count();
        let mut map = FxHashMap::default();
        for (y, line) in map_str.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = BoundedCoordinate::new(x, max_x);
                let y = BoundedCoordinate::new(y, max_y);
                map.insert((x, y), MapContents::from_char(c));
            }
        }
        for y in 0.. {
            for x in 0.. {
                let x = BoundedCoordinate::new(x, max_x);
                let y = BoundedCoordinate::new(y, max_y);
                if map.get(&(x, y)) == Some(&MapContents::Floor) {
                    return Self {
                        map,
                        position: (x, y),
                        direction: Direction::Right,
                        bounds: (max_x, max_y),
                    };
                }
            }
        }
        unreachable!("No floor found");
    }

    fn move_forward(&mut self) {
        let mut look_forward = match self.direction {
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        };
        while self.get(&look_forward) == Some(&MapContents::Blank) {
            look_forward = match self.direction {
                Direction::Up => (look_forward.0, look_forward.1 - 1),
                Direction::Down => (look_forward.0, look_forward.1 + 1),
                Direction::Left => (look_forward.0 - 1, look_forward.1),
                Direction::Right => (look_forward.0 + 1, look_forward.1),
            };
        }

        if self.get(&look_forward) == Some(&MapContents::Walls) {
            return;
        }
        if self.get(&look_forward) == Some(&MapContents::Floor) {
            self.position = look_forward;
            return;
        }
    }
    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn follow_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Move(n) => {
                for _ in 0..*n {
                    self.move_forward();
                }
            }
            Instruction::TurnLeft => self.turn_left(),
            Instruction::TurnRight => self.turn_right(),
        }
    }

    fn get(&self, pos: &Position) -> Option<&MapContents> {
        self.map.get(pos)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MapContents {
    Blank,
    Floor,
    Walls,
}

impl MapContents {
    fn from_char(c: char) -> Self {
        match c {
            '.' => MapContents::Floor,
            '#' => MapContents::Walls,
            _ => MapContents::Blank,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

impl Instruction {
    fn from_str(s: &str) -> Vec<Self> {
        REGEX
            .captures_iter(s)
            .map(|cap| cap[0].to_string())
            .map(|s| match s.as_str() {
                "L" => Instruction::TurnLeft,
                "R" => Instruction::TurnRight,
                _ => Instruction::Move(s.parse().unwrap()),
            })
            .collect()
    }
}
