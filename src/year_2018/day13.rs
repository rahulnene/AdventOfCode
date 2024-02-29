use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2018/day_13_test.txt");

pub fn solution() -> ((String, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (String, Duration) {
    let now = Instant::now();
    let mut track = RaceTrack::from_str();
    while track.get_collision().is_none() {
        // println!(
        //     "Car 1 at ({}, {}) moving {:?} and will go {:?} at next intersection",
        //     track.cars[0].position.0,
        //     track.cars[0].position.1,
        //     track.cars[0].direction,
        //     track.cars[0].next_intersection_turn
        // );
        // println!(
        //     "Car 2 at ({}, {}) moving {:?} and will go {:?} at next intersection\n",
        //     track.cars[1].position.0,
        //     track.cars[1].position.1,
        //     track.cars[1].direction,
        //     track.cars[1].next_intersection_turn
        // );
        track.advance_cars();
    }
    let ans = track.get_collision().unwrap();
    (format!("{},{}", ans.0, ans.1), now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct RaceTrack {
    cars: Vec<Car>,
    tracks: FxHashMap<Position, Track>,
}

impl RaceTrack {
    fn from_str() -> RaceTrack {
        let mut tracks = FxHashMap::default();
        let mut cars = Vec::new();
        for (y, line) in LINES.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '<' || c == '>' || c == '^' || c == 'v' {
                    cars.push(Car::new((x, y), c));
                }
                if c != ' ' {
                    tracks.insert((x, y), Track::from_char(c));
                }
            }
        }
        RaceTrack { tracks, cars }
    }

    fn advance_cars(&mut self) {
        let old_cars = self.cars.clone();
        let mut new_cars = Vec::with_capacity(old_cars.len());
        for car in old_cars {
            let mut hit_intersection = false;
            let new_position = match car.direction {
                Direction::Up => (car.position.0, car.position.1 - 1),
                Direction::Down => (car.position.0, car.position.1 + 1),
                Direction::Left => (car.position.0 - 1, car.position.1),
                Direction::Right => (car.position.0 + 1, car.position.1),
            };
            let new_direction = match self.tracks.get(&new_position) {
                Some(track) => match track {
                    Track::Horizontal | Track::Vertical => car.direction,
                    Track::BackSlash => match car.direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    },
                    Track::FrontSlash => match car.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    },
                    Track::Intersection => match car.direction {
                        Direction::Up => {
                            hit_intersection = true;
                            match car.next_intersection_turn {
                                ToTurn::Left => Direction::Left,
                                ToTurn::Straight => car.direction,
                                ToTurn::Right => Direction::Right,
                            }
                        }
                        Direction::Down => {
                            hit_intersection = true;
                            match car.next_intersection_turn {
                                ToTurn::Left => Direction::Right,
                                ToTurn::Straight => car.direction,
                                ToTurn::Right => Direction::Left,
                            }
                        }
                        Direction::Left => {
                            hit_intersection = true;
                            match car.next_intersection_turn {
                                ToTurn::Left => Direction::Down,
                                ToTurn::Straight => car.direction,
                                ToTurn::Right => Direction::Up,
                            }
                        }
                        Direction::Right => {
                            hit_intersection = true;
                            match car.next_intersection_turn {
                                ToTurn::Left => Direction::Up,
                                ToTurn::Straight => car.direction,
                                ToTurn::Right => Direction::Down,
                            }
                        }
                    },
                },
                None => unreachable!(),
            };
            let new_car = Car {
                position: new_position,
                direction: new_direction,
                next_intersection_turn: if hit_intersection {
                    car.next_turn()
                } else {
                    car.next_intersection_turn
                },
            };
            new_cars.push(new_car);
        }
        self.cars = new_cars;
    }

    fn get_collision(&self) -> Option<Position> {
        let mut positions = FxHashSet::default();
        for car in &self.cars {
            if !positions.insert(car.position) {
                return Some(car.position);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Track {
    Horizontal,
    Vertical,
    BackSlash,
    FrontSlash,
    Intersection,
}

impl Track {
    fn from_char(c: char) -> Track {
        match c {
            '-' | '<' | '>' => Track::Horizontal,
            '|' | '^' | 'v' => Track::Vertical,
            '/' => Track::FrontSlash,
            '\\' => Track::BackSlash,
            '+' => Track::Intersection,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Car {
    position: Position,
    direction: Direction,
    next_intersection_turn: ToTurn,
}

impl Car {
    fn new(position: Position, direction: char) -> Car {
        match direction {
            '^' => Car {
                position,
                direction: Direction::Up,
                next_intersection_turn: ToTurn::Left,
            },
            'v' => Car {
                position,
                direction: Direction::Down,
                next_intersection_turn: ToTurn::Left,
            },
            '<' => Car {
                position,
                direction: Direction::Left,
                next_intersection_turn: ToTurn::Left,
            },
            '>' => Car {
                position,
                direction: Direction::Right,
                next_intersection_turn: ToTurn::Left,
            },
            _ => unreachable!(),
        }
    }

    fn next_turn(&self) -> ToTurn {
        match self.next_intersection_turn {
            ToTurn::Left => ToTurn::Straight,
            ToTurn::Straight => ToTurn::Right,
            ToTurn::Right => ToTurn::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ToTurn {
    Left,
    Straight,
    Right,
}
