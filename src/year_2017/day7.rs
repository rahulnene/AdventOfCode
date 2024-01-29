use std::{
    ops::SubAssign,
    time::{Duration, Instant},
};

use itertools::Itertools;
use petgraph::{
    graph::{Node, NodeIndex},
    Directed,
    Direction::{Incoming, Outgoing},
    Graph,
};
use rustc_hash::FxHashMap;
pub fn solution() -> ((String, Duration), (isize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_7.txt");

    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (String, Duration) {
    let now = Instant::now();
    let programs: Vec<Program> = lines.lines().map(Program::from_str).collect();
    let mut program_weight_to_index = FxHashMap::default();
    let mut program_name_to_weight = FxHashMap::default();
    let mut graph = Graph::<usize, ()>::new();
    for program in programs.iter() {
        let index = graph.add_node(program.weight.clone());
        program_weight_to_index.insert(program.name.clone(), index);
    }
    for line in lines.lines() {
        let mut split = line.split(" -> ");
        let name_weight = split.next().unwrap();
        let name_weight_split: Vec<&str> = name_weight.split(" ").collect();
        let name = name_weight_split[0];
        let weight = name_weight_split[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .parse::<usize>()
            .unwrap();
        program_name_to_weight.insert(name.to_owned(), weight);
        let index = program_weight_to_index.get(name).unwrap();
        if let Some(children) = split.next() {
            let children: Vec<&str> = children.split(", ").collect();
            for child in children {
                let child_index = program_weight_to_index.get(child).unwrap();
                graph.add_edge(*index, *child_index, ());
            }
        }
    }
    let root = graph
        .node_indices()
        .find(|node| {
            graph
                .neighbors_directed(*node, petgraph::Direction::Incoming)
                .count()
                == 0
        })
        .unwrap();
    let root_weight = graph.node_weight(root).unwrap();
    let root_name = program_name_to_weight
        .iter()
        .filter(|(k, v)| *v == root_weight)
        .next()
        .unwrap();
    (root_name.0.to_owned(), now.elapsed())
}

fn solve02(lines: &str) -> (isize, Duration) {
    let now = Instant::now();
    let programs: Vec<Program> = lines.lines().map(Program::from_str).collect();
    let mut program_weight_to_index = FxHashMap::default();
    let mut program_name_to_weight = FxHashMap::default();
    let mut graph = Graph::<usize, ()>::new();
    for program in programs.iter() {
        let index = graph.add_node(program.weight.clone());
        program_weight_to_index.insert(program.name.clone(), index);
    }
    for line in lines.lines() {
        let mut split = line.split(" -> ");
        let name_weight = split.next().unwrap();
        let name_weight_split: Vec<&str> = name_weight.split(" ").collect();
        let name = name_weight_split[0];
        let weight = name_weight_split[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .parse::<usize>()
            .unwrap();
        program_name_to_weight.insert(name.to_owned(), weight);
        let index = program_weight_to_index.get(name).unwrap();
        if let Some(children) = split.next() {
            let children: Vec<&str> = children.split(", ").collect();
            for child in children {
                let child_index = program_weight_to_index.get(child).unwrap();
                graph.add_edge(*index, *child_index, ());
            }
        }
    }
    let root = graph
        .node_indices()
        .find(|node| {
            graph
                .neighbors_directed(*node, petgraph::Direction::Incoming)
                .count()
                == 0
        })
        .unwrap();
    let bad_node = find_faulty_child(&graph, root);
    let bad_node_parent = graph.neighbors_directed(bad_node, Incoming).next().unwrap();
    let mut sibling_weights = calculate_weight_of_children(&graph, bad_node_parent);
    // dbg!(sibling_weights);
    let c = sibling_weights.iter().counts_by(|w| w.1);
    let c = c.iter().sorted_by_key(|(k, v)| **v).collect_vec();
    dbg!(&c);
    let overweight = *c[0].0 as isize - *c[1].0 as isize;
    dbg!(overweight);
    let old_weight = *graph.node_weight(bad_node).unwrap();
    dbg!(old_weight);
    let ans = old_weight as isize - overweight;
    (ans, now.elapsed())
}

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: usize,
}

impl Program {
    fn new(name: &str, weight: usize) -> Self {
        Self {
            name: name.to_string(),
            weight,
        }
    }

    fn from_str(s: &str) -> Self {
        let mut split = s.split(" -> ");
        let name_weight = split.next().unwrap();
        let name_weight_split: Vec<&str> = name_weight.split(" ").collect();
        let name = name_weight_split[0];
        let weight = name_weight_split[1]
            .trim_start_matches('(')
            .trim_end_matches(')')
            .parse::<usize>()
            .unwrap();
        Self::new(name, weight)
    }
}

fn calculate_total_node_weight(
    original_graph: &Graph<usize, (), Directed>,
    node: NodeIndex<u32>,
) -> usize {
    let children = original_graph
        .neighbors_directed(node, Outgoing)
        .collect_vec();
    let ans = if children.is_empty() {
        let weight = *original_graph.node_weight(node).unwrap();
        weight
    } else {
        let ans: usize = children
            .iter()
            .map(|n| calculate_total_node_weight(original_graph, *n))
            .sum();
        ans + original_graph.node_weight(node).unwrap()
    };
    ans
}

fn calculate_weight_of_children(
    original_graph: &Graph<usize, (), Directed>,
    node: NodeIndex<u32>,
) -> Vec<(NodeIndex, usize)> {
    let children = original_graph
        .neighbors_directed(node, Outgoing)
        .collect_vec();
    children
        .iter()
        .map(|n| (*n, calculate_total_node_weight(original_graph, *n)))
        .collect_vec()
}

fn find_faulty_child(graph: &Graph<usize, (), Directed>, node: NodeIndex) -> NodeIndex {
    let children = calculate_weight_of_children(graph, node);
    if children.is_empty() {
        return node;
    }
    if let Some(bad) = find_odd_one_out(children) {
        bad
    } else {
        node
    }
}

fn find_odd_one_out(vec: Vec<(NodeIndex, usize)>) -> Option<NodeIndex> {
    let mut xor = 0;
    let mut temp = NodeIndex::from(0);
    for &(n, w) in vec.iter() {
        xor ^= w;
        temp = n;
    }
    if xor != 0 {
        Some(temp)
    } else {
        None
    }
}
