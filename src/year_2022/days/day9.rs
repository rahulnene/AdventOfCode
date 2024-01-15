use std::time::{Duration, Instant};

use fxhash::FxHashSet;
use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2022/day_9_test.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut snake = Rope::new(3);
    for line in lines.lines() {
        let mut chars = line.split_whitespace();
        let direction = match chars.next().unwrap() {
            "U" => MoveDirection::Up,
            "D" => MoveDirection::Down,
            "L" => MoveDirection::Left,
            "R" => MoveDirection::Right,
            _ => unreachable!(),
        };
        let amount = chars.next().unwrap().parse::<usize>().unwrap();
        snake.move_snake(direction, amount);
    }
    (snake.tail_visited.len(), now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Knot {
    pos: (isize, isize),
}

impl Knot {
    fn new(pos: (isize, isize)) -> Self {
        Self { pos }
    }

    fn move_as(&mut self, direction: MoveDirection) {
        match direction {
            MoveDirection::Up => {
                self.pos.1 += 1;
            }
            MoveDirection::Down => {
                self.pos.1 -= 1;
            }
            MoveDirection::Left => {
                self.pos.0 -= 1;
            }
            MoveDirection::Right => {
                self.pos.0 += 1;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rope {
    knots: Vec<Knot>,
    tail_visited: FxHashSet<(isize, isize)>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        let mut tail_visited = FxHashSet::default();
        tail_visited.insert((0, 0));
        Self {
            knots: vec![Knot::new((0, 0)); knot_count],
            tail_visited,
        }
    }

    fn move_snake(&mut self, direction: MoveDirection, amount: usize) {
        for _ in 0..amount {
            // println!("head: {:?}, tail: {:?}", self.head_pos, self.tail_pos);
            self.knots[0].move_as(direction);
            let mut neighbor_pairs = self.knots.windows(2).collect_vec();
            for pair in neighbor_pairs.iter_mut() {
                let head = &pair[0];
                let tail = &mut pair[1];
                if are_neighbors(*head, *tail) {
                    continue;
                } else {
                    let diff = get_head_tail_relative_diff(*head, *tail);
                    let (x_diff, y_diff) = diff;
                    match x_diff {
                        2 => match y_diff {
                            -1 => {
                                tail.move_as(MoveDirection::Right);
                                tail.move_as(MoveDirection::Up);
                            }
                            0 => tail.move_as(MoveDirection::Right),
                            1 => {
                                tail.move_as(MoveDirection::Right);
                                tail.move_as(MoveDirection::Down);
                            }
                            _ => {}
                        },
                        -2 => match y_diff {
                            -1 => {
                                tail.move_as(MoveDirection::Left);
                                tail.move_as(MoveDirection::Up);
                            }
                            0 => tail.move_as(MoveDirection::Left),
                            1 => {
                                tail.move_as(MoveDirection::Left);
                                tail.move_as(MoveDirection::Down);
                            }
                            _ => {}
                        },
                        1 => match y_diff {
                            2 => {
                                tail.move_as(MoveDirection::Up);
                                tail.move_as(MoveDirection::Right);
                            }
                            -2 => {
                                tail.move_as(MoveDirection::Down);
                                tail.move_as(MoveDirection::Right);
                            }
                            _ => {}
                        },
                        -1 => match y_diff {
                            2 => {
                                tail.move_as(MoveDirection::Up);
                                tail.move_as(MoveDirection::Left);
                            }
                            -2 => {
                                tail.move_as(MoveDirection::Down);
                                tail.move_as(MoveDirection::Left);
                            }
                            _ => {}
                        },
                        0 => match y_diff {
                            2 => tail.move_as(MoveDirection::Up),
                            -2 => tail.move_as(MoveDirection::Down),
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            self.tail_visited.insert(self.knots.last().unwrap().pos);
        }
    }
}

fn are_neighbors(knot1: Knot, knot2: Knot) -> bool {
    let (head_x, head_y) = knot1.pos;
    let (tail_x, tail_y) = knot2.pos;
    (head_x.abs_diff(tail_x)) <= 1 && (head_y.abs_diff(tail_y)) <= 1
}

fn get_head_tail_relative_diff(knot1: Knot, knot2: Knot) -> (isize, isize) {
    let (head_x, head_y) = knot1.pos;
    let (tail_x, tail_y) = knot2.pos;
    let x_diff = head_x - tail_x;
    let y_diff = head_y - tail_y;
    (x_diff, y_diff)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
