use std::time::{Duration, Instant};

use rustc_hash::{FxHashMap, FxHashSet};
use petgraph::{Directed, Graph};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2022/day_7.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    // let mut name_to_index = FxHashMap::default();
    let mut computer = Computer {
        current_dir: String::from("/"),
        filesystem: Graph::new(),
    };
    // for line in lines.lines() {
    //     if line.starts_with('$') {
    //     } else {
    //         if line.starts_with('d') {
    //             let dir_name = line.split(' ').nth(1).unwrap();
    //             if name_to_index.contains_key(&dir_name) {
    //                 computer.filesystem.add_edge(dir);
    //             } else {
    //                 let dir = Node {
    //                     name: dir_name.to_string(),
    //                     size: 0,
    //                     content: NodeType::Directory,
    //                     parent: computer.current_dir,
    //                 };
    //                 computer.filesystem.add_node(dir);
    //             }
    //         }
    //     }
    // }
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone)]
struct Computer {
    current_dir: String,
    filesystem: Graph<Node, Node, Directed>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeType {
    Directory,
    File,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    name: String,
    size: usize,
    content: NodeType,
    parent: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Command {
    Cd(String),
    CdUp,
    CdRoot,
    Ls,
}
