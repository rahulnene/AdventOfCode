use rustc_hash::FxHashMap;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};
const LINES: &str = include_str!("../../problem_inputs_2018/day_18.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

const TOTAL_STEPS: usize = 1_000_000_000;

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut grid = MyHashMap {
        map: FxHashMap::default(),
    };
    for (y, line) in LINES.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let content = match c {
                '.' => AcreContent::Open,
                '|' => AcreContent::Trees,
                '#' => AcreContent::Lumberyard,
                _ => panic!("Invalid character in input"),
            };
            grid.map.insert((x as isize, y as isize), content);
        }
    }
    let mut grid_to_step = FxHashMap::default();
    let mut step = 1;
    while step <= 10 {
        grid.map = update(&grid.map);
        let seen = grid_to_step.insert(grid.clone(), step);
        if let Some(old_step) = seen {
            let cycle_length = step - old_step;
            let mut remaining_steps = TOTAL_STEPS - step;
            remaining_steps %= cycle_length;
            for _ in 0..remaining_steps {
                grid.map = update(&grid.map);
            }
            break;
        }
        step += 1;
    }
    (res_value(&grid.map), now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut grid = MyHashMap {
        map: FxHashMap::default(),
    };
    for (y, line) in LINES.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let content = match c {
                '.' => AcreContent::Open,
                '|' => AcreContent::Trees,
                '#' => AcreContent::Lumberyard,
                _ => panic!("Invalid character in input"),
            };
            grid.map.insert((x as isize, y as isize), content);
        }
    }
    let mut grid_to_step = FxHashMap::default();
    let mut step = 1;
    while step <= TOTAL_STEPS {
        grid.map = update(&grid.map);
        let seen = grid_to_step.insert(grid.clone(), step);
        if let Some(old_step) = seen {
            let cycle_length = step - old_step;
            let mut remaining_steps = 1_000_000_000 - step;
            remaining_steps %= cycle_length;
            for _ in 0..remaining_steps {
                grid.map = update(&grid.map);
            }
            break;
        }
        step += 1;
    }
    (res_value(&grid.map), now.elapsed())
}

type Position = (isize, isize);

#[derive(Clone, PartialEq, Eq)]
struct MyHashMap<T, U>
where
    T: Hash + Eq + PartialEq,
    U: Hash + Eq + PartialEq,
{
    map: FxHashMap<T, U>,
}

impl<T, U> Hash for MyHashMap<T, U>
where
    T: Hash + Eq + PartialEq,
    U: Hash + Eq + PartialEq,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in &self.map {
            k.hash(state);
            v.hash(state);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum AcreContent {
    Open,
    Trees,
    Lumberyard,
}

fn update(old_grid: &FxHashMap<Position, AcreContent>) -> FxHashMap<Position, AcreContent> {
    let mut new_grid = FxHashMap::default();
    for position in old_grid.keys() {
        let new_content = update_position(old_grid, position);
        new_grid.insert(*position, new_content);
    }
    new_grid
}

fn update_position(grid: &FxHashMap<Position, AcreContent>, position: &Position) -> AcreContent {
    let old_value = grid.get(position).unwrap();
    match old_value {
        AcreContent::Open => {
            if grows_tree(grid, position) {
                AcreContent::Trees
            } else {
                AcreContent::Open
            }
        }
        AcreContent::Trees => {
            if becomes_lumberyard(grid, position) {
                AcreContent::Lumberyard
            } else {
                AcreContent::Trees
            }
        }
        AcreContent::Lumberyard => {
            if remains_lumberyard(grid, position) {
                AcreContent::Lumberyard
            } else {
                AcreContent::Open
            }
        }
    }
}

fn grows_tree(grid: &FxHashMap<Position, AcreContent>, position: &Position) -> bool {
    let neighbor_positions = [
        (position.0 - 1, position.1 - 1),
        (position.0, position.1 - 1),
        (position.0 + 1, position.1 - 1),
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0 - 1, position.1 + 1),
        (position.0, position.1 + 1),
        (position.0 + 1, position.1 + 1),
    ];
    let mut count = 0;
    for neighbor_position in &neighbor_positions {
        if let Some(AcreContent::Trees) = grid.get(neighbor_position) {
            count += 1;
            if count >= 3 {
                return true;
            }
        }
    }
    false
}

fn becomes_lumberyard(grid: &FxHashMap<Position, AcreContent>, position: &Position) -> bool {
    let neighbor_positions = [
        (position.0 - 1, position.1 - 1),
        (position.0, position.1 - 1),
        (position.0 + 1, position.1 - 1),
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0 - 1, position.1 + 1),
        (position.0, position.1 + 1),
        (position.0 + 1, position.1 + 1),
    ];
    let mut count = 0;
    for neighbor_position in &neighbor_positions {
        if let Some(AcreContent::Lumberyard) = grid.get(neighbor_position) {
            count += 1;
            if count >= 3 {
                return true;
            }
        }
    }
    false
}
fn remains_lumberyard(grid: &FxHashMap<Position, AcreContent>, position: &Position) -> bool {
    let neighbor_positions = [
        (position.0 - 1, position.1 - 1),
        (position.0, position.1 - 1),
        (position.0 + 1, position.1 - 1),
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0 - 1, position.1 + 1),
        (position.0, position.1 + 1),
        (position.0 + 1, position.1 + 1),
    ];
    let mut yard_exists = false;
    let mut tree_exists = false;
    for neighbor_position in &neighbor_positions {
        if let Some(AcreContent::Lumberyard) = grid.get(neighbor_position) {
            yard_exists = true;
        }
        if let Some(AcreContent::Trees) = grid.get(neighbor_position) {
            tree_exists = true;
        }
        if yard_exists && tree_exists {
            return true;
        }
    }
    false
}

fn res_value(grid: &FxHashMap<Position, AcreContent>) -> usize {
    let mut trees = 0;
    let mut lumberyards = 0;
    for content in grid.values() {
        match content {
            AcreContent::Open => {}
            AcreContent::Trees => trees += 1,
            AcreContent::Lumberyard => lumberyards += 1,
        }
    }
    trees * lumberyards
}
