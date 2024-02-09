use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use std::{
    str::FromStr,
    time::{Duration, Instant},
};

const LINES: &str = include_str!("../../problem_inputs_2020/day_11.txt");
lazy_static! {
    static ref AREA: Area = LINES.parse::<Area>().unwrap();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve(true), solve(false))
}

fn solve(part_1: bool) -> (usize, Duration) {
    let now = Instant::now();
    let mut area = AREA.clone();
    let mut new_area = update(&area, part_1);
    while area != new_area {
        area = new_area;
        new_area = update(&area, part_1);
    }
    (area.count_occupied(), now.elapsed())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn get_neighbors(&self) -> [Position; 8] {
        [
            Position {
                x: self.x - 1,
                y: self.y - 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
            Position {
                x: self.x + 1,
                y: self.y - 1,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Area {
    seats: FxHashMap<Position, SeatType>,
}

impl Area {
    fn get_visible_seats(&self, position: &Position, part_1: bool) -> usize {
        if part_1 {
            self.get_neighbor_count_adj(position)
        } else {
            self.get_neighbor_count_raycast(position)
        }
    }

    fn get_neighbor_count_adj(&self, position: &Position) -> usize {
        position
            .get_neighbors()
            .iter()
            .filter(|p| self.seats.get(p) == Some(&SeatType::Occupied))
            .count()
    }

    fn get_neighbor_count_raycast(&self, position: &Position) -> usize {
        let mut count = 0;
        for direction in &[
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
            let mut current = *position;
            loop {
                current.x += direction.0;
                current.y += direction.1;
                match self.seats.get(&current) {
                    Some(SeatType::Occupied) => {
                        count += 1;
                        break;
                    }
                    Some(SeatType::Empty) | None => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        count
    }

    fn count_occupied(&self) -> usize {
        self.seats
            .values()
            .filter(|seat| **seat == SeatType::Occupied)
            .count()
    }
}

impl FromStr for Area {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seats = FxHashMap::default();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                seats.insert(
                    Position {
                        x: x as isize,
                        y: y as isize,
                    },
                    SeatType::from_char(c),
                );
            }
        }
        Ok(Self { seats })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SeatType {
    Floor,
    Empty,
    Occupied,
}

impl SeatType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!("Invalid seat type"),
        }
    }
}

fn update(area: &Area, part_1: bool) -> Area {
    let mut new_seats = FxHashMap::default();
    let tolerated_count = if part_1 { 4 } else { 5 };
    for (position, seat) in &area.seats {
        let neighbor_count = area.get_visible_seats(position, part_1);
        match seat {
            SeatType::Empty => {
                if neighbor_count == 0 {
                    new_seats.insert(*position, SeatType::Occupied);
                } else {
                    new_seats.insert(*position, SeatType::Empty);
                }
            }
            SeatType::Occupied => {
                if neighbor_count >= tolerated_count {
                    new_seats.insert(*position, SeatType::Empty);
                } else {
                    new_seats.insert(*position, SeatType::Occupied);
                }
            }
            SeatType::Floor => {
                new_seats.insert(*position, SeatType::Floor);
            }
        }
    }
    Area { seats: new_seats }
}
