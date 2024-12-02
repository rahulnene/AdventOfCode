use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution(test: bool) -> ((usize, Duration), (usize, Duration)) {
    let lines;
    if test {
        lines = include_str!("../../problem_inputs_2024/day_1_test.txt");
    } else {
        lines = include_str!("../../problem_inputs_2024/day_1.txt");
    }
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut count = 0;
    for line in lines.lines() {
        let game_id: usize = line
            .split(':')
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let games_str = line.split(':').nth(1).unwrap().trim();
        let rounds: Vec<_> = games_str.split(';').map(parse_round).collect_vec();
        if rounds.iter().all(|r| valid_round(r)) {
            count += game_id;
        }
        // dbg!(games);
    }
    (count, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut count = 0;
    for line in lines.lines() {
        let games_str = line.split(':').nth(1).unwrap().trim();
        let rounds: Vec<_> = games_str.split(';').map(parse_round).collect_vec();
        let mut blue_balls = 0;
        let mut green_balls = 0;
        let mut red_balls = 0;
        for round in rounds {
            for ball in round {
                match ball.color {
                    Color::Red => red_balls = red_balls.max(ball.amount),
                    Color::Green => green_balls = green_balls.max(ball.amount),
                    Color::Blue => blue_balls = blue_balls.max(ball.amount),
                }
            }
        }
        count += red_balls * green_balls * blue_balls;
        // dbg!(games);
    }
    (count, now.elapsed())
}

fn parse_round(round: &str) -> Vec<Ball> {
    let round = round.trim();
    let mut balls = Vec::new();
    for ball in round.split(',') {
        let ball = ball.trim();
        let (amount, color) = ball.split_at(ball.find(' ').unwrap());
        balls.push(Ball::new(amount, color));
    }
    balls
}

#[derive(Debug, Clone)]
struct Ball {
    amount: usize,
    color: Color,
}

impl Ball {
    fn new(amount: &str, color: &str) -> Self {
        Self {
            amount: amount.parse().unwrap(),
            color: Color::from_str(color),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_str(color: &str) -> Self {
        match color.trim() {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => {
                dbg! {color};
                panic!("Invalid color")
            }
        }
    }
}

fn valid_round(round: &[Ball]) -> bool {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for ball in round {
        match ball.color {
            Color::Red => red += ball.amount,
            Color::Green => green += ball.amount,
            Color::Blue => blue += ball.amount,
        }
    }
    red <= 12 && green <= 13 && blue <= 14
}
