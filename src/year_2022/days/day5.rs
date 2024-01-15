use std::{time::{Instant, Duration}, collections::VecDeque};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2022/day_5.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stack {
    stack: VecDeque<usize>,
}

impl Stack {
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    fn push(&mut self, value: usize) {
        self.stack.push_back(value);
    }

    fn pop(&mut self) -> Option<usize> {
        self.stack.pop_back()
    }

    fn peek(&self) -> Option<usize> {
        self.stack.back().copied()
    }

    fn len(&self) -> usize {
        self.stack.len()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Instruction {
    amount: usize,
    source: usize,
    target: usize,
}

fn process(instr: Instruction, stacks: &mut [Stack]) {
    let mut source = stacks[instr.source].pop().unwrap();
    let mut target = stacks[instr.target].pop().unwrap();
    let mut amount = instr.amount;
    while amount > 0 {
        if source == target {
            stacks[source].push(source);
            source = stacks[instr.source].pop().unwrap();
            target = stacks[instr.target].pop().unwrap();
        } else {
            stacks[target].push(source);
            source = stacks[instr.source].pop().unwrap();
            target = stacks[instr.target].pop().unwrap();
        }
        amount -= 1;
    }
    stacks[instr.source].push(source);
    stacks[instr.target].push(target);
}