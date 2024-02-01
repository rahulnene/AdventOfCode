use std::time::{Duration, Instant};

use rustc_hash::FxHashMap;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../problem_inputs_2016/day_25.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let instructions: Vec<_> = lines.lines().map(Instruction::parse).collect();
    for a in 0.. {
        let mut computer = Computer::new(a);
        while computer.pc < instructions.len() {
            if let Some(result) = computer.compute(&instructions[computer.pc]) {
                if result {
                    // dbg!(a);
                    return (a as usize, now.elapsed());
                } else {
                    continue;
                }
            } else {
                break;
            }
        }
    }
    (0, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Computer {
    registers: FxHashMap<char, isize>,
    pc: usize,
    out_buffer: Vec<isize>,
}

impl Computer {
    fn new(a: isize) -> Self {
        let mut registers = FxHashMap::default();
        registers.insert('a', a);
        registers.insert('b', 0);
        registers.insert('c', 0);
        registers.insert('d', 0);
        Self {
            registers,
            pc: 0,
            out_buffer: Vec::new(),
        }
    }

    fn check_clock(&self) -> bool {
        let mut prev = self.out_buffer[0];
        for i in 1..self.out_buffer.len() {
            if prev == self.out_buffer[i] {
                return false;
            }
            prev = self.out_buffer[i];
        }
        true
    }

    fn compute(&mut self, instr: &Instruction) -> Option<bool> {
        if self.out_buffer.len() == 10 {
            if self.check_clock() {
                return Some(true);
            } else {
                return None;
            }
        }
        match instr {
            Instruction::Cpy(Operand::Immediate(x), Operand::Register(y)) => {
                self.registers.insert(*y, *x);
                self.pc += 1;
            }
            Instruction::Cpy(Operand::Register(x), Operand::Register(y)) => {
                let x = self.registers.get(x).unwrap();
                self.registers.insert(*y, *x);
                self.pc += 1;
            }
            Instruction::Inc(Operand::Register(x)) => {
                let x_val = self.registers.get(x).unwrap();
                self.registers.insert(*x, x_val + 1);
                self.pc += 1;
            }
            Instruction::Dec(Operand::Register(x)) => {
                let x_val = self.registers.get(x).unwrap();
                self.registers.insert(*x, x_val - 1);
                self.pc += 1;
            }
            Instruction::Jnz(Operand::Immediate(x), Operand::Register(y)) => {
                let y_val = self.registers.get(y).unwrap();
                if *x != 0 {
                    self.pc = (self.pc as isize + *y_val) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jnz(Operand::Register(x), Operand::Register(y)) => {
                let x_val = self.registers.get(x).unwrap();
                let y_val = self.registers.get(y).unwrap();
                if *x_val != 0 {
                    self.pc = (self.pc as isize + *y_val) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jnz(Operand::Immediate(x), Operand::Immediate(y)) => {
                if *x != 0 {
                    self.pc = (self.pc as isize + *y) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jnz(Operand::Register(x), Operand::Immediate(y)) => {
                let x_val = self.registers.get(x).unwrap();
                if *x_val != 0 {
                    self.pc = (self.pc as isize + *y) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Out(x) => {
                self.out_buffer.push(match x {
                    Operand::Immediate(x) => *x,
                    Operand::Register(x) => *self.registers.get(x).unwrap(),
                });
                self.pc += 1
            }
            _ => {
                self.pc += 1;
            }
        };
        Some(false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Out(Operand),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        match parts.next().unwrap() {
            "cpy" => Self::Cpy(
                Operand::parse(parts.next().unwrap()),
                Operand::parse(parts.next().unwrap()),
            ),
            "inc" => Self::Inc(Operand::parse(parts.next().unwrap())),
            "dec" => Self::Dec(Operand::parse(parts.next().unwrap())),
            "jnz" => Self::Jnz(
                Operand::parse(parts.next().unwrap()),
                Operand::parse(parts.next().unwrap()),
            ),
            "out" => Self::Out(Operand::parse(parts.next().unwrap())),
            _ => panic!("Unknown instruction"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Operand {
    Register(char),
    Immediate(isize),
}

impl Operand {
    fn parse(s: &str) -> Self {
        if let Ok(i) = s.parse::<isize>() {
            Self::Immediate(i)
        } else {
            Self::Register(s.chars().next().unwrap())
        }
    }
}
