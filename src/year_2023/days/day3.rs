use fxhash::FxHashSet;
use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2019/day_3.txt");
    match part {
        1 => solve(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve(lines: &str) -> usize {
    0
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Wire {
    points: FxHashSet<Point>,
}

impl Wire {
    fn new() -> Self {
        Wire {
            points: FxHashSet::default(),
        }
    }

    fn add_point(&mut self, point: Point) {
        self.points.insert(point);
    }

    fn contains(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    fn add_points_from_direction(&mut self, direction: &str) {
        let amount = direction[1..].parse::<i32>().unwrap();
        let direction = direction.chars().nth(0).unwrap();
    }
}
