use itertools::Itertools;
use petgraph::{
    algo::dijkstra, graph::DiGraph, prelude::NodeIndex, visit::EdgeRef, Direction::Outgoing,
};
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2020/day_7.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut color_to_node = FxHashMap::default();
    let mut graph = DiGraph::<String, u32>::new();
    for line in LINES.lines() {
        let line = line.split(' ').collect_vec();
        let current_color = format!("{}{}", line[0], line[1]);
        let current_color_node = if let Some(node) = color_to_node.get(&current_color.clone()) {
            *node
        } else {
            graph.add_node(current_color.clone())
        };
        color_to_node.insert(current_color.clone(), current_color_node);
        if line[4] != "no" {
            for i in (5..line.len()).step_by(4) {
                let count = line[i - 1].parse::<u32>().unwrap();
                let color = format!("{}{}", line[i], line[i + 1]);
                let color_node = if let Some(node) = color_to_node.get(&color.clone()) {
                    *node
                } else {
                    graph.add_node(color.clone())
                };
                color_to_node.insert(color.clone(), color_node);
                graph.add_edge(current_color_node, color_node, count);
            }
        }
    }
    let shiny_gold_node = *color_to_node.get("shinygold").unwrap();
    (
        solve01(shiny_gold_node, &graph),
        solve02(shiny_gold_node, &graph),
    )
}

fn solve01(shiny_gold_node: NodeIndex, graph: &DiGraph<String, u32>) -> (usize, Duration) {
    let now = Instant::now();
    let reachable_from_shiny_gold = dijkstra(graph, shiny_gold_node, None, |_| 1).len() - 1;
    (reachable_from_shiny_gold, now.elapsed())
}

fn solve02(shiny_gold_node: NodeIndex, graph: &DiGraph<String, u32>) -> (usize, Duration) {
    let now = Instant::now();
    let res = count_bags(graph, shiny_gold_node);
    (res, now.elapsed())
}

fn count_bags(graph: &DiGraph<String, u32>, node: NodeIndex) -> usize {
    graph
        .edges_directed(node, Outgoing)
        .map(|edge| {
            let bag_count = *edge.weight() as usize;
            bag_count * (1 + count_bags(graph, edge.target()))
        })
        .sum()
}
