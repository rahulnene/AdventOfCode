use std::{
    hash::{Hash, Hasher},
    time::{Duration, Instant},
};

use fxhash::FxHasher;
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2020/day_20.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let tiles: Vec<Tile> = lines.split("\r\n\r\n").map(|s| Tile::process(s)).collect();
    let mut edges = Vec::new();
    for edge in tiles.iter().map(|t| t.edges.iter()).flatten() {
        edges.push(edge.0);
        edges.push(edge.1);
    }
    let edge_counts = edges.iter().counts();
    let a = tiles
        .iter()
        .map(|t| {
            (
                t.id,
                t.edges
                    .iter()
                    .map(|e| edge_counts.get(&e.0).unwrap() - 1)
                    .sum::<usize>(),
            )
        })
        .filter(|s| s.1 == 2)
        .map(|s| s.0)
        .product();
    (a, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: usize,
    edges: Vec<(usize, usize)>, //forward and reverse edge hash
}

impl Tile {
    fn new(id: usize, edges: Vec<(usize, usize)>) -> Self {
        Self { id, edges }
    }

    fn process(s: &str) -> Self {
        let mut lines = s.lines();
        let id = lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .trim_end_matches(':')
            .parse::<usize>()
            .unwrap();
        let size = lines.clone().count();
        let mut image = vec![vec!['.'; size]; size];
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                image[y][x] = c;
            }
        }
        let mut edges = Vec::with_capacity(4);
        edges.push(edge_hash(&image[0]));
        edges.push(edge_hash(&image[size - 1]));
        edges.push(edge_hash(&image.iter().map(|v| v[0]).collect::<Vec<_>>()));
        edges.push(edge_hash(
            &image.iter().map(|v| v[size - 1]).collect::<Vec<_>>(),
        ));
        Self::new(id, edges)
    }
}

fn edge_hash(edge: &[char]) -> (usize, usize) {
    let mut h = FxHasher::default();
    let rev: &mut [char] = &mut edge.to_vec();
    edge.hash(&mut h);
    let fh = h.finish() as usize;
    let mut h = FxHasher::default();
    rev.reverse();
    rev.hash(&mut h);
    let rh = h.finish() as usize;
    (fh, rh)
}
