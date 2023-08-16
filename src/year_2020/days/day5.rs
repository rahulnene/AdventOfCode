use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

pub fn solution(part: u8) -> isize {
    let lines = include_str!("../../../problem_inputs_2021/day_5.txt");
    match part {
        1 => solve(lines, false),
        2 => solve(lines, true),
        _ => 1,
    }
}

fn solve(lines: &str, part2: bool) -> isize {
    let mut map = Map::new();
    for line in lines.lines() {
        let (a, b) = line.split(" -> ").collect_tuple().unwrap();
        map.add_line(Point::from_str(a), Point::from_str(b), part2);
    }
    map.map.values().filter(|&v| v > &1).count() as isize
}

#[derive(Clone)]
struct Map {
    map: HashMap<Point, isize>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn from_str(s: &str) -> Self {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        Self::new(x, y)
    }
}

impl Map {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn add_point(&mut self, point: Point) {
        let key = &point;
        self.map.get_mut(key).map(|v| *v += 1).unwrap_or_else(|| {
            self.map.insert(*key, 1);
        });
    }

    fn add_line(&mut self, point1: Point, point2: Point, diagonals: bool) {
        let (point1, point2) = (point1.min(point2), point1.max(point2));
        if point1.x == point2.x {
            for y in point1.y..=point2.y {
                self.add_point(Point::new(point1.x, y));
            }
        } else if point1.y == point2.y {
            for x in point1.x..=point2.x {
                self.add_point(Point::new(x, point1.y));
            }
        } else if diagonals {
            let b: isize = point2.y - point1.y;
            for x in point1.x..=point2.x {
                let y = point1.y - (b * (x - point1.x)) / (point1.x - point2.x);
                self.add_point(Point::new(x, y));
            }
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let x_max = self.map.clone().into_keys().max_by_key(|p| p.x).unwrap().x;
        let y_max = self.map.clone().into_keys().max_by_key(|p| p.y).unwrap().y;
        for y in 0..=y_max {
            for x in 0..=x_max {
                let point = Point::new(x, y);
                let value = self.map.get(&point).unwrap_or(&0);
                match value {
                    0 => write!(f, ".")?,
                    _ => write!(f, "{value}")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)?;
        Ok(())
    }
}
