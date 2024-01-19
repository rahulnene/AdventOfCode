use itertools::Itertools;
use petgraph::{graph::Graph, graph::NodeIndex, Directed};
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};
#[must_use]
pub fn solution() -> ((u32, Duration), (u32, Duration)) {
    let lines = include_str!("../../problem_inputs_2022/day_12.txt");
    let mut map = Map::from_str(lines);
    map.generate_graph();
    (solve01(&map), solve02(&map))
}

fn solve01(map: &Map) -> (u32, Duration) {
    let now = Instant::now();
    let start_pos = map.heights.iter().find(|(_, v)| **v == 0).unwrap().0;
    let end_pos = map.heights.iter().find(|(_, v)| **v == 27).unwrap().0;
    let start_index = *map.pos_to_index.get(start_pos).unwrap();
    let end_index = *map.pos_to_index.get(end_pos).unwrap();
    let path = petgraph::algo::dijkstra(&map.graph, start_index, None, |e| *e.weight());
    (*path.get(&end_index).unwrap(), now.elapsed())
}

fn solve02(map: &Map) -> (u32, Duration) {
    let now = Instant::now();
    let start_positions = map.heights.iter().filter(|(_, v)| **v == 1).collect_vec();
    let end_pos = map.heights.iter().find(|(_, v)| **v == 27).unwrap().0;
    let a = start_positions
        .iter()
        .map(|(pos, _)| *pos)
        .filter_map(|start_pos| {
            let start_index = *map.pos_to_index.get(start_pos).unwrap();
            let end_index = *map.pos_to_index.get(end_pos).unwrap();
            let path = petgraph::algo::dijkstra(&map.graph, start_index, None, |e| *e.weight());
            path.get(&end_index).copied()
        })
        .min()
        .unwrap();
    (a, now.elapsed())
}

type Position = (u32, u32);

#[derive(Debug, Clone)]
struct Map {
    heights: FxHashMap<Position, u32>,
    graph: Graph<Position, u32, Directed>,
    pos_to_index: FxHashMap<Position, NodeIndex>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut heights: FxHashMap<Position, u32> = FxHashMap::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as u32;
                let y = y as u32;
                match c {
                    'S' => {
                        heights.insert((x, y), 0);
                    }
                    'E' => {
                        heights.insert((x, y), 27);
                    }
                    k => {
                        heights.insert((x, y), k as u32 - 'a' as u32 + 1);
                    }
                }
            }
        }
        Self {
            heights,
            graph: Graph::new(),
            pos_to_index: FxHashMap::default(),
        }
    }

    fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let (x, y) = pos;
        if y > 0 && self.heights.get(&(x, y - 1)).is_some() {
            neighbors.push((x, y - 1));
        }

        if x > 0 && self.heights.get(&(x - 1, y)).is_some() {
            neighbors.push((x - 1, y));
        }

        if self.heights.get(&(x, y + 1)).is_some() {
            neighbors.push((x, y + 1));
        }
        if self.heights.get(&(x + 1, y)).is_some() {
            neighbors.push((x + 1, y));
        }
        neighbors
    }

    fn generate_graph(&mut self) {
        for pos in self.heights.keys() {
            let neighbors = self.get_neighbors(*pos);
            let cur_index = if self.pos_to_index.contains_key(pos) {
                *self.pos_to_index.get(pos).unwrap()
            } else {
                let index = self.graph.add_node(*pos);
                self.pos_to_index.insert(*pos, index);
                index
            };
            for neighbor in neighbors {
                let neighbor_index = if let std::collections::hash_map::Entry::Vacant(e) =
                    self.pos_to_index.entry(neighbor)
                {
                    let index = self.graph.add_node(neighbor);
                    e.insert(index);
                    index
                } else {
                    *self.pos_to_index.get(&neighbor).unwrap()
                };
                if (*self.heights.get(pos).unwrap() as isize
                    - *self.heights.get(&neighbor).unwrap() as isize)
                    >= -1
                {
                    self.graph.add_edge(cur_index, neighbor_index, 1);
                }
            }
        }
    }
}
