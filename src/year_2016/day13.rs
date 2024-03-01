use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use rustc_hash::FxHashMap;

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();

    let mut maze = Maze::new(1350);
    let ans = maze.find_path_to((31, 39)).unwrap();
    // let mut maze = Maze::new(10);
    // let ans = maze.find_path_to((7, 4)).unwrap();
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();

    let mut maze = Maze::new(1350);
    // let ans = maze.find_all_locations_within(50);
    (0, now.elapsed())
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Content {
    Wall,
    Open,
}

type Position = (isize, isize);
#[derive(Debug, Clone, PartialEq, Eq)]
struct Maze {
    favorite_number: u16,
    contents: FxHashMap<Position, Content>,
}

impl Maze {
    fn new(favorite_number: u16) -> Self {
        Self {
            favorite_number,
            contents: FxHashMap::default(),
        }
    }

    fn get_content(&mut self, position: Position) -> Content {
        if let Some(content) = self.contents.get(&position) {
            return *content;
        }
        let (x, y) = position;
        let value = x * x + 3 * x + 2 * x * y + y + y * y + self.favorite_number as isize;
        let is_open = value.count_ones() % 2 == 0;
        let content = if is_open {
            Content::Open
        } else {
            Content::Wall
        };
        self.contents.insert(
            position,
            if is_open {
                Content::Open
            } else {
                Content::Wall
            },
        );
        content
    }

    fn find_path_to(&mut self, target: Position) -> Option<usize> {
        let mut queue = VecDeque::new();
        queue.push_front((1, 1, 0));
        let mut visited = FxHashMap::default();
        visited.insert((1, 1), 0);
        while let Some((x, y, steps)) = queue.pop_back() {
            if (x, y) == target {
                return Some(steps);
            }
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x < 0 || new_y < 0 {
                    continue;
                }
                if visited.contains_key(&(new_x, new_y)) {
                    continue;
                }
                let content = self.get_content((new_x, new_y));
                if content == Content::Wall {
                    continue;
                }
                visited.insert((new_x, new_y), steps + 1);
                queue.push_front((new_x, new_y, steps + 1));
            }
        }
        None
    }
}
