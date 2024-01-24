use itertools::Itertools;
use petgraph::{graph::Graph, Directed, Direction::Incoming};
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2018/day_7_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut graph = Graph::<char, (), Directed>::new();
    let mut char_to_index = FxHashMap::default();
    for line in lines.lines() {
        let words = line.split_whitespace().collect_vec();
        let (base, next) = (
            words[1].chars().next().unwrap(),
            words[7].chars().next().unwrap(),
        );
        let (base_index, next_index) = (
            *char_to_index.entry(base).or_insert_with(|| {
                let index = graph.add_node(base);
                // graph.add_edge(index, index, ());
                index
            }),
            *char_to_index.entry(next).or_insert_with(|| {
                let index = graph.add_node(next);
                // graph.add_edge(index, index, ());
                index
            }),
        );
        graph.add_edge(base_index, next_index, ());
    }
    dbg!(&char_to_index);
    let mut current = char_to_index
        .iter()
        .filter(|(_, v)| graph.neighbors_directed(**v, Incoming).count() == 0)
        .map(|(k, _)| k)
        .copied()
        .sorted()
        .collect_vec();
    let total = char_to_index.len();
    dbg!(&current);
    loop {
        let mut visible_from_current = Vec::new();
        if current.len() == total {
            break;
        }

        for &node in &current {
            let mut neighbors = graph.neighbors_directed(
                *char_to_index.get(&node).unwrap(),
                petgraph::Direction::Outgoing,
            );
            while let Some(neighbor) = neighbors.next() {
                if neighbor != *char_to_index.get(&node).unwrap() {
                    visible_from_current.push(graph[neighbor]);
                }
            }
        }
        visible_from_current.sort();
        visible_from_current.dedup();
        let mut next = visible_from_current
            .iter()
            .filter(|&v| !current.contains(v));
        loop {
            let next_node = next.next().unwrap();
            if graph
                .neighbors_directed(*char_to_index.get(next_node).unwrap(), Incoming)
                .all(|v| current.contains(&graph[v]))
            {
                current.push(*next_node);
                break;
            }
        }
        println!("{}", current.iter().collect::<String>());
    }
    println!("{}", current.iter().collect::<String>());
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}
