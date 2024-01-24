use itertools::Itertools;
use rustc_hash::FxHashSet;

use std::{time::{Duration, Instant}, thread::sleep};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2018/day_10_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut grid = Grid::new();
    for line in lines.lines() {
        grid.points.insert(Point::from_str(line));
    }
    // let mut seconds = 0;
    while !grid.all_have_neighbors() {
        grid.update();
        grid.pprint();
        sleep(Duration::from_millis(1000));
    }
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Point {
    fn new(x: isize, y: isize, vx: isize, vy: isize) -> Self {
        Point { x, y, vx, vy }
    }

    fn from_str(line: &str) -> Self {
        let parts: Vec<&str> = line.split('>').collect();

        let position: Vec<&str> = parts[0][10..].split(',').collect();
        let velocity: Vec<&str> = parts[1][11..].split(',').collect();

        let x = position[0].trim().parse::<isize>().unwrap();
        let y = position[1].trim().parse::<isize>().unwrap();
        let vx = velocity[0].trim().parse::<isize>().unwrap();
        let vy = velocity[1].trim().parse::<isize>().unwrap();

        Point::new(x, y, vx, vy)
    }

    fn step(&self) -> Point {
        Point {
            x: self.x + self.vx,
            y: self.y + self.vy,
            vx: self.vx,
            vy: self.vy,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    points: FxHashSet<Point>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            points: FxHashSet::default(),
        }
    }

    fn update(&mut self) {
        let mut new_points = FxHashSet::default();
        for point in &self.points {
            new_points.insert(point.step());
        }
        assert_eq!(self.points.len(), new_points.len());
        self.points = new_points;
    }

    fn bounds(&self) -> ((isize, isize), (isize, isize)) {
        let points = self.points.iter().map(|p| (p.x, p.y)).collect_vec();
        (
            (
                points.iter().min_by_key(|p| p.0).unwrap().0,
                points.iter().max_by_key(|p| p.1).unwrap().0,
            ),
            (
                points.iter().min_by_key(|p| p.0).unwrap().1,
                points.iter().max_by_key(|p| p.1).unwrap().1,
            ),
        )
    }

    fn pprint(&self) {
        let ((minx, maxx), (miny, maxy)) = self.bounds();
        for y in miny..maxy {
            for x in minx..maxx {
                if self.points.iter().any(|p| p.x == x && p.y == y) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!("");
        }
    }

    fn all_have_neighbors(&self) -> bool {
        for point in &self.points {
            let x = point.x;
            let y = point.y;
            let neighbor_pos = [
                (x + 1, y + 1),
                (x + 1, y),
                (x + 1, y - 1),
                (x, y + 1),
                (x, y - 1),
                (x - 1, y + 1),
                (x - 1, y),
                (x - 1, y - 1),
            ];
            return !self
                .points
                .iter()
                .any(|p| !neighbor_pos.contains(&(p.x, p.y)));
        }
        false
    }
}
