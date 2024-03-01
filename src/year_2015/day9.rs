use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2015/day_9.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let start = Instant::now();
    let mut graph = FxHashMap::default();
    let mut cities = FxHashSet::default();
    for line in LINES.lines() {
        let (from, _, to, _, distance) = line.split_ascii_whitespace().collect_tuple().unwrap();
        graph.insert(
            (from.to_string(), to.to_string()),
            distance.parse::<usize>().unwrap(),
        );
        graph.insert(
            (to.to_string(), from.to_string()),
            distance.parse::<usize>().unwrap(),
        );
        cities.insert(from.to_string());
        cities.insert(to.to_string());
    }
    let all_permutations = cities
        .iter()
        .permutations(cities.len())
        .map(|path| length_of_path(&path, &graph))
        .collect_vec();
    (
        (*all_permutations.iter().min().unwrap(), start.elapsed()),
        (*all_permutations.iter().max().unwrap(), start.elapsed()),
    )
}

fn length_of_path(
    path: &[&String],
    graph: &std::collections::HashMap<
        (String, String),
        usize,
        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
    >,
) -> usize {
    path.windows(2)
        .map(|pair| {
            graph
                .get(&(pair[0].to_string(), pair[1].to_string()))
                .unwrap()
        })
        .sum()
}
