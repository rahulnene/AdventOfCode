use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::time::{Duration, Instant};

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let line = include_str!("../../problem_inputs_2016/day_1.txt");
    let now = Instant::now();
    let mut human = Human::new();
    let path_iter = line
        .split(", ")
        .map(Instruction::parse)
        .filter_map(|instr| human.follow(instr))
        .collect_vec();
    let ans1 = (human.pos.0.abs() + human.pos.1.abs()) as usize;
    (
        (ans1, now.elapsed()),
        (*path_iter.first().unwrap(), now.elapsed()),
    )
}

#[derive(Debug, Clone, Copy)]

enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    turn: Turn,
    steps: usize,
}
impl Instruction {
    fn parse(s: &str) -> Self {
        let turn = match s.chars().nth(0).unwrap() {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => panic!("Invalid turn"),
        };
        let steps = s[1..].parse::<usize>().unwrap();
        Self { turn, steps }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]

struct Human {
    pos: (isize, isize),
    dir: Direction,
    visited_pos: FxHashSet<(isize, isize)>,
}

impl Human {
    fn new() -> Self {
        Self {
            pos: (0, 0),
            dir: Direction::North,
            visited_pos: FxHashSet::default(),
        }
    }

    fn follow(&mut self, instr: Instruction) -> Option<usize> {
        let mut ans = None;
        match instr.turn {
            Turn::Left => match self.dir {
                Direction::North => self.dir = Direction::West,
                Direction::East => self.dir = Direction::North,
                Direction::South => self.dir = Direction::East,
                Direction::West => self.dir = Direction::South,
            },
            Turn::Right => match self.dir {
                Direction::North => self.dir = Direction::East,
                Direction::East => self.dir = Direction::South,
                Direction::South => self.dir = Direction::West,
                Direction::West => self.dir = Direction::North,
            },
        }
        for _ in 0..instr.steps {
            match self.dir {
                Direction::North => self.pos.1 += 1,
                Direction::East => self.pos.0 += 1,
                Direction::South => self.pos.1 -= 1,
                Direction::West => self.pos.0 -= 1,
            }
            let visited = !self.visited_pos.insert(self.pos);
            if visited {
                ans = Some((self.pos.0.abs() + self.pos.1.abs()) as usize);
            }
        }
        ans
    }
}
