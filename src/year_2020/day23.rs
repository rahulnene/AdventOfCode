use std::time::{Duration, Instant};
const LINES: &str = include_str!("../../problem_inputs_2020/day_23.txt");

pub fn solution() -> ((String, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (String, Duration) {
    let now = Instant::now();
    let mut game = Game::from_input();
    for _ in 1..=100 {
        game.cycle();
    }
    let ans = game.print_order().strip_suffix('1').unwrap().to_owned();
    (ans, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut game = Game::from_input();
    game.extend_for_p2();
    for _ in 1..=10_000_000 {
        game.cycle();
    }
    let first = game.cup_to_next[1];
    let second = game.cup_to_next[first];
    let ans = first * second;
    (ans, now.elapsed())
}

#[derive(Debug, Clone, Default)]
struct Game {
    cup_to_next: Vec<usize>,
    current_cup: usize,
    max_cup: usize,
    picked_up: [usize; 3],
    next_cup: usize,
    destination_cup: usize,
}

impl Game {
    fn from_input() -> Self {
        let mut cups_to_next = vec![0; 10];
        let mut input = LINES.chars().map(|c| c.to_digit(10).unwrap() as usize);
        let first = input.next().unwrap();
        let mut prev = first;
        for cup in input {
            cups_to_next[prev] = cup;
            prev = cup;
        }
        cups_to_next[prev] = first;
        Self {
            cup_to_next: cups_to_next,
            current_cup: first,
            max_cup: 9,
            ..Default::default()
        }
    }

    fn extend_for_p2(&mut self) {
        let last = LINES.chars().last().unwrap().to_digit(10).unwrap() as usize;
        self.cup_to_next[last] = 10;
        for i in 11..=1_000_000 {
            self.cup_to_next.push(i);
        }
        self.cup_to_next.push(self.current_cup);
        self.max_cup = 1_000_000;
    }
    fn cycle(&mut self) {
        self.pick_up();
        self.get_destination();
        self.place_cups();
    }

    fn pick_up(&mut self) {
        let first_pickup = self.cup_to_next[self.current_cup];
        let second_pickup = self.cup_to_next[first_pickup];
        let third_pickup = self.cup_to_next[second_pickup];
        let cup_after_current = self.cup_to_next[third_pickup];
        self.cup_to_next[self.current_cup] = cup_after_current;
        self.picked_up = [first_pickup, second_pickup, third_pickup];
    }

    fn get_destination(&mut self) {
        self.destination_cup = self.current_cup;
        if self.destination_cup == 0 {
            self.destination_cup = self.max_cup;
        }
        loop {
            self.destination_cup -= 1;
            if self.destination_cup == 0 {
                self.destination_cup = self.max_cup;
            }
            if !self.picked_up.contains(&self.destination_cup) {
                break;
            }
        }
        self.current_cup = self.cup_to_next[self.current_cup];
    }

    fn place_cups(&mut self) {
        self.next_cup = self.cup_to_next[self.destination_cup];
        self.cup_to_next[self.destination_cup] = self.picked_up[0];
        self.cup_to_next[self.picked_up[2]] = self.next_cup;
    }

    fn print_order(&self) -> String {
        let mut ans = String::new();
        let mut next = self.cup_to_next[1];
        while next != 1 {
            ans.push_str(&next.to_string());
            next = self.cup_to_next[next];
        }
        ans.push_str(&next.to_string());
        ans
    }
}
