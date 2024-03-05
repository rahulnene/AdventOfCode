use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2018/day_21.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let instruction_pointer: usize = LINES
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let instructions: Vec<Instruction> = LINES.lines().skip(1).map(Instruction::from_str).collect();

    (
        solve(instruction_pointer, &instructions),
        (0, Duration::default()),
    )
}

fn solve(instruction_pointer: usize, instructions: &[Instruction]) -> (usize, Duration) {
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

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let opcode = match parts.next().unwrap() {
            "addr" => OpCode::Add(InputType::Reg, InputType::Reg),
            "addi" => OpCode::Add(InputType::Reg, InputType::Imm),
            "mulr" => OpCode::Mul(InputType::Reg, InputType::Reg),
            "muli" => OpCode::Mul(InputType::Reg, InputType::Imm),
            "banr" => OpCode::Ban(InputType::Imm, InputType::Imm),
            "bani" => OpCode::Ban(InputType::Reg, InputType::Imm),
            "bori" => OpCode::Bor(InputType::Imm, InputType::Imm),
            "borr" => OpCode::Bor(InputType::Reg, InputType::Reg),
            "setr" => OpCode::Set(InputType::Reg, InputType::Reg),
            "seti" => OpCode::Set(InputType::Imm, InputType::Reg),
            "gtir" => OpCode::Gt(InputType::Imm, InputType::Reg),
            "gtri" => OpCode::Gt(InputType::Reg, InputType::Imm),
            "gtrr" => OpCode::Gt(InputType::Reg, InputType::Reg),
            "eqir" => OpCode::Eq(InputType::Imm, InputType::Reg),
            "eqri" => OpCode::Eq(InputType::Reg, InputType::Imm),
            "eqrr" => OpCode::Eq(InputType::Reg, InputType::Reg),
            _ => panic!("Invalid opcode"),
        };
        let in_a = parts.next().unwrap().parse().unwrap();
        let in_b = parts.next().unwrap().parse().unwrap();
        let out_c = parts.next().unwrap().parse().unwrap();
        Self {
            opcode,
            in_a,
            in_b,
            out_c,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum InputType {
    Reg,
    Imm,
}

#[derive(Debug, Clone)]
struct CPU<'a> {
    registers: [usize; 6],
    ip_bound: usize,
    instruction_pointer: usize,
    instructions: &'a [Instruction],
}

impl<'a> CPU<'a> {
    fn new(instrs: &'a [Instruction], ip: usize, init: usize) -> Self {
        let mut registers = [0; 6];
        registers[0] = init;
        Self {
            registers,
            ip_bound: ip,
            instruction_pointer: 0,
            instructions: instrs,
        }
    }

    fn read_value(&self, input: InputType, value: usize) -> usize {
        match input {
            InputType::Reg => self.registers[value],
            InputType::Imm => value,
        }
    }

    fn step(&mut self) -> Option<usize> {
        let instruction = self.instructions.get(self.instruction_pointer)?;
        self.registers[self.ip_bound] = self.instruction_pointer;
        self.registers[instruction.out_c] = match instruction.opcode {
            OpCode::Add(a, b) => {
                let a = self.read_value(a, instruction.in_a);
                let b = self.read_value(b, instruction.in_b);
                a + b
            }
            OpCode::Mul(a, b) => {
                let a = self.read_value(a, instruction.in_a);
                let b = self.read_value(b, instruction.in_b);
                a * b
            }
            OpCode::Ban(a, b) => {
                let a = self.read_value(a, instruction.in_a);
                let b = self.read_value(b, instruction.in_b);
                a & b
            }
            OpCode::Bor(a, b) => {
                let a = self.read_value(a, instruction.in_a);
                let b = self.read_value(b, instruction.in_b);
                a | b
            }
            OpCode::Set(a, _) => {
                let a = self.read_value(a, instruction.in_a);
                a
            }
            OpCode::Gt(a, b) => {
                let a = self.read_value(a, instruction.in_a);
                let b = self.read_value(b, instruction.in_b);
                (a > b) as usize
            }
            OpCode::Eq(a, b) => {
                let a = self.read_value(a, instruction.in_a);
                let b = self.read_value(b, instruction.in_b);
                (a == b) as usize
            }
        };
        None
    }
}
