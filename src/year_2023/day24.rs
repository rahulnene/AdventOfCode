use std::fmt::Debug;

use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!(r"..\..\..\problem_inputs_2023\day_24_test.txt");
    match part {
        1 => solve01(lines),
        // 2 => solve(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut rays: Vec<Ray> = vec![];
    for line in lines.lines() {
        let (pos, dir): (Vector, Vector) =
            line.split(" @ ").map(Vector::new).collect_tuple().unwrap();
        rays.push(Ray { pos, dir });
    }
    dbg!(intersection_point(rays[0], rays[1]));
    dbg!(rays);

    0
}

fn solve02(lines: &str) -> usize {
    0
}

type Point = (isize, isize, isize);

#[derive(Clone, Copy, Default)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone, Copy, Default)]
struct Ray {
    pos: Vector,
    dir: Vector,
}

impl Vector {
    fn new(str: &str) -> Vector {
        let pos: (isize, isize, isize) = str
            .split(',')
            .map(|f| f.trim().parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();

        Vector {
            x: pos.0,
            y: pos.1,
            z: pos.2,
        }
    }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<{} {} {}>", self.x, self.y, self.z))
    }
}

impl Debug for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ray")
            .field("pos", &self.pos)
            .field("dir", &self.dir)
            .finish()
    }
}

fn intersection_point(r1: Ray, r2: Ray) -> Option<Point> {
    todo!()
}
