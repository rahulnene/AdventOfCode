use std::fmt::Debug;

use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_8.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut program = Program::new();
    for line in lines.lines() {
        program.instructions.push(Instruction::new(line));
    }
    // dbg!(program.instructions);
    while program.instruction_pointer < program.instructions.len() {
        program.step();
    }
    0
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Clone, Copy)]
struct Instruction {
    operation: u8,
    argument: i32,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let mut split = line.split(' ');
        let operation = match split.next().unwrap() {
            "nop" => 0,
            "acc" => 1,
            "jmp" => 2,
            _ => panic!("Invalid operation"),
        };
        let argument = split.next().unwrap().parse::<i32>().unwrap();
        let positive = argument >= 0;
        Instruction {
            operation,
            argument,
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operation = match self.operation {
            0 => "nop",
            1 => "acc",
            2 => "jmp",
            _ => panic!("Invalid operation"),
        };
        let sign = if self.argument > 0 { "+" } else { "" };
        write!(f, "{operation} {sign}{}", self.argument)
    }
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
    accumulator: i32,
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

    fn step(&mut self) {
        let instruction = self.instructions[self.instruction_pointer];
        // dbg!(&self, &instruction);
        // println!("-------------");
        match instruction.operation {
            0 => self.instruction_pointer += 1,
            1 => {
                self.accumulator += instruction.argument;
                self.instruction_pointer += 1;
            }
            2 => {
                // dbg!(self.instruction_pointer, instruction.argument);
                if instruction.argument > 0 {
                    self.instruction_pointer += instruction.argument as usize;
                } else {
                    self.instruction_pointer -= instruction.argument.abs() as usize;
                }
            }
            _ => panic!("Invalid operation"),
        }
        if !self
            .visited_instructions
            .contains(&self.instruction_pointer)
        {
            self.visited_instructions.push(self.instruction_pointer);
        } else {
            dbg!(self.accumulator);
            panic!("Infinite loop");
        }
    }
}
