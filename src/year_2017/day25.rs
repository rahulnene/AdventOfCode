use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_25.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut state_strs = lines.split("\n\n");
    let origin_strs = state_strs.next().unwrap().split('\n').collect_vec();
    let rules = state_strs.map(parse).collect_vec();
    let first_state = origin_strs[0]
        .trim()
        .replace("Begin in state ", "")
        .replace('.', "")
        .chars()
        .next()
        .unwrap();
    let max_steps: usize = origin_strs[1]
        .trim()
        .replace("Perform a diagnostic checksum after ", "")
        .replace(" steps.", "")
        .parse()
        .unwrap();
    let mut turing = Turing::new(first_state, rules);
    for _ in 0..max_steps {
        turing.step();
    }
    (turing.diagnostic_checksum(), now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone)]
struct Turing {
    tape: Vec<bool>,
    cursor: usize,
    state: char,
    rules: Vec<Condition>,
}

impl Turing {
    fn new(start_state: char, conditions: Vec<Condition>) -> Self {
        Self {
            tape: vec![false; 40_000_000],
            cursor: 20_000_000,
            state: start_state,
            rules: conditions,
        }
    }

    fn diagnostic_checksum(&self) -> usize {
        self.tape.iter().filter(|&&b| b).count()
    }

    fn step(&mut self) {
        let relevant_rule = self
            .rules
            .iter()
            .find(|rule| rule.name == self.state)
            .unwrap();
        let tape_val = self.tape[self.cursor];
        let action = relevant_rule
            .actions
            .iter()
            .find(|action| action.cur_val == tape_val)
            .unwrap();
        self.tape[self.cursor] = action.write_val;
        match action.move_dir {
            Direction::Left => self.cursor -= 1,
            Direction::Right => self.cursor += 1,
        }
        self.state = action.next_state;
    }
}

#[derive(Debug, Clone)]
struct Action {
    cur_val: bool,
    write_val: bool,
    move_dir: Direction,
    next_state: char,
}

#[derive(Debug, Clone)]
struct Condition {
    name: char,
    actions: Vec<Action>,
}

fn parse(input: &str) -> Condition {
    let lines: Vec<&str> = input.lines().collect();
    let state_name = lines[0].trim().replace("In state ", "").replace(':', "");

    let mut actions = Vec::new();

    for i in (1..lines.len()).step_by(4) {
        let cur_val = lines[i]
            .trim()
            .replace("If the current value is ", "")
            .replace(':', "")
            .parse::<usize>()
            .unwrap()
            == 1;
        let write_val = lines[i + 1]
            .trim()
            .replace("- Write the value ", "")
            .replace('.', "")
            .parse::<usize>()
            .unwrap()
            == 1;
        let move_dir = match lines[i + 2]
            .trim()
            .replace("- Move one slot to the ", "")
            .replace('.', "")
            .as_str()
        {
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => unreachable!(),
        };
        let next_state = lines[i + 3]
            .trim()
            .replace("- Continue with state ", "")
            .replace('.', "")
            .chars()
            .next()
            .unwrap();

        actions.push(Action {
            cur_val,
            write_val,
            move_dir,
            next_state,
        });
    }

    Condition {
        name: state_name.chars().next().unwrap(),
        actions,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}
