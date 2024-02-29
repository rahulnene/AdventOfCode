use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    hash::{Hash, Hasher},
    rc::Rc,
    time::{Duration, Instant},
};

use itertools::Itertools;
use petgraph::graph::Node;
use rustc_hash::{FxHashMap, FxHasher};

const LINES: &str = include_str!("../../problem_inputs_2022/day_7.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone)]
enum NodeType {
    File(usize),
    Dir(usize),
}

impl NodeType {
    fn size(&self) -> usize {
        match self {
            NodeType::File(size) => *size,
            NodeType::Dir(size) => *size,
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    name: u64,
    children: Vec<NodeType>,
}

impl Directory {
    fn new(name: &str) -> Self {
        let mut hasher = FxHasher::default();
        hasher.write(&name.bytes().collect_vec());
        let name = hasher.finish();
        Self {
            name,
            children: Vec::new(),
        }
    }

    fn insert(&mut self, insertion: &NodeType) {
        self.children.push(insertion.clone());
    }

    fn size(&self) -> usize {
        self.children.iter().map(|c| c.size()).sum()
    }
}

#[derive(Debug, Clone)]
enum Command {
    Cd(String),
    Ls,
}
