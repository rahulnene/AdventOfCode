use core::num;
use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2018/day_8.txt");
    let mut numbers = lines
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let nodes = parse_node(&mut numbers);
    (solve01(&nodes), solve02(&nodes))
}

fn solve01(nodes: &Node) -> (usize, Duration) {
    let now = Instant::now();
    (nodes.sum_metadata(), now.elapsed())
}

fn solve02(nodes: &Node) -> (usize, Duration) {
    let now = Instant::now();
    (value(&nodes), now.elapsed())
}

#[derive(Debug, Clone)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(num_child: usize, num_metadata: usize) -> Self {
        Self {
            children: Vec::with_capacity(num_child),
            metadata: Vec::with_capacity(num_metadata),
        }
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn sum_metadata(&self) -> usize {
        let mut ans = self.metadata.iter().sum::<usize>();
        for child in &self.children {
            ans += child.sum_metadata();
        }
        ans
    }
}

fn parse_node(numbers: &mut Vec<usize>) -> Node {
    let num_child = numbers.remove(0);
    let num_metadata = numbers.remove(0);
    let mut node = Node::new(num_child, num_metadata);
    for _ in 0..num_child {
        node.children.push(parse_node(numbers));
    }
    for _ in 0..num_metadata {
        node.metadata.push(numbers.remove(0));
    }
    node
}

fn value(node: &Node) -> usize {
    if !node.has_children() {
        node.metadata.iter().sum::<usize>()
    } else {
        let mut ans = 0;
        for &index in &node.metadata {
            if index == 0 {
                continue;
            }
            if let Some(child) = node.children.get(index - 1) {
                ans += value(child);
            }
        }
        ans
    }
}
