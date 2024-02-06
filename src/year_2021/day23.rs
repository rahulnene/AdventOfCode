use std::time::{Duration, Instant};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2021/day_23_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    dbg!(State::from_input());
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

//0,1,2,3,4,5 <-> Empty,A,B,C,D
type RoomState = u8;

#[derive(Debug, Clone, Copy)]
struct State {
    rooms: [RoomState; 15],
}

impl State {
    fn from_input() -> Self {
        let mut lines = LINES.lines().skip(2);
        let first_row = lines.next().unwrap();
        let second_row = lines.next().unwrap();
        let top_contents = first_row
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect_vec();
        let bottom_contents = second_row
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect_vec();
        let mut rooms = [0; 15];
        for i in 0..4 {
            let top = top_contents[i];
            let bottom = bottom_contents[i];
            rooms[2 * i] = match top {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                _ => unreachable!("Invalid input"),
            };
            rooms[2 * i + 1] = match bottom {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                _ => unreachable!("Invalid input"),
            };
        }
        Self { rooms }
    }
}
