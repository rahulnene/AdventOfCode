use itertools::Itertools;
use petgraph::{
    // algo::{self, k_shortest_path},
    algo,
    graph::{Graph, Node, UnGraph},
    stable_graph::NodeIndex,
    visit::{self, Bfs, Dfs},
    Undirected,
};
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2016/day_24_test.txt");

    let mut pos_to_index = FxHashMap::default();
    let mut pos_to_contents = FxHashMap::default();
    let mut maze = Vec::new();

    let mut graph = UnGraph::<MazeBlock, f64>::default();
    for (y, line) in lines.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let content = MazeBlock::from_char(c);
            row.push(MazeBlock::from_char(c));
            if c != '#' {
                pos_to_index.insert((x, y), graph.add_node(content));
            }
            pos_to_contents.insert((x, y), content);
        }
        maze.push(row);
    }
    for y in 1..maze.len() - 1 {
        for x in 1..maze[y].len() - 1 {
            if maze[y][x] == MazeBlock::Open {
                if maze[y - 1][x] != MazeBlock::Wall {
                    graph.add_edge(
                        *pos_to_index.get(&(x, y)).unwrap(),
                        *pos_to_index.get(&(x, y - 1)).unwrap(),
                        1.0,
                    );
                }
                if maze[y + 1][x] != MazeBlock::Wall {
                    graph.add_edge(
                        *pos_to_index.get(&(x, y)).unwrap(),
                        *pos_to_index.get(&(x, y + 1)).unwrap(),
                        1.0,
                    );
                }
                if maze[y][x - 1] != MazeBlock::Wall {
                    graph.add_edge(
                        *pos_to_index.get(&(x, y)).unwrap(),
                        *pos_to_index.get(&(x - 1, y)).unwrap(),
                        1.0,
                    );
                }
                if maze[y][x + 1] != MazeBlock::Wall {
                    graph.add_edge(
                        *pos_to_index.get(&(x, y)).unwrap(),
                        *pos_to_index.get(&(x + 1, y)).unwrap(),
                        1.0,
                    );
                }
            }
        }
    }
    let start_pos = *pos_to_contents
        .iter()
        .find(|(_, v)| **v == MazeBlock::Node(0))
        .unwrap()
        .0;
    let actual_nodes = pos_to_contents
        .iter()
        .filter(|(_, v)| **v != MazeBlock::Wall && **v != MazeBlock::Open)
        .map(|n| *n.0)
        // .filter(|p| *p != start_pos)
        .map(|n| *pos_to_index.get(&n).unwrap())
        .collect_vec();
    let start_node = *pos_to_index.get(&start_pos).unwrap();
    let mut actual_graph = graph.filter_map(
        |n, _| {
            if actual_nodes.contains(&n) {
                Some(n)
            } else {
                None
            }
        },
        |_, _| Some(1),
    );
    for (i, start) in actual_nodes.iter().enumerate() {
        for (j, target) in actual_nodes.iter().enumerate() {
            if start < target {
                let distance = *algo::dijkstra(&graph, *start, Some(*target), |_| 1)
                    .get(target)
                    .unwrap();
                actual_graph.update_edge(
                    NodeIndex::from(i as u32),
                    NodeIndex::from(j as u32),
                    distance,
                );
            }
        }
    }
    let mut bfs = Bfs::new(&actual_graph, NodeIndex::from(0));
    let mut counter = 0;
    let walker = visit::Bfs::new(&actual_graph, NodeIndex::from(0));
    
    // dbg!(actual_graph);
    dbg!(counter);
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MazeBlock {
    Wall,
    Open,
    Node(isize),
}

impl MazeBlock {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Open,
            _ => Self::Node(c.to_digit(10).unwrap() as isize),
        }
    }
}
