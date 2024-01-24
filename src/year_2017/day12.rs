use petgraph::{
    algo::{connected_components, has_path_connecting},
    graph::{NodeIndex, UnGraph},
};
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_12.txt");
    let now = Instant::now();
    let mut node_to_index = FxHashMap::default();
    let mut graph = UnGraph::<usize, ()>::default();
    for line in lines.lines() {
        let mut split = line.split(" <-> ");
        let node = split.next().unwrap().parse::<usize>().unwrap();
        let node_index = *node_to_index
            .entry(node)
            .or_insert_with(|| graph.add_node(node));
        for neighbor in split.next().unwrap().split(", ") {
            let neighbor = neighbor.parse::<usize>().unwrap();
            let neighbor_index = *node_to_index
                .entry(neighbor)
                .or_insert_with(|| graph.add_node(neighbor));
            graph.add_edge(node_index, neighbor_index, ());
        }
    }
    let ans = graph
        .node_indices()
        .filter(|n| has_path_connecting(&graph, NodeIndex::from(0), *n, None))
        .count();
    (
        (ans, now.elapsed()),
        (connected_components(&graph), now.elapsed()),
    )
}
