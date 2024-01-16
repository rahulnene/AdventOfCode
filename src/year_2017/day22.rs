use fxhash::FxHashMap;

const STEPS:usize = 10_000;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2017/day_21_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    let mut grid = Grid::new(lines);
    for _ in 0..STEPS {
        grid.step();
    }
    grid.carrier.infections
}

fn solve02(lines: &str) -> usize {
    0
}
#[derive(Debug, Clone, Copy)]
enum NodeStatus {
    Infected,
    Clean,
}

impl Default for NodeStatus {
    fn default() -> Self {
        Self::Clean
    }
}

impl NodeStatus {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Infected,
            '.' => Self::Clean,
            _ => panic!("Invalid char"),
        }
    }
    fn flip(&mut self) {
        match self {
            Self::Infected => *self = Self::Clean,
            Self::Clean => *self = Self::Infected,
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

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "up" => Self::Up,
            "down" => Self::Down,
            "left" => Self::Left,
            "right" => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

type Position = (isize, isize);

#[derive(Debug, Clone, Copy)]
struct Carrier {
    position: Position,
    direction: Direction,
    infections: usize,
}

impl Carrier {
    fn new() -> Self {
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
        let carrier = Carrier::new();
        let mut y = 0;
        for line in grid.lines() {
            let mut x = 0;
            for c in line.chars() {
                nodes.insert((x, y), NodeStatus::from_char(c));
                x += 1;
            }
            y += 1;
        }
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
        }
    }

    fn step(&mut self) {
        self.change_direction();
        let current_node_status = *self
            .nodes
            .get(&self.carrier.position)
            .unwrap_or(&NodeStatus::Clean);
        let current_node = self
            .nodes
            .entry(self.carrier.position)
            .or_insert(current_node_status);
        match current_node_status {
            NodeStatus::Clean => {
                self.carrier.infections += 1;
            }
            _ => (),
        };
        current_node.flip();
        self.carrier.move_forward();
    }
}
