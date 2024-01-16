use fxhash::FxHashMap;
use itertools::Itertools;
use petgraph::{visit::GetAdjacencyMatrix, Directed, Graph};
use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2015/day_9_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut graph: Graph<&str, usize, Directed> = Graph::new();
    let mut name_to_index = FxHashMap::default();
    for line in lines.lines() {
        let (from, to, dist) = parse_line(line);
        let from_node = if name_to_index.contains_key(from) {
            *name_to_index.get(from).unwrap()
        } else {
            let node_id = graph.add_node(from);
            name_to_index.insert(from, node_id);
            node_id
        };
        let to_node = if name_to_index.contains_key(to) {
            *name_to_index.get(to).unwrap()
        } else {
            let node_id = graph.add_node(from);
            name_to_index.insert(to, node_id);
            node_id
        };
        graph.add_edge(from_node, to_node, dist);
    }
    dbg!(&graph);
    let adj_mat = graph.adjacency_matrix();
    

    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn parse_line(line: &str) -> (&str, &str, usize) {
    let (from, _, to, _, dist) = line.split(' ').collect_tuple().unwrap();
    let dist = dist.parse::<usize>().unwrap();
    (from, to, dist)
}
