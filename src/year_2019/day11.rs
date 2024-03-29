use std::time::{Duration, Instant};

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

const LINES: &str = include_str!("../../problem_inputs_2019/day_11.txt");

pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    let now = Instant::now();
    let mut tiles: FxHashMap<Position, isize> = FxHashMap::default();
    let mut painted: FxHashSet<Position> = FxHashSet::default();
    let mut robot = Robot {
        pos: (0, 0),
        facing: Direction::Up,
    };
    let mut history = [None; 100];
    loop {
        let old_color = tiles.get(&robot.pos).copied().unwrap_or(0);
        let output = &Computer::new(LINES, old_color).run_to_halt()[0..2];
        let new_color = output[0];
        let turn = output[1];
        tiles.insert(robot.pos, new_color);
        if old_color != new_color {
            painted.insert(robot.pos);
        }
        history[9] = Some(painted.len());
        history.rotate_left(1);
        if history.iter().unique().count() == 1 {
            break;
        }
        robot.facing = match turn {
            0 => match robot.facing {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            1 => match robot.facing {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            _ => panic!("Invalid turn"),
        };
        robot.pos = match robot.facing {
            Direction::Up => (robot.pos.0, robot.pos.1 + 1),
            Direction::Down => (robot.pos.0, robot.pos.1 - 1),
            Direction::Left => (robot.pos.0 - 1, robot.pos.1),
            Direction::Right => (robot.pos.0 + 1, robot.pos.1),
        };
    }
    dbg!(painted.len());
    (0, now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
type Position = (isize, isize);
struct Robot {
    pos: Position,
    facing: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AddressMode {
    Position,
    Immediate,
    Relative,
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
    AdjustOffset(AddressMode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    arg_in_1: Option<isize>,
    arg_in_2: Option<isize>,
    arg_out: Option<isize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Computer {
    input: isize,
    memory: FxHashMap<usize, isize>,
    instruction_pointer: usize,
    output: Vec<isize>,
    debug_instrs_history: Vec<Instruction>,
    offset: isize,
}

impl Computer {
    fn new(s: &str, input: isize) -> Self {
        let memory: FxHashMap<usize, isize> = s
            .split(',')
            .map(|s| s.parse().unwrap())
            .enumerate()
            .collect();
        Computer {
            input,
            memory,
            instruction_pointer: 0,
            output: Vec::new(),
            debug_instrs_history: Vec::new(),
            offset: 0,
        }
    }

    fn read_memory(&self, address: &isize) -> isize {
        let address = *address as usize;
        self.memory.get(&address).copied().unwrap_or(0)
    }

    fn set_memory(&mut self, address: usize, value: isize) {
        let address = address as usize;
        self.memory.insert(address, value);
    }

    fn read(&self, address: isize, address_mode: AddressMode) -> isize {
        match address_mode {
            AddressMode::Position => self.read_memory(&address),
            AddressMode::Relative => self.read_memory(&(address + self.offset)),
            AddressMode::Immediate => address,
        }
    }
    fn write(&mut self, address: isize, address_mode: AddressMode, val: isize) {
        let target = match address_mode {
            AddressMode::Position => address as usize,
            AddressMode::Relative => (address + self.offset) as usize,
            AddressMode::Immediate => panic!("Invalid address mode"),
        };
        self.set_memory(target, val);
    }
    fn get_current_instr(&self) -> Instruction {
        let instr_str = self.memory[&self.instruction_pointer].to_string();
        let raw_opcode = instr_str.chars().rev().collect_vec();
        let opcode = raw_opcode[0].to_digit(10).unwrap();
        let discrim = raw_opcode.get(1).unwrap_or(&'0').to_digit(10).unwrap();
        let address_mode_1 = match raw_opcode.get(2) {
            Some('1') => AddressMode::Immediate,
            Some('2') => AddressMode::Relative,
            _ => AddressMode::Position,
        };
        let address_mode_2 = match raw_opcode.get(3) {
            Some('1') => AddressMode::Immediate,
            Some('2') => AddressMode::Relative,
            _ => AddressMode::Position,
        };
        let address_mode_3 = match raw_opcode.get(4) {
            Some('1') => panic!("cannot write in immediate mode"),
            Some('2') => AddressMode::Relative,
            _ => AddressMode::Position,
        };
        let opcode = match opcode {
            1 => Opcode::Add(address_mode_1, address_mode_2, address_mode_3),
            2 => Opcode::Mul(address_mode_1, address_mode_2, address_mode_3),
            3 => Opcode::Set(address_mode_1),
            4 => Opcode::Output(address_mode_1),
            5 => Opcode::JIfTrue(address_mode_1, address_mode_2),
            6 => Opcode::JIfFalse(address_mode_1, address_mode_2),
            7 => Opcode::LessThan(address_mode_1, address_mode_2, address_mode_3),
            8 => Opcode::Equals(address_mode_1, address_mode_2, address_mode_3),
            9 => {
                if discrim == 9 {
                    Opcode::Halt
                } else {
                    Opcode::AdjustOffset(address_mode_1)
                }
            }
            _ => panic!("Invalid opcode"),
        };
        let arg_in_1 = self.memory.get(&(self.instruction_pointer + 1)).copied();
        let arg_in_2 = self.memory.get(&(self.instruction_pointer + 2)).copied();
        let arg_out = self.memory.get(&(self.instruction_pointer + 3)).copied();
        Instruction {
            opcode,
            arg_in_1,
            arg_in_2,
            arg_out,
        }
    }
    fn step(&mut self) -> Option<Vec<isize>> {
        let instr = self.get_current_instr();
        // dbg!(instr);
        match instr.opcode {
            Opcode::Add(mode_1, mode_2, mode_3) => {
                let arg_in_1 = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_in_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                self.write(instr.arg_out.unwrap(), mode_3, arg_in_1 + arg_in_2);
                self.instruction_pointer += 4;
            }
            Opcode::Mul(mode_1, mode_2, mode_3) => {
                let arg_in_1 = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_in_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                self.write(instr.arg_out.unwrap(), mode_3, arg_in_1 * arg_in_2);
                self.instruction_pointer += 4;
            }
            Opcode::Set(mode_1) => {
                self.write(instr.arg_in_1.unwrap(), mode_1, self.input);
                self.instruction_pointer += 2;
            }
            Opcode::Output(mode_1) => {
                self.output.push(self.read(instr.arg_in_1.unwrap(), mode_1));
                // dbg!(self.output);
                if self.output.len() == 2 {
                    return Some(self.output.clone());
                }
                self.instruction_pointer += 2;
            }
            Opcode::Halt => {
                return Some(self.output.clone());
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
            Opcode::LessThan(mode_1, mode_2, mode_3) => {
                let arg_1 = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                self.write(instr.arg_out.unwrap(), mode_3, isize::from(arg_1 < arg_2));
                self.instruction_pointer += 4;
            }
            Opcode::Equals(mode_1, mode_2, mode_3) => {
                let arg_1 = self.read(instr.arg_in_1.unwrap(), mode_1);
                let arg_2 = self.read(instr.arg_in_2.unwrap(), mode_2);
                self.write(instr.arg_out.unwrap(), mode_3, isize::from(arg_1 == arg_2));
                self.instruction_pointer += 4;
            }
            Opcode::AdjustOffset(mode_1) => {
                self.offset += self.read(instr.arg_in_1.unwrap(), mode_1);
                self.instruction_pointer += 2;
            }
        }
        None
    }
    fn run_to_halt(&mut self) -> Vec<isize> {
        loop {
            let out = self.step();
            if let Some(x) = out {
                return x;
            }
        }
    }
}
