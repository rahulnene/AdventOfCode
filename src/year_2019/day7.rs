use std::time::{Duration, Instant};

use itertools::Itertools;

const LINES: &str = include_str!("../../problem_inputs_2019/day_7.txt");

pub fn solution() -> ((i32, Duration), (i32, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (i32, Duration) {
    let now = Instant::now();
    let mut max_val = 0;
    let all_phases_permutations = (0..5).permutations(5);
    for perm in all_phases_permutations {
        let mut input = 0;
        for phase in perm {
            let mut comp = Amplifier::new(LINES, input, phase);
            input = comp.run_to_halt();
        }
        max_val = max_val.max(input);
    }

    (max_val, now.elapsed())
}

fn solve02() -> (i32, Duration) {
    let now = Instant::now();
    let mut max_val = 0;
    let all_phases_permutations = (5..10).permutations(5);
    for perm in all_phases_permutations {
        let mut input = 0;
        for phase in perm {
            let mut comp = Amplifier::new(LINES, input, phase);
            input = comp.run_to_halt();
        }
        max_val = max_val.max(input);
    }

    (max_val, now.elapsed())
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AddressMode {
    Position,
    Immediate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Opcode {
    Add(AddressMode, AddressMode, AddressMode),
    Mul(AddressMode, AddressMode, AddressMode),
    Halt,
    Set(AddressMode),
    Output(AddressMode),
    JIfTrue(AddressMode, AddressMode),
    JIfFalse(AddressMode, AddressMode),
    LessThan(AddressMode, AddressMode, AddressMode),
    Equals(AddressMode, AddressMode, AddressMode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    arg_in_1: Option<i32>,
    arg_in_2: Option<i32>,
    arg_out: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Amplifier {
    input_stream: Vec<i32>,
    memory: Vec<i32>,
    instruction_pointer: usize,
    output: Option<i32>,
}

impl Amplifier {
    fn new(s: &str, input: i32, phase: i32) -> Self {
        Amplifier {
            input_stream: vec![input, phase],
            memory: s.split(',').map(|s| s.parse().unwrap()).collect(),
            instruction_pointer: 0,
            output: None,
        }
    }

    fn read(&self, address: i32, address_mode: AddressMode) -> i32 {
        match address_mode {
            AddressMode::Position => self.memory[address as usize],
            AddressMode::Immediate => address,
        }
    }
    fn get_current_instr(&self) -> Instruction {
        let instr_str = self.memory[self.instruction_pointer].to_string();
        let raw_opcode = instr_str.chars().rev().collect_vec();
        let opcode = raw_opcode[0].to_digit(10).unwrap();
        let address_mode_1 = match raw_opcode.get(2) {
            Some('1') => AddressMode::Immediate,
            _ => AddressMode::Position,
        };
        let address_mode_2 = match raw_opcode.get(3) {
            Some('1') => AddressMode::Immediate,
            _ => AddressMode::Position,
        };
        let address_mode_3 = AddressMode::Position;
        let opcode = match opcode {
            1 => Opcode::Add(address_mode_1, address_mode_2, address_mode_3),
            2 => Opcode::Mul(address_mode_1, address_mode_2, address_mode_3),
            3 => Opcode::Set(address_mode_1),
            4 => Opcode::Output(address_mode_1),
            5 => Opcode::JIfTrue(address_mode_1, address_mode_2),
            6 => Opcode::JIfFalse(address_mode_1, address_mode_2),
            7 => Opcode::LessThan(address_mode_1, address_mode_2, address_mode_3),
            8 => Opcode::Equals(address_mode_1, address_mode_2, address_mode_3),
            9 => Opcode::Halt,
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
    fn step(&mut self) -> Option<i32> {
        let instr = self.get_current_instr();
        match instr.opcode {
            Opcode::Add(mode_1, mode_2, _) => {
                let arg_in_1 = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_in_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                let arg_out = instr.arg_out.unwrap();
                self.memory[arg_out as usize] = arg_in_1 + arg_in_2;
                if arg_out != self.instruction_pointer as i32 {
                    self.instruction_pointer += 4;
                }
            }
            Opcode::Mul(mode_1, mode_2, _) => {
                let arg_in_1 = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_in_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                let arg_out = instr.arg_out.unwrap();
                self.memory[arg_out as usize] = arg_in_1 * arg_in_2;
                if arg_out != self.instruction_pointer as i32 {
                    self.instruction_pointer += 4;
                }
            }
            Opcode::Set(_) => {
                let save_pos = instr.arg_in_1.unwrap();
                let valid_input = self.input_stream.pop().unwrap();
                self.memory[save_pos as usize] = valid_input;
                if save_pos as usize != self.instruction_pointer {
                    self.instruction_pointer += 2;
                }
            }
            Opcode::Output(mode_1) => {
                self.output = Some(self.read(instr.arg_in_1.unwrap(), mode_1));
                self.instruction_pointer += 2;
            }
            Opcode::Halt => {
                return self.output;
            }
            Opcode::JIfTrue(mode_1, mode_2) => {
                let arg = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                if arg != 0 {
                    self.instruction_pointer = arg_2 as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            Opcode::JIfFalse(mode_1, mode_2) => {
                let arg = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                if arg == 0 {
                    self.instruction_pointer = arg_2 as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            Opcode::LessThan(mode_1, mode_2, _) => {
                let arg_1 = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                let arg_out = instr.arg_out.unwrap();
                self.memory[arg_out as usize] = i32::from(arg_1 < arg_2);
                if arg_out as usize != self.instruction_pointer {
                    self.instruction_pointer += 4;
                }
            }
            Opcode::Equals(mode_1, mode_2, _) => {
                let arg_1 = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                let arg_out = instr.arg_out.unwrap();
                self.memory[arg_out as usize] = i32::from(arg_1 == arg_2);
                if arg_out as usize != self.instruction_pointer {
                    self.instruction_pointer += 4;
                }
            }
        }
        None
    }
    fn run_to_halt(&mut self) -> i32 {
        loop {
            let out = self.step();
            if let Some(x) = out {
                return x;
            }
        }
    }
}
