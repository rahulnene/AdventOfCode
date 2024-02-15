use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    time::{Duration, Instant},
};

use itertools::Itertools;
use rustc_hash::FxHashSet;

const LINES: &str = include_str!("../../problem_inputs_2023/day_17.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let grid = Grid::from_str(LINES);
    let ans = min_path_cost::<SimpleCrucible>(&grid);
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let grid = Grid::from_str(LINES);
    let ans = min_path_cost::<UltraCrucible>(&grid);
    (ans, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    cells: Vec<Vec<u8>>,
    size: usize,
}

impl Grid {
    fn from_str(s: &str) -> Self {
        let cells = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u8>().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        let size = cells.len();
        Self { cells, size }
    }
}

trait CrucibleLike<T>: Default + PartialEq + Eq + Ord + PartialOrd + Clone + std::fmt::Debug
where
    Self: Sized,
{
    fn get_next_crucibles(&self, grid: &Grid) -> Vec<T>;
    fn new(position: (usize, usize), moved: usize, cost: usize, facing: Direction) -> Self;
    fn position(&self) -> (usize, usize);
    fn cost(&self) -> usize;
    fn moved(&self) -> usize;
    fn facing(&self) -> Direction;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

fn get_opp(other: Direction) -> Direction {
    match other {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

impl Direction {
    fn allowed(self) -> Vec<Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter(|d| **d != get_opp(self))
        .copied()
        .collect_vec()
    }

    fn move_to_next(self, x: usize, y: usize, size: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
            Direction::Down => {
                if x < size - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Direction::Left => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if y < size - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct SimpleCrucible {
    position: (usize, usize),
    cost: usize,
    moved: usize,
    facing: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct UltraCrucible {
    position: (usize, usize),
    cost: usize,
    moved: usize,
    facing: Direction,
}

impl CrucibleLike<SimpleCrucible> for SimpleCrucible {
    fn new(position: (usize, usize), moved: usize, cost: usize, facing: Direction) -> Self {
        Self {
            position,
            cost,
            moved,
            facing,
        }
    }
    fn facing(&self) -> Direction {
        self.facing
    }
    fn position(&self) -> (usize, usize) {
        self.position
    }
    fn cost(&self) -> usize {
        self.cost
    }
    fn moved(&self) -> usize {
        self.moved
    }
    fn get_next_crucibles(&self, grid: &Grid) -> Vec<Self> {
        let mut crucibles = Vec::new();
        let allowed = Direction::allowed(self.facing);
        assert!(allowed.len() <= 3);
        for direction in allowed {
            if let Some((x, y)) =
                direction.move_to_next(self.position.0, self.position.1, grid.size)
            {
                let new_cost = self.cost + grid.cells[x][y] as usize;
                if direction != self.facing {
                    crucibles.push(SimpleCrucible::new((x, y), 1, new_cost, direction));
                } else if self.moved < 3 {
                    crucibles.push(SimpleCrucible::new(
                        (x, y),
                        self.moved + 1,
                        new_cost,
                        direction,
                    ));
                }
            }
        }
        crucibles
    }
}

impl CrucibleLike<UltraCrucible> for UltraCrucible {
    fn new(position: (usize, usize), moved: usize, cost: usize, facing: Direction) -> Self {
        Self {
            position,
            cost,
            moved,
            facing,
        }
    }
    fn facing(&self) -> Direction {
        self.facing
    }
    fn moved(&self) -> usize {
        self.moved
    }
    fn position(&self) -> (usize, usize) {
        self.position
    }
    fn cost(&self) -> usize {
        self.cost
    }
    fn get_next_crucibles(&self, grid: &Grid) -> Vec<Self> {
        let mut crucibles = Vec::new();
        let allowed = Direction::allowed(self.facing);
        assert!(allowed.len() <= 3);
        for direction in allowed {
            if let Some((x, y)) =
                direction.move_to_next(self.position.0, self.position.1, grid.size)
            {
                let new_cost = self.cost + grid.cells[x][y] as usize;
                if self.moved < 4 {
                    if self.facing == direction {
                        crucibles.push(UltraCrucible::new(
                            (x, y),
                            self.moved + 1,
                            new_cost,
                            direction,
                        ));
                    }
                } else if self.moved == 10 {
                    if self.facing != direction {
                        crucibles.push(UltraCrucible::new((x, y), 1, new_cost, direction));
                    }
                } else if self.facing == direction {
                    crucibles.push(UltraCrucible::new(
                        (x, y),
                        self.moved + 1,
                        new_cost,
                        direction,
                    ));
                } else {
                    crucibles.push(UltraCrucible::new((x, y), 1, new_cost, direction));
                }
            }
        }
        crucibles
    }
}

impl Ord for UltraCrucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for UltraCrucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SimpleCrucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for SimpleCrucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_path_cost<T>(grid: &Grid) -> usize
where
    T: CrucibleLike<T>,
{
    let mut queue = BinaryHeap::new();
    let mut seen = FxHashSet::default();
    let dest = (grid.size - 1, grid.size - 1);
    let right = T::new((0, 0), 1, 0, Direction::Right);
    let down = T::new((0, 0), 1, 0, Direction::Down);
    queue.push(right);
    queue.push(down);
    while let Some(current) = queue.pop() {
        if current.position() == dest {
            return current.cost();
        }
        for crucible in current.get_next_crucibles(grid) {
            if seen.insert((crucible.position(), crucible.facing(), crucible.moved())) {
                queue.push(crucible);
            }
        }
    }
    unreachable!("No path found")
}

fn get_next_crucibles_helper<T: CrucibleLike<T>>(
    crucible: &T,
    grid: &Grid,
    should_create_new_crucible: impl Fn(&T, Direction, Direction) -> bool,
) -> Vec<T> {
    let mut crucibles = Vec::new();
    let allowed = Direction::allowed(crucible.facing());
    assert!(allowed.len() <= 3);
    for direction in allowed {
        if let Some((x, y)) =
            direction.move_to_next(crucible.position().0, crucible.position().1, grid.size)
        {
            let new_cost = crucible.cost() + grid.cells[x][y] as usize;
            if should_create_new_crucible(crucible, direction, crucible.facing()) {
                crucibles.push(T::new((x, y), crucible.moved() + 1, new_cost, direction));
            }
        }
    }
    crucibles
}
