use itertools::Itertools;
use std::collections::VecDeque;
use std::time::Instant;

pub fn solution(part: u8) -> usize {
    let line = include_str!("../../../problem_inputs_2018/day_9.txt");

    let (player_count, last_marble) = {
        let split = line.split_ascii_whitespace().collect_vec();
        (split[0].parse().unwrap(), split[6].parse().unwrap())
    };
    match part {
        1 => solve(player_count, last_marble),
        2 => solve(player_count, last_marble * 100),
        _ => 1,
    }
}

fn solve(player_count: usize, last_marble: usize) -> usize {
    let now = Instant::now();
    let mut circle = Circle::new(last_marble);
    let mut players = vec![0; player_count];
    let mut player = 0;
    for _ in 0..last_marble {
        let score = circle.place();
        if score != 0 {
            players[player] += score;
        }
        player = (player + 1) % player_count;
    }
    dbg!(Instant::now() - now);
    *players.iter().max().unwrap()
}

#[derive(Debug, Clone)]
struct Circle {
    marbles: VecDeque<usize>,
    current: usize,
    insertion_index: usize,
}

impl Circle {
    fn new(last_marble: usize) -> Self {
        let mut marbles = VecDeque::with_capacity(last_marble + 1);
        marbles.push_front(0);
        Circle {
            marbles,
            current: 0,
            insertion_index: 0,
        }
    }
    fn place(&mut self) -> usize {
        if self.current == 0 {
            self.marbles.push_front(1);
            self.current = 1;
            self.insertion_index = 1;
            0
        } else if (self.current + 1) % 23 == 0 {
            self.trigger_23()
        } else {
            self.normal_move()
        }
    }

    fn normal_move(&mut self) -> usize {
        self.current += 1;
        for _ in 0..2 {
            let front = self.marbles.pop_front().unwrap();
            self.marbles.push_back(front);
        }
        self.marbles.push_front(self.current);
        0
    }

    fn debug(&self) {
        println!("{:?}", self.marbles);
        println!("{} {}", self.current, self.insertion_index);
    }

    fn trigger_23(&mut self) -> usize {
        self.current += 1;
        for _ in 0..7 {
            let back = self.marbles.pop_back().unwrap();
            self.marbles.push_front(back);
        }
        self.marbles.pop_front().unwrap() + self.current
    }
}
