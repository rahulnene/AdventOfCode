use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2017/day_23.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let program: Vec<Instruction> = LINES.lines().map(Instruction::from_str).collect();
    let mut comp = CPU::new(program);
    while comp.pc < comp.program.len() {
        comp.run_once();
    }
    (comp.mul_count, now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let ans = (106700..=123700)
        .step_by(17)
        .filter(|&n| !prime_check(n))
        .count();
    (ans, now.elapsed())
}

fn prime_check(num: usize) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..(f64::sqrt(num as f64) as usize + 1) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, Copy)]
enum Arg {
    Register(char),
    Value(isize),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SET(Arg, Arg),
    SUB(Arg, Arg),
    MUL(Arg, Arg),
    JNZ(Arg, Arg),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let (instr, a, b) = {
            let mut parts = s.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        };
        let a = if let Ok(v) = a.parse() {
            Arg::Value(v)
        } else {
            Arg::Register(a.chars().next().unwrap())
        };
        let b = if let Ok(v) = b.parse() {
            Arg::Value(v)
        } else {
            Arg::Register(b.chars().next().unwrap())
        };
        match instr {
            "set" => Instruction::SET(a, b),
            "sub" => Instruction::SUB(a, b),
            "mul" => Instruction::MUL(a, b),
            "jnz" => Instruction::JNZ(a, b),
            _ => panic!("Unknown instruction: {}", instr),
        }
    }
}

#[derive(Debug, Clone)]
struct CPU {
    registers: [isize; 8],
    program: Vec<Instruction>,
    pc: usize,
    mul_count: usize,
}

impl CPU {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            registers: [0; 8],
            program,
            pc: 0,
            mul_count: 0,
        }
    }

    fn get_value(&self, arg: Arg) -> isize {
        match arg {
            Arg::Register(r) => self.registers[r as usize - 'a' as usize] as isize,
            Arg::Value(v) => v,
        }
    }

    fn run_once(&mut self) {
        let instr = self.program[self.pc];
        match instr {
            Instruction::SET(Arg::Register(r), v) => {
                self.registers[r as usize - 'a' as usize] = self.get_value(v);
                self.pc += 1;
            }
            Instruction::SUB(Arg::Register(r), v) => {
                self.registers[r as usize - 'a' as usize] -= self.get_value(v);
                self.pc += 1;
            }
            Instruction::MUL(Arg::Register(r), v) => {
                self.registers[r as usize - 'a' as usize] *= self.get_value(v);
                self.mul_count += 1;
                self.pc += 1;
            }
            Instruction::JNZ(a, b) => {
                if self.get_value(a) != 0 {
                    self.pc = (self.pc as isize + self.get_value(b) - 1) as usize;
                }
                self.pc += 1;
            }
            _ => panic!("Unknown instruction: {:?}", instr),
        }
    }
}
