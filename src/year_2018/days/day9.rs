use std::time::Instant;
use std::collections::VecDeque;
use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let line = include_str!("../../../problem_inputs_2018/day_9.txt");

    let (player_count, last_marble) = parse_input(line);
    match part {
        1 => solve01(player_count, last_marble),
        2 => solve02(player_count, last_marble * 100),
        _ => 1,
    }
}

fn solve01(player_count: usize, last_marble: usize) -> usize {
    let now = Instant::now();
    let mut circle = Circle::new(last_marble);
    let mut players = vec![0; player_count];
    let mut player = 0;
    for _ in 0..last_marble {
        players[player] += circle.place();
        player = (player + 1) % player_count;
    }
    dbg!(Instant::now() - now);
    *players.iter().max().unwrap()
}

fn solve02(player_count: usize, last_marble: usize) -> usize {
    let mut circle = Circle::new(last_marble);
    let mut players = vec![0; player_count];
    let mut player = 0;
    for _ in 0..last_marble {
        players[player] += circle.place();
        player = (player + 1) % player_count;
    }
    *players.iter().max().unwrap()
}

fn parse_input(line: &str) -> (usize, usize) {
    let split = line.split_ascii_whitespace().collect_vec();
    (split[0].parse().unwrap(), split[6].parse().unwrap())
}

#[derive(Debug, Clone)]
struct Circle {
    marbles: Vec<usize>,
    current: usize,
    insertion_index: usize,
}

impl Circle {
    fn new(last_marble: usize) -> Self {
        let mut marbles = Vec::with_capacity(last_marble + 1);
        marbles.push(0);
        Circle {
            marbles,
            current: 0,
            insertion_index: 0,
        }
    }
    fn place(&mut self) -> usize {
        if self.current == 0 {
            self.marbles.push(1);
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
        let insertion_ind = self.wrapping_increase(self.insertion_index);
        self.marbles.insert(insertion_ind, self.current + 1);
        self.current += 1;
        self.insertion_index = insertion_ind;
        0
    }

    fn wrapping_increase(&self, ind: usize) -> usize {
        (ind + 2) % self.marbles.len()
    }

    fn debug(&self) {
        println!("{:?}", self.marbles);
        println!("{} {}", self.current, self.insertion_index);
    }

    fn trigger_23(&mut self) -> usize {
        self.insertion_index = (self.marbles.len() + self.insertion_index - 7) % self.marbles.len();
        let removed = self.marbles.remove(self.insertion_index);
        self.current += 1;
        return removed + self.current;
    }
}
