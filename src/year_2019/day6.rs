use fxhash::FxHashMap;
use petgraph::{algo::dijkstra, graph::NodeIndex};
use petgraph::{Graph, Undirected};
use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2019/day_6.txt");
    let mut system = Graph::<&str, &str, Undirected>::new_undirected();
    let mut name_to_index = FxHashMap::<&str, NodeIndex>::default();
    for line in lines.lines() {
        let mut iter = line.split(')');
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let ind_a = if name_to_index.contains_key(a) {
            *name_to_index.get(a).unwrap()
        } else {
            let ind = system.add_node(a);
            name_to_index.insert(a, ind);
            ind
        };
        let ind_b = if name_to_index.contains_key(b) {
            *name_to_index.get(b).unwrap()
        } else {
            let ind = system.add_node(b);
            name_to_index.insert(b, ind);
            ind
        };
        system.extend_with_edges(&[(ind_a, ind_b)]);
    }
    (
        solve01(&system, &name_to_index),
        solve02(&system, &name_to_index),
    )
}

fn solve01(
    system: &Graph<&str, &str, Undirected>,
    name_to_index: &FxHashMap<&str, NodeIndex>,
) -> (usize, Duration) {
    let now = Instant::now();

    let node_map = dijkstra(&system, *name_to_index.get("COM").unwrap(), None, |_| 1);
    let ans = node_map.values().sum::<usize>();
    (ans, now.elapsed())
}

fn solve02(
    system: &Graph<&str, &str, Undirected>,
    name_to_index: &FxHashMap<&str, NodeIndex>,
) -> (usize, Duration) {
    let now = Instant::now();

    let parent_of_you = system
        .neighbors_undirected(*name_to_index.get("YOU").unwrap())
        .next()
        .unwrap();
    let parent_of_san = system
        .neighbors_undirected(*name_to_index.get("SAN").unwrap())
        .next()
        .unwrap();
    let node_map = dijkstra(&system, parent_of_you, Some(parent_of_san), |_| 1);
    let ans = *node_map.get(&parent_of_san).unwrap();
    (ans, now.elapsed())
}
