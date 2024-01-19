use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2019/day_3.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let wires: (Wire, Wire) = lines
        .trim()
        .split('\n')
        .map(|s| Wire::from_str(s))
        .collect_tuple()
        .unwrap();
    let ans = find_common_points(&wires.0, &wires.1)
        .iter()
        .map(|p| manhattan_distance(**p, (0, 0)))
        .min()
        .unwrap();

    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let wires: (Wire, Wire) = lines
        .trim()
        .split('\n')
        .map(|s| Wire::from_str(s))
        .collect_tuple()
        .unwrap();
    let intersections = find_common_points(&wires.0, &wires.1);
    let timings_wire_1 = intersections
        .iter()
        .map(|p| (**p, wires.0.points.iter().position(|x| x == *p).unwrap()));
    let timings_wire_2 = intersections
        .iter()
        .map(|p| (**p, wires.1.points.iter().position(|x| x == *p).unwrap()));
    let ans = timings_wire_1
        .zip(timings_wire_2)
        .map(|s| s.0 .1 + s.1 .1)
        .min()
        .unwrap();

    (ans, now.elapsed())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Wire {
    points: Vec<Point>,
}

impl Wire {
    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }
    fn from_str(s: &str) -> Self {
        let now = Instant::now();
        let mut wire = Wire { points: Vec::new() };
        let mut current_point = (0, 0);
        wire.add_point(current_point);
        let instrs = s.trim().split(',').map(Instruction::from_str);
        for instr in instrs {
            match instr {
                Instruction {
                    direction: Direction::Up,
                    distance,
                } => {
                    for _ in 0..distance {
                        current_point.1 += 1;
                        wire.add_point(current_point);
                    }
                }
                Instruction {
                    direction: Direction::Down,
                    distance,
                } => {
                    for _ in 0..distance {
                        current_point.1 -= 1;
                        wire.add_point(current_point);
                    }
                }
                Instruction {
                    direction: Direction::Left,
                    distance,
                } => {
                    for _ in 0..distance {
                        current_point.0 -= 1;
                        wire.add_point(current_point);
                    }
                }
                Instruction {
                    direction: Direction::Right,
                    distance,
                } => {
                    for _ in 0..distance {
                        current_point.0 += 1;
                        wire.add_point(current_point);
                    }
                }
            }
        }
        println!("Wire::from_str took {:?} to create.", now.elapsed());
        wire
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let direction = Direction::from_char(s.chars().next().unwrap());
        let distance = s[1..].parse::<usize>().unwrap();
        Instruction {
            direction,
            distance,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction char: {}", c),
        }
    }
}

fn find_common_points<'a>(w1: &'a Wire, w2: &'a Wire) -> Vec<&'a Point> {
    w1.points
        .iter()
        .filter(|p| w2.points.contains(p) && **p != (0, 0))
        .collect_vec()
}

type Point = (isize, isize);
fn manhattan_distance(p1: Point, p2: Point) -> usize {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as usize
}
