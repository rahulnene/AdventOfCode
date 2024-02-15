use derive_more::{Add, AddAssign};
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2020/day_12.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut ship = Ship::new();
    for instr in LINES.lines() {
        ship.action_p1(instr);
    }
    (ship.manhattan_distance(), now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut ship = Ship::new();
    for instr in LINES.lines() {
        ship.action_p2(instr);
    }
    (ship.manhattan_distance(), now.elapsed())
}

#[derive(Clone, Copy, Debug, Add, AddAssign, Default, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate_left(&mut self) {
        let orig = *self;
        self.x = -orig.y;
        self.y = orig.x;
    }
    fn rotate_right(&mut self) {
        let orig = *self;
        self.x = orig.y;
        self.y = -orig.x;
    }
    fn flip(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Direction {
    N,
    #[default]
    E,
    W,
    S,
}

impl Direction {
    fn rotate(&mut self, clockwise: bool, value: u32) {
        match (clockwise, value) {
            (false, 90) | (true, 270) => self.rotate_left(),
            (_, 180) => self.flip(),
            (false, 270) | (true, 90) => self.rotate_right(),
            _ => {
                dbg!(clockwise, value);
                panic!()
            }
        }
    }

    fn rotate_left(&mut self) -> () {
        let new_dir = match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
        };
        *self = new_dir;
    }
    fn rotate_right(&mut self) {
        let new_dir = match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::W => Direction::N,
            Direction::S => Direction::W,
        };
        *self = new_dir;
    }
    fn flip(&mut self) {
        let new_dir = match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
        };
        *self = new_dir;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Ship {
    position: Position,
    direction: Direction,
    waypoint: Position,
}

impl Ship {
    fn new() -> Self {
        Self {
            waypoint: Position::new(10, 1),
            ..Default::default()
        }
    }

    fn manhattan_distance(&self) -> usize {
        self.position.manhattan_distance() as usize
    }

    fn action_p1(&mut self, instr: &str) {
        let (action, value) = instr.split_at(1);
        let value = value.parse::<u32>().unwrap();
        match action {
            "N" => self.go_north(value),
            "S" => self.go_south(value),
            "E" => self.go_east(value),
            "W" => self.go_west(value),
            "L" => self.direction.rotate(false, value),
            "R" => self.direction.rotate(true, value),
            "F" => self.go_forward(value),
            _ => {
                dbg!(action);
                panic!("Invalid action")
            }
        }
    }

    fn action_p2(&mut self, instr: &str) {
        let (action, value) = instr.split_at(1);
        let value = value.parse::<u32>().unwrap();
        match action {
            "N" => self.waypoint += Position::new(0, 1 * value as i32),
            "S" => self.waypoint += Position::new(0, -1 * value as i32),
            "E" => self.waypoint += Position::new(1 * value as i32, 0),
            "W" => self.waypoint += Position::new(-1 * value as i32, 0),
            "L" => self.rotate_waypoint(false, value),
            "R" => self.rotate_waypoint(true, value),
            "F" => self.go_toward_waypoint(value),
            _ => {
                dbg!(action);
                panic!("Invalid action")
            }
        }
    }

    fn go_north(&mut self, value: u32) {
        self.position += Position::new(0, value as i32);
    }
    fn go_south(&mut self, value: u32) {
        self.position += Position::new(0, -1 * value as i32);
    }
    fn go_east(&mut self, value: u32) {
        self.position += Position::new(value as i32, 0);
    }
    fn go_west(&mut self, value: u32) {
        self.position += Position::new(-1 * value as i32, 0);
    }
    fn go_forward(&mut self, value: u32) {
        match self.direction {
            Direction::N => self.go_north(value),
            Direction::E => self.go_east(value),
            Direction::S => self.go_south(value),
            Direction::W => self.go_west(value),
        }
    }

    fn rotate_waypoint(&mut self, clock: bool, value: u32) {
        match (clock, value) {
            (false, 90) | (true, 270) => self.waypoint.rotate_left(),
            (_, 180) => self.waypoint.flip(),
            (true, 90) | (false, 270) => self.waypoint.rotate_right(),
            _ => unreachable!(),
        }
    }
    fn go_toward_waypoint(&mut self, value: u32) {
        let del_x = value as i32 * self.waypoint.x;
        let del_y = value as i32 * self.waypoint.y;
        self.position += Position::new(del_x, del_y);
    }
}
