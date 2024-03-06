use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2019/day_2.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut comp = Computer::new(LINES, 12, 2);
    comp.run_to_halt();
    (comp.memory[0], now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    for a in 0..100 {
        for b in 0..100 {
            let mut comp = Computer::new(LINES, a, b);
            comp.run_to_halt();
            if comp.memory[0] == 19690720 {
                return (100 * a + b, now.elapsed());
            }
        }
    }
    (0, now.elapsed())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Opcode {
    Add,
    Multiply,
    Halt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    arg_in_1: Option<usize>,
    arg_in_2: Option<usize>,
    arg_out: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Computer {
    memory: Vec<usize>,
    instruction_pointer: usize,
}

impl Computer {
    fn new(s: &str, init_a: usize, init_b: usize) -> Self {
        let mut memory: Vec<usize> = s.split(',').map(|s| s.parse().unwrap()).collect();
        memory[1] = init_a;
        memory[2] = init_b;
        Computer {
            memory,
            instruction_pointer: 0,
        }
    }
    fn get_current_instr(&self) -> Instruction {
        let opcode = match self.memory[self.instruction_pointer] {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::Halt,
            _ => panic!("Invalid opcode"),
        };
        let arg_in_1 = self.memory.get(self.instruction_pointer + 1).copied();
        let arg_in_2 = self.memory.get(self.instruction_pointer + 2).copied();
        let arg_out = self.memory.get(self.instruction_pointer + 3).copied();
        Instruction {
            opcode,
            arg_in_1,
            arg_in_2,
            arg_out,
        }
    }
    fn step(&mut self) -> bool {
        let instr = self.get_current_instr();
        match instr.opcode {
            Opcode::Add => {
                let arg_in_1 = instr.arg_in_1.unwrap();
                let arg_in_2 = instr.arg_in_2.unwrap();
                let arg_out = instr.arg_out.unwrap();
                self.memory[arg_out] = self.memory[arg_in_1] + self.memory[arg_in_2];
                self.instruction_pointer += 4;
            }
            Opcode::Multiply => {
                let arg_in_1 = instr.arg_in_1.unwrap();
                let arg_in_2 = instr.arg_in_2.unwrap();
                let arg_out = instr.arg_out.unwrap();
                self.memory[arg_out] = self.memory[arg_in_1] * self.memory[arg_in_2];
                self.instruction_pointer += 4;
            }
            Opcode::Halt => {
                return true;
            }
        }
        false
    }
    fn run_to_halt(&mut self) {
        loop {
            if self.step() {
                break;
            }
        }
    }
}
