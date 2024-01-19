use std::time::{Duration, Instant};

use fxhash::FxHashMap;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2020/day_11_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut hall = Hall::new();
    for (y, line) in lines.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'L' => hall.add_seat(x, y, Occupancy::Empty),
                '#' => hall.add_seat(x, y, Occupancy::Occupied),
                '.' => hall.add_seat(x, y, Occupancy::Floor),
                _ => panic!("Invalid input"),
            }
        }
    }
    dbg!(hall.count_seated());
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hall {
    seats: FxHashMap<(usize, usize), Occupancy>,
}

impl Hall {
    fn new() -> Self {
        Self {
            seats: FxHashMap::default(),
        }
    }

    fn add_seat(&mut self, x: usize, y: usize, status: Occupancy) {
        self.seats.insert((x, y), status);
    }

    fn count_seated(&self) -> usize {
        self.seats
            .values()
            .filter(|&&v| v == Occupancy::Occupied)
            .count()
    }

    fn read_seat_status(&self, x: isize, y: isize) -> Occupancy {
        if x < 0 || y < 0 {
            Occupancy::Floor
        } else {
            *self
                .seats
                .get(&(x as usize, y as usize))
                .unwrap_or(&Occupancy::Floor)
        }
    }

    fn get_neighbor_active_count(){}

    fn pulse(&mut self) {
        let old_status = self.clone();
        for seat in old_status.seats.iter() {

        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Occupancy {
    Empty,
    Occupied,
    Floor,
}
