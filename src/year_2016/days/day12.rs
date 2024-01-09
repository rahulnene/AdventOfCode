use std::time::{Instant, Duration};

use fxhash::FxHashMap;

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2016/day_12.txt");
    let instructions = lines
        .lines()
        .map(|l| Instruction::parse(l))
        .collect::<Vec<_>>();
    (solve01(&instructions), solve02(&instructions))
}

fn solve01(instructions: &[Instruction]) -> (usize, Duration) {
    let now = Instant::now();
    let max_pc = instructions.len();
    let mut computer = Computer::new(0,0,0,0);
    while computer.pc < max_pc {
        let instr = &instructions[computer.pc];
        computer.compute(instr);
        // dbg!(instr, &computer.pc);
    }
    (*computer.registers.get(&'a').unwrap() as usize, now.elapsed())
}

fn solve02(instructions: &[Instruction]) -> (usize, Duration) {
    let now = Instant::now();
    let max_pc = instructions.len();
    let mut computer = Computer::new(0,0,1,0);
    while computer.pc < max_pc {
        let instr = &instructions[computer.pc];
        computer.compute(instr);
        // dbg!(instr, &computer.pc);
    }
    (*computer.registers.get(&'a').unwrap() as usize, now.elapsed())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Computer {
    registers: FxHashMap<char, i32>,
    pc: usize,
}

impl Computer {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        let mut registers = FxHashMap::default();
        registers.insert('a', a);
        registers.insert('b', b);
        registers.insert('c', c);
        registers.insert('d', d);
        Self { registers, pc: 0 }
    }

    fn compute(&mut self, instr: &Instruction) {
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
                    self.pc = (self.pc as i32 + *y_val) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jnz(Operand::Register(x), Operand::Register(y)) => {
                let x_val = self.registers.get(x).unwrap();
                let y_val = self.registers.get(y).unwrap();
                if *x_val != 0 {
                    self.pc = (self.pc as i32 + *y_val) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jnz(Operand::Immediate(x), Operand::Immediate(y)) => {
                if *x != 0 {
                    self.pc = (self.pc as i32 + *y) as usize;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::Jnz(Operand::Register(x), Operand::Immediate(y)) => {
                let x_val = self.registers.get(x).unwrap();
                if *x_val != 0 {
                    self.pc = (self.pc as i32 + *y) as usize;
                } else {
                    self.pc += 1;
                }
            }
            _ => panic!("Unknown instruction: {:?}", instr),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
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
            _ => panic!("Unknown instruction"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Operand {
    Register(char),
    Immediate(i32),
}

impl Operand {
    fn parse(s: &str) -> Self {
        if let Ok(i) = s.parse::<i32>() {
            Self::Immediate(i)
        } else {
            Self::Register(s.chars().next().unwrap())
        }
    }
}
