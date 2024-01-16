use fxhash::FxHashSet;
use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_6_test.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    // let mut graph = FxHashSet::default();
    let mut point_set = Vec::new();
    for line in lines.lines() {
        let new_point = Point::from_str(line);
        if true {
            point_set.push(new_point);
        }
    }
    0
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
    fn from_str(s: &str) -> Self {
        let (x_str, y_str) = s.split(' ').collect_tuple().unwrap();
        let binding = x_str.chars().filter(|c| c.is_digit(10)).collect::<String>();
        Point {
            x: binding.parse().unwrap(),
            y: y_str.parse().unwrap(),
        }
    }
}

fn manhattan(p1: Point, p2: Point) -> usize {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

fn check_if_outer(point: Point) -> bool {
    let topLeft = Point::new(0, 0);
    let topRight = Point::new(usize::MAX, 0);
    let bottom_left = Point::new(0, usize::MAX);
    let bottom_right = Point::new(usize::MAX, usize::MAX);
    
}
