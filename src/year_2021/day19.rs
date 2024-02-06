use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    time::{Duration, Instant},
};

use itertools::Itertools;
use petgraph::{Graph, Undirected};
use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2021/day_19_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut scanners = LINES
        .split("\r\n\r\n")
        .map(|s| {
            let mut lines = s.lines();
            let id = lines
                .next()
                .unwrap()
                .split_whitespace()
                .nth(2)
                .unwrap()
                .parse()
                .unwrap();
            let detected = lines
                .map(|l| {
                    let parts = l.split(": ").next().unwrap();
                    Coordinate::from_str(parts)
                })
                .collect();
            Scanner::new(id, detected)
        })
        .collect_vec();

    dbg!(scanners);
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn rotate_x(&self, amount: isize) -> Self {
        match amount.signum() {
            0 => return *self,
            1 => return Self::new(self.x, -self.z, self.y),
            -1 => return Self::new(self.x, self.z, -self.y),
            _ => unreachable!(),
        }
    }
    fn rotate_y(&self, amount: isize) -> Self {
        match amount.signum() {
            0 => return *self,
            1 => return Self::new(-self.z, self.y, self.x),
            -1 => return Self::new(self.z, self.y, -self.x),
            _ => unreachable!(),
        }
    }
    fn rotate_z(&self, amount: isize) -> Self {
        match amount.signum() {
            0 => return *self,
            1 => return Self::new(-self.y, self.x, self.z),
            -1 => return Self::new(self.y, -self.x, self.z),
            _ => unreachable!(),
        }
    }

    fn from_str(s: &str) -> Self {
        let mut parts = s.split(",");
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Self { x, y, z }
    }
}
#[derive(Clone, Hash, PartialEq, Eq)]
struct Scanner {
    id: usize,
    detected: Vec<Coordinate>,
}

impl Scanner {
    fn new(id: usize, detected: Vec<Coordinate>) -> Self {
        Self { id, detected }
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id.to_string())?;
        Ok(())
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({},{}) ", self.x, self.y))?;
        Ok(())
    }
}

impl Debug for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Scanner {")?;
        f.write_str("id: ")?;
        f.write_str(&self.id.to_string())?;
        f.write_str(", detected: ")?;
        for d in &self.detected {
            f.write_str(&d.to_string())?;
        }
        f.write_str("}")?;
        Ok(())
    }
}

impl Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Coordinate")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
