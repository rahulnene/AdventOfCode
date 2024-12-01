use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2023/day_23_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let maze = Maze::from_str(LINES);
    let paths = maze.get_all_paths(maze.get_start(), maze.get_end());
    let ans = paths.iter().map(|p| get_path_length(&p)).max().unwrap();
    dbg!(&ans);
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

type Position = (isize, isize);
type Path = Vec<Position>;

fn get_path_length(path: &Path) -> usize {
    let mut length = 0;
    for (a, b) in path.iter().tuple_windows() {
        let del = a.0 - b.0 + a.1 - b.1;
        length += del.abs() as usize;
    }
    length
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Content {
    Path,
    #[default]
    Forest,
    Slope(Slope),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slope {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Maze {
    maze: FxHashMap<Position, Content>,
}

impl Maze {
    fn new(maze: FxHashMap<Position, Content>) -> Self {
        Self { maze }
    }
    fn from_str(str: &str) -> Self {
        let mut maze = FxHashMap::default();
        for (y, line) in str.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;
                let content = match c {
                    '.' => Content::Path,
                    '#' => Content::Forest,
                    _ => Content::Slope(match c {
                        '^' => Slope::Up,
                        'v' => Slope::Down,
                        '<' => Slope::Left,
                        '>' => Slope::Right,
                        _ => unreachable!(),
                    }),
                };
                maze.insert((x, y), content);
            }
        }
        Self { maze }
    }
    fn from_str_p2(str: &str) -> Self {
        let mut maze = FxHashMap::default();
        for (y, line) in str.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;
                let content = match c {
                    '#' => Content::Forest,
                    _ => Content::Path,
                };
                maze.insert((x, y), content);
            }
        }
        Self { maze }
    }

    fn get_content(&self, pos: &Position) -> Content {
        *self.maze.get(&pos).unwrap_or(&Content::Forest)
    }

    fn get_neighbors(&self, pos: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let (x, y) = pos;
        // println!("{:?}", &pos);
        match self.get_content(&(x - 1, y)) {
            Content::Path => neighbors.push((x - 1, y)),
            Content::Slope(Slope::Left) => neighbors.push((x - 2, y)),
            _ => {}
        }
        match self.get_content(&(x + 1, y)) {
            Content::Path => neighbors.push((x + 1, y)),
            Content::Slope(Slope::Right) => neighbors.push((x + 2, y)),
            _ => {}
        }
        match self.get_content(&(x, y - 1)) {
            Content::Path => neighbors.push((x, y - 1)),
            Content::Slope(Slope::Up) => neighbors.push((x, y - 2)),
            _ => {}
        }
        match self.get_content(&(x, y + 1)) {
            Content::Path => neighbors.push((x, y + 1)),
            Content::Slope(Slope::Down) => neighbors.push((x, y + 2)),
            _ => {}
        }
        // println!("{:?} => {:?}", pos, &neighbors);
        neighbors
    }

    fn get_start(&self) -> Position {
        *self
            .maze
            .iter()
            .find(|(k, v)| k.1 == 0 && **v == Content::Path)
            .unwrap()
            .0
    }
    fn get_end(&self) -> Position {
        let max_y = self.maze.keys().map(|(_, y)| y).max().unwrap();
        *self
            .maze
            .iter()
            .find(|(k, v)| k.1 == *max_y && **v == Content::Path)
            .unwrap()
            .0
    }
    fn get_all_paths(
        &self,
        start: (isize, isize),
        end: (isize, isize),
    ) -> Vec<Vec<(isize, isize)>> {
        let mut paths = Vec::new();
        let mut stack = vec![(start, 0)];
        let mut path = vec![start];
        let mut visited = FxHashSet::default();
        visited.insert(start);

        while let Some((current, length)) = stack.pop() {
            while path.len() > length {
                visited.remove(&path.pop().unwrap());
            }
            path.push(current);

            if current == end {
                paths.push(path.clone());
            } else {
                for neighbor in self.get_neighbors(current) {
                    if !visited.contains(&neighbor) {
                        stack.push((neighbor, length + 1));
                        visited.insert(neighbor);
                    }
                }
            }
        }

        paths
    }
}
