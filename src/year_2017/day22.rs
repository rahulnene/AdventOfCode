use rustc_hash::FxHashMap;

use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_22.txt");
    let mut grid_1 = Grid::new(lines);
    let mut grid_2 = grid_1.clone();
    (solve01(&mut grid_1), solve02(&mut grid_2))
}

fn solve01(grid: &mut Grid) -> (usize, Duration) {
    let now = Instant::now();
    for _ in 0..10_000 {
        grid.step_old();
    }
    (grid.carrier.infections, now.elapsed())
}

fn solve02(grid: &mut Grid) -> (usize, Duration) {
    let now = Instant::now();
    for _ in 0..10_000_000 {
        grid.step_new();
    }
    (grid.carrier.infections, now.elapsed())
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeStatus {
    Weakened,
    Flagged,
    Infected,
    Clean,
}

impl Default for NodeStatus {
    fn default() -> Self {
        Self::Clean
    }
}

impl NodeStatus {
    const fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Infected,
            '.' => Self::Clean,
            _ => panic!("Invalid char"),
        }
    }
    fn flip_old(&mut self) {
        match self {
            Self::Infected => *self = Self::Clean,
            Self::Clean => *self = Self::Infected,
            _ => unreachable!(),
        }
    }

    fn flip_new(&mut self) {
        match self {
            Self::Infected => *self = Self::Flagged,
            Self::Flagged => *self = Self::Clean,
            Self::Clean => *self = Self::Weakened,
            Self::Weakened => *self = Self::Infected,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (isize, isize);

#[derive(Debug, Clone, Copy)]
struct Carrier {
    position: Position,
    direction: Direction,
    infections: usize,
}

impl Carrier {
    const fn new() -> Self {
        Self {
            position: (0, 0),
            direction: Direction::Up,
            infections: 0,
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 -= 1,
            Direction::Down => self.position.1 += 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    nodes: FxHashMap<Position, NodeStatus>,
    carrier: Carrier,
}

impl Grid {
    fn new(grid: &str) -> Self {
        let mut nodes = FxHashMap::default();
        let mut carrier = Carrier::new();
        for (y, line) in grid.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as isize;
                let y = y as isize;
                nodes.insert((x, y), NodeStatus::from_char(c));
            }
        }
        let max_x = nodes.keys().map(|(x, _)| x).max().unwrap() + 1;
        let max_y = nodes.keys().map(|(_, y)| y).max().unwrap() + 1;
        carrier.position = (max_x / 2, max_y / 2);
        Self { nodes, carrier }
    }

    fn get_infection_status(&self, position: Position) -> NodeStatus {
        *self.nodes.get(&position).unwrap_or(&NodeStatus::Clean)
    }

    fn turn_left(&mut self) {
        match self.carrier.direction {
            Direction::Up => self.carrier.direction = Direction::Left,
            Direction::Down => self.carrier.direction = Direction::Right,
            Direction::Left => self.carrier.direction = Direction::Down,
            Direction::Right => self.carrier.direction = Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        match self.carrier.direction {
            Direction::Up => self.carrier.direction = Direction::Right,
            Direction::Down => self.carrier.direction = Direction::Left,
            Direction::Left => self.carrier.direction = Direction::Up,
            Direction::Right => self.carrier.direction = Direction::Down,
        }
    }

    fn change_direction(&mut self) {
        let carrier_position = self.carrier.position;
        match self.get_infection_status(carrier_position) {
            NodeStatus::Infected => self.turn_right(),
            NodeStatus::Clean => self.turn_left(),
            NodeStatus::Flagged => {
                self.turn_left();
                self.turn_left();
            }
            NodeStatus::Weakened => {}
        }
    }

    fn step_old(&mut self) {
        let current_node = self
            .nodes
            .entry(self.carrier.position)
            .or_insert(NodeStatus::Clean);
        let current_node_status = *current_node;
        self.change_direction();
        let current_node = self.nodes.get_mut(&self.carrier.position).unwrap();
        if current_node_status == NodeStatus::Clean {
            self.carrier.infections += 1;
        }
        current_node.flip_old();
        self.carrier.move_forward();
    }

    fn step_new(&mut self) {
        let current_node = self
            .nodes
            .entry(self.carrier.position)
            .or_insert(NodeStatus::Clean);
        let current_node_status = *current_node;
        self.change_direction();
        let current_node = self.nodes.get_mut(&self.carrier.position).unwrap();
        if current_node_status == NodeStatus::Weakened {
            self.carrier.infections += 1;
        }
        current_node.flip_new();
        self.carrier.move_forward();
    }
}
