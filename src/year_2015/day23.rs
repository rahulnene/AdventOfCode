use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let instructions = include_str!("../../../problem_inputs_2015/day_23.txt");
    let instructions = instructions
        .lines()
        .map(|line| Instruction::from_str(line))
        .collect::<Vec<_>>();
    (solve01(&instructions), solve02(&instructions))
}

fn solve01<'a>(instrs: &'a [Instruction]) -> (usize, Duration) {
    let now = Instant::now();

    let mut cpu = CPU::new(0, 0, instrs);
    cpu.process();
    (cpu.b, now.elapsed())
}

fn solve02<'a>(instrs: &'a [Instruction]) -> (usize, Duration) {
    let now = Instant::now();
    let mut cpu = CPU::new(1, 0, instrs);
    cpu.process();
    (cpu.b, now.elapsed())
}

#[derive(Debug, Clone)]
struct CPU<'a> {
    a: usize,
    b: usize,
    pc: usize,
    instructions: &'a [Instruction],
}

impl<'a> CPU<'a> {
    fn new(a: usize, b: usize, instructions: &'a [Instruction]) -> Self {
        Self {
            a,
            b,
            pc: 0,
            instructions,
        }
    }

    fn process(&mut self) {
        while self.pc < self.instructions.len() {
            match self.instructions[self.pc] {
                Instruction::Hlf(register) => match register {
                    'a' => self.a /= 2,
                    'b' => self.b /= 2,
                    _ => panic!("Invalid register"),
                },
                Instruction::Tpl(register) => match register {
                    'a' => self.a *= 3,
                    'b' => self.b *= 3,
                    _ => panic!("Invalid register"),
                },
                Instruction::Inc(register) => match register {
                    'a' => self.a += 1,
                    'b' => self.b += 1,
                    _ => panic!("Invalid register"),
                },
                Instruction::Jmp(offset) => {
                    self.pc = (self.pc as isize + offset) as usize;
                    continue;
                }
                Instruction::Jie(register, offset) => match register {
                    'a' => {
                        if self.a % 2 == 0 {
                            self.pc = (self.pc as isize + offset) as usize;
                            continue;
                        }
                    }
                    'b' => {
                        if self.b % 2 == 0 {
                            self.pc = (self.pc as isize + offset) as usize;
                            continue;
                        }
                    }
                    _ => panic!("Invalid register"),
                },
                Instruction::Jio(register, offset) => match register {
                    'a' => {
                        if self.a == 1 {
                            self.pc = (self.pc as isize + offset) as usize;
                            continue;
                        }
                    }
                    'b' => {
                        if self.b == 1 {
                            self.pc = (self.pc as isize + offset) as usize;
                            continue;
                        }
                    }
                    _ => panic!("Invalid register"),
                },
            }
            self.pc += 1;
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(isize),
    Jie(char, isize),
    Jio(char, isize),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let instruction = iter.next().unwrap();
        match instruction {
            "hlf" => Instruction::Hlf(iter.next().unwrap().chars().next().unwrap()),
            "tpl" => Instruction::Tpl(iter.next().unwrap().chars().next().unwrap()),
            "inc" => Instruction::Inc(iter.next().unwrap().chars().next().unwrap()),
            "jmp" => Instruction::Jmp(iter.next().unwrap().parse().unwrap()),
            "jie" => {
                let register = iter.next().unwrap().chars().next().unwrap();
                let offset = iter.next().unwrap().parse().unwrap();
                Instruction::Jie(register, offset)
            }
            "jio" => {
                let register = iter.next().unwrap().chars().next().unwrap();
                let offset = iter.next().unwrap().parse().unwrap();
                Instruction::Jio(register, offset)
            }
            _ => panic!("Invalid instruction"),
        }
    }
}
