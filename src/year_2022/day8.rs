use std::time::{Duration, Instant};

use fxhash::{FxHashMap, FxHashSet};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2022/day_8_test.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let size = lines.lines().count();
    let mut forest = Forest::new(size);
    for (y, line) in lines.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            forest.add_tree(x, y, c.to_digit(10).unwrap() as usize)
        }
    }
    dbg!(forest.count_visible_trees());
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Forest {
    trees: Vec<Vec<usize>>,
    size: usize,
}

impl Forest {
    fn new(size: usize) -> Self {
        Self {
            trees: vec![vec![0; size]; size],
            size,
        }
    }

    fn add_tree(&mut self, x: usize, y: usize, height: usize) {
        self.trees[y][x] = height;
    }

    fn get_tree(&self, x: usize, y: usize) -> usize {
        self.trees[y][x]
    }

    fn count_visible_trees(&self) -> usize {
        let mut visible_trees = FxHashSet::default();
        for (row_num, row) in self.trees.iter().enumerate() {
            let mut last_tree = 0;
            for (col_num, tree) in row.iter().enumerate() {
                if row_num == 0
                    || col_num == 0
                    || row_num == self.size - 1
                    || col_num == self.size - 1
                {
                    last_tree = *tree;
                    continue;
                }
                if tree < &last_tree {
                    visible_trees.insert((row_num - 1, col_num - 1, last_tree));
                    continue;
                } else {
                    last_tree = *tree;
                }
            }
        }
        dbg!(&visible_trees);
        visible_trees.len() + 4 * self.size - 4
    }
}
