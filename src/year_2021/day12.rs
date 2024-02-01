use petgraph::{graph::Graph, graph::NodeIndex, Undirected};
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2021/day_12.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let mut graph = Graph::<usize, (), Undirected>::default();
    let mut name_to_index = FxHashMap::default();
    let mut index_to_name = FxHashMap::default();
    for line in LINES.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let ind_a = *name_to_index.entry(a).or_insert_with(|| graph.add_node(0));
        let ind_b = *name_to_index.entry(b).or_insert_with(|| graph.add_node(0));
        index_to_name.insert(ind_a, a);
        index_to_name.insert(ind_b, b);
        graph.update_edge(ind_a, ind_b, ());
    }
    let start_ind = *name_to_index.get("start").unwrap();
    let end_ind = *name_to_index.get("end").unwrap();
    (
        solve(start_ind, end_ind, &graph, &index_to_name, find_paths_p1),
        solve(start_ind, end_ind, &graph, &index_to_name, find_paths_p2),
    )
}

fn solve(
    start_ind: NodeIndex,
    end_ind: NodeIndex,
    graph: &Graph<usize, (), Undirected>,
    index_to_name: &FxHashMap<NodeIndex, &str>,
    path_fn: fn(
        &Graph<usize, (), Undirected>,
        NodeIndex,
        NodeIndex,
        &Vec<NodeIndex>,
        &mut FxHashSet<Vec<NodeIndex>>,
        &FxHashMap<NodeIndex, &str>,
        Option<bool>,
    ),
) -> (usize, Duration) {
    let now = Instant::now();
    let mut paths = FxHashSet::default();
    path_fn(
        &graph,
        start_ind,
        end_ind,
        &Vec::new(),
        &mut paths,
        &index_to_name,
        Some(false),
    );
    (paths.len(), now.elapsed())
}

fn find_paths_p1(
    graph: &Graph<usize, (), Undirected>,
    start_ind: NodeIndex,
    end_ind: NodeIndex,
    path: &Vec<NodeIndex>,
    paths: &mut FxHashSet<Vec<NodeIndex>>,
    index_to_name: &FxHashMap<NodeIndex, &str>,
    has_visited_small_twice: Option<bool>,
) {
    let mut path = path.clone();
    path.push(start_ind);

    if start_ind == end_ind {
        paths.insert(path);
    } else {
        for neighbor in graph.neighbors(start_ind) {
            let is_upper = index_to_name
                .get(&neighbor)
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .is_uppercase();
            match (is_upper, path.contains(&neighbor)) {
                (true, _) => {
                    find_paths_p1(
                        graph,
                        neighbor,
                        end_ind,
                        &path,
                        paths,
                        index_to_name,
                        has_visited_small_twice,
                    );
                }
                (false, true) => {}
                (false, false) => {
                    find_paths_p1(
                        graph,
                        neighbor,
                        end_ind,
                        path.as_ref(),
                        paths,
                        index_to_name,
                        has_visited_small_twice,
                    );
                }
            }
        }
    }
}
fn find_paths_p2(
    graph: &Graph<usize, (), Undirected>,
    start_ind: NodeIndex,
    end_ind: NodeIndex,
    path: &Vec<NodeIndex>,
    paths: &mut FxHashSet<Vec<NodeIndex>>,
    index_to_name: &FxHashMap<NodeIndex, &str>,
    has_visited_small_twice: Option<bool>,
) {
    let has_visited_small_twice = has_visited_small_twice.unwrap();
    let mut path = path.clone();
    path.push(start_ind);

    if start_ind == end_ind {
        paths.insert(path);
    } else {
        for neighbor in graph.neighbors(start_ind) {
            if index_to_name.get(&neighbor).unwrap() != &"start" {
                let is_upper = index_to_name
                    .get(&neighbor)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .is_uppercase();
                if is_upper {
                    find_paths_p2(
                        graph,
                        neighbor,
                        end_ind,
                        path.as_ref(),
                        paths,
                        index_to_name,
                        Some(has_visited_small_twice),
                    );
                } else {
                    if !path.contains(&neighbor) {
                        find_paths_p2(
                            graph,
                            neighbor,
                            end_ind,
                            path.as_ref(),
                            paths,
                            index_to_name,
                            Some(has_visited_small_twice),
                        );
                    } else if !has_visited_small_twice {
                        find_paths_p2(
                            graph,
                            neighbor,
                            end_ind,
                            path.as_ref(),
                            paths,
                            index_to_name,
                            Some(true),
                        );
                    }
                }
            }
        }
    }
}
