use fxhash::FxHashMap;
use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::DiGraph;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_7.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut seen_colors = FxHashMap::default();
    // Graph is a directed graph that connects holder bag from its contents
    let mut graph = DiGraph::<String, ()>::new();
    for line in lines.lines() {
        let line = line.split(' ').collect_vec();
        let current_color = format!("{}{}", line[0], line[1]);
        let current_color_node = if let Some(node) = seen_colors.get(&current_color.clone()) {
            *node
        } else {
            graph.add_node(current_color.clone())
        };
        seen_colors.insert(current_color, current_color_node);
        if line[4] != "no" {
            for i in (5..line.len()).step_by(4) {
                let color = format!("{}{}", line[i], line[i + 1]);
                let color_node = if let Some(node) = seen_colors.get(&color.clone()) {
                    *node
                } else {
                    graph.add_node(color.clone())
                };
                seen_colors.insert(color, color_node);
                graph.add_edge(color_node, current_color_node, ());
            }
        }
    }
    // dbg!(&graph);
    let res = dijkstra(&graph, *seen_colors.get("shinygold").unwrap(), None, |_| 1).len();
    res - 1
}

fn solve02(lines: &str) -> usize {
    let mut seen_colors = FxHashMap::default();
    // Graph is a directed graph that connects holder bag from its contents
    let mut graph = DiGraph::<String, ()>::new();
    for line in lines.lines() {
        let line = line.split(' ').collect_vec();
        let current_color = format!("{}{}", line[0], line[1]);
        let current_color_node = if let Some(node) = seen_colors.get(&current_color.clone()) {
            *node
        } else {
            graph.add_node(current_color.clone())
        };
        seen_colors.insert(current_color, current_color_node);
        if line[4] != "no" {
            for i in (5..line.len()).step_by(4) {
                let color = format!("{}{}", line[i], line[i + 1]);
                let color_node = if let Some(node) = seen_colors.get(&color.clone()) {
                    *node
                } else {
                    graph.add_node(color.clone())
                };
                seen_colors.insert(color, color_node);
                graph.add_edge(current_color_node, color_node, ());
            }
        }
    }
    // dbg!(&graph);
    let res = dijkstra(&graph, *seen_colors.get("shinygold").unwrap(), None, |_| 1).len();
    res - 1
}
