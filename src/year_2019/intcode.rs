use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::sync::mpsc::{self, Receiver, Sender};
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
enum State {
    Running,
    Waiting,
    Halted,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    arg_in_1: Option<isize>,
    arg_in_2: Option<isize>,
    arg_out: Option<isize>,
}

#[derive(Debug)]
pub struct Computer {
    input_channel: Receiver<isize>,
    memory: FxHashMap<usize, isize>,
    instruction_pointer: usize,
    output_channel: Sender<isize>,
    offset: isize,
    state: State,
}

impl Computer {
    pub fn new(s: &str, input_receiver: Receiver<isize>) -> (Self, Receiver<isize>) {
        let memory: FxHashMap<usize, isize> = s
            .split(',')
            .map(|s| s.parse().unwrap())
            .enumerate()
            .collect();
        let (output_sender, output_receiver) = mpsc::channel();
        (
            Computer {
                input_channel: input_receiver,
                memory,
                instruction_pointer: 0,
                output_channel: output_sender,
                offset: 0,
                state: State::Waiting,
            },
            output_receiver,
        )
    }

    pub fn start(&mut self) {
        let signal = self.input_channel.recv().unwrap();
        if signal == isize::MIN {
            self.state = State::Running;
        }
        while self.state != State::Halted {
            self.step();
        }
    }
    pub fn read_memory(&self, address: &isize) -> isize {
        let address = *address as usize;
        self.memory.get(&address).copied().unwrap_or(0)
    }

    pub fn set_memory(&mut self, address: usize, value: isize) {
        let address = address;
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

    pub fn is_halted(&self) -> bool {
        self.state == State::Halted
    }

    pub fn is_waiting(&self) -> bool {
        self.state == State::Waiting
    }

    fn step(&mut self) {
        if self.is_halted() {
            return;
        }
        let instr = self.get_current_instr();
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
                let to_write = self.input_channel.recv().unwrap();
                self.write(instr.arg_in_1.unwrap(), mode_1, to_write);
                self.instruction_pointer += 2;
            }
            Opcode::Output(mode_1) => {
                let _ = self
                    .output_channel
                    .send(self.read(instr.arg_in_1.unwrap(), mode_1));
                self.instruction_pointer += 2;
            }
            Opcode::Halt => {
                self.state = State::Halted;
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
    }

    pub fn run_to_halt(&mut self) {
        while self.state != State::Halted {
            self.step();
        }
        self.output_channel.send(isize::MAX).unwrap();
    }
}
