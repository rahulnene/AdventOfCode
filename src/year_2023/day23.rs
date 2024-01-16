use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2023/day_23_test.txt");
    match part {
        1 => solve01(lines),
        // 2 => solve(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut circle = Circle::from_str(lines);
    for _ in 0..10 {
        circle.cycle();
        circle.print_state();
    }
    0
}

#[derive(Debug, Clone)]
struct Circle {
    cups: Vec<usize>,
    picked_up: Vec<usize>,
    current: usize,
    destination: usize,
}

impl Circle {
    fn new(cups: Vec<usize>) -> Self {
        let current = cups[0];
        Self {
            cups,
            picked_up: Vec::new(),
            current,
            destination: 0,
        }
    }

    fn from_str(s: &str) -> Self {
        let cups = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect_vec();
        Self::new(cups)
    }

    fn cycle(&mut self) {
        let index = self.cups.iter().position(|&x| x == self.current).unwrap();
        self.picked_up = self.cups.drain(index + 1..index + 4).collect_vec();
        self.destination = self.current - 1;
        if self.destination == 0 {
            self.destination = 9;
        }
        while !self.cups.contains(&self.destination) {
            self.destination -= 1;
            if self.destination == 0 {
                self.destination = 9;
            }
        }
        let index = self
            .cups
            .iter()
            .position(|&x| x == self.destination)
            .unwrap();
        self.cups
            .splice(index + 1..index + 1, self.picked_up.iter().cloned());
        self.current = (index + 1) % (self.cups.len());
    }

    fn print_state(&self) {
        println!("cups: {:?}", self);
    }
}
