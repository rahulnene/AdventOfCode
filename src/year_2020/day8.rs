use std::fmt::Debug;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2020/day_8.txt");

pub fn solution() -> ((isize, Duration), (usize, Duration)) {
    let mut program = Program::new();
    let mut jmp_instr_locations = Vec::new();
    let mut nop_instr_locations = Vec::new();
    for (loc, line) in LINES.lines().enumerate() {
        let instr = Instruction::new(line);
        if instr.operation == Operation::Jmp {
            jmp_instr_locations.push(loc);
        } else if instr.operation == Operation::Nop {
            nop_instr_locations.push(loc);
        }
        program.instructions.push(instr);
    }
    (
        solve01(&program),
        solve02(&program, &jmp_instr_locations, &nop_instr_locations),
    )
}

fn solve01(program: &Program) -> (isize, Duration) {
    let now = Instant::now();
    let mut program = program.clone();
    while program.instruction_pointer < program.instructions.len() {
        if let Some(x) = program.step() {
            return (x, now.elapsed());
        };
    }
    (0, now.elapsed())
}

fn solve02(
    program: &Program,
    jmp_instr_locations: &[usize],
    nop_instr_locations: &[usize],
) -> (usize, Duration) {
    let now = Instant::now();
    for jmp_loc in jmp_instr_locations {
        let mut program = program.clone();
        program.instructions[*jmp_loc].operation = Operation::Nop;
        if let Some(x) = halts(&program) {
            return (x as usize, now.elapsed());
        }
    }
    for nop_loc in nop_instr_locations {
        let mut program = program.clone();
        program.instructions[*nop_loc].operation = Operation::Jmp;
        if let Some(x) = halts(&program) {
            return (x as usize, now.elapsed());
        }
    }
    (0, now.elapsed())
}

#[derive(Clone, Copy)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let mut split = line.split(' ');
        let operation = match split.next().unwrap() {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => panic!("Invalid operation"),
        };
        let argument = split.next().unwrap().parse::<isize>().unwrap();
        Instruction {
            operation,
            argument,
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operation = match self.operation {
            Operation::Nop => "nop",
            Operation::Acc => "acc",
            Operation::Jmp => "jmp",
        };
        let sign = if self.argument > 0 { "+" } else { "" };
        write!(f, "{operation} {sign}{}", self.argument)
    }
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
    accumulator: isize,
    instruction_pointer: usize,
    visited_instructions: Vec<usize>,
}

impl Program {
    fn new() -> Self {
        Program {
            instructions: Vec::new(),
            accumulator: 0,
            instruction_pointer: 0,
            visited_instructions: Vec::new(),
        }
    }

    fn step(&mut self) -> Option<isize> {
        let instruction = self.instructions[self.instruction_pointer];
        match instruction.operation {
            Operation::Nop => self.instruction_pointer += 1,
            Operation::Acc => {
                self.accumulator += instruction.argument;
                self.instruction_pointer += 1;
            }
            Operation::Jmp => {
                if instruction.argument > 0 {
                    self.instruction_pointer += instruction.argument as usize;
                } else {
                    self.instruction_pointer -= instruction.argument.unsigned_abs();
                }
            }
        }
        if self
            .visited_instructions
            .contains(&self.instruction_pointer)
        {
            Some(self.accumulator)
        } else {
            self.visited_instructions.push(self.instruction_pointer);
            None
        }
    }
}

fn halts(program: &Program) -> Option<isize> {
    let mut program = program.clone();
    while program.instruction_pointer < program.instructions.len() {
        if program.step().is_some() {
            return None;
        };
    }
    Some(program.accumulator)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}
