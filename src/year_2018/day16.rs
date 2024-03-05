use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2018/day_16_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: OpCode,
    in_a: usize,
    in_b: usize,
    out_c: usize,
}

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Add(InputType, InputType),
    Mul(InputType, InputType),
    Ban(InputType, InputType),
    Bor(InputType, InputType),
    Set(InputType, InputType),
    Gt(InputType, InputType),
    Eq(InputType, InputType),
}

#[derive(Debug, Clone, Copy)]
enum InputType {
    Reg,
    Imm,
}
