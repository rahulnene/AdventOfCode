use itertools::Itertools;
use rustc_hash::FxHashMap;

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
pub struct Computer {
    input: isize,
    memory: FxHashMap<usize, isize>,
    instruction_pointer: usize,
    output: Option<isize>,
    debug_instrs_history: Vec<Instruction>,
    offset: isize,
}

impl Computer {
    pub fn new(s: &str, input: isize) -> Self {
        let memory: FxHashMap<usize, isize> = s
            .split(',')
            .map(|s| s.parse().unwrap())
            .enumerate()
            .collect();
        Computer {
            input,
            memory,
            instruction_pointer: 0,
            output: None,
            debug_instrs_history: Vec::new(),
            offset: 0,
        }
    }

    pub fn read_memory(&self, address: &isize) -> isize {
        let address = *address as usize;
        self.memory.get(&address).copied().unwrap_or(0)
    }

    pub fn set_memory(&mut self, address: usize, value: isize) {
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
    fn step(&mut self) -> Option<isize> {
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
                self.output = Some(self.read(instr.arg_in_1.unwrap(), mode_1));
                // dbg!(self.output);
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
    pub fn run_to_halt(&mut self) -> isize {
        loop {
            let out = self.step();
            if let Some(x) = out {
                return x;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut comp = Computer::new("109,-1,4,1,99", 1);
        let ans = comp.run_to_halt();
        assert_eq!(ans, -1);
    }
    #[test]
    fn test_2() {
        let mut comp = Computer::new("109,-1,104,1,99", 1);
        let ans = comp.run_to_halt();
        assert_eq!(ans, 1);
    }
    #[test]
    fn test_3() {
        let mut comp = Computer::new("104,1125899906842624,99", 1);
        let ans = comp.run_to_halt();
        assert_eq!(ans, 1125899906842624);
    }
    #[test]
    fn test_4() {
        let mut comp = Computer::new(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
            1,
        );
        let ans = comp.run_to_halt();
        assert_eq!(ans, 99);
    }
    #[test]
    fn test_5() {
        let mut comp = Computer::new("109,1,3,3,204,2,99", 7);
        let ans = comp.run_to_halt();
        assert_eq!(ans, 7);
    }
    #[test]
    fn test_6() {
        let mut comp = Computer::new("109,1,203,2,204,2,99", -5);
        let ans = comp.run_to_halt();
        assert_eq!(ans, -5);
    }
    #[test]
    fn test_7() {
        let mut comp = Computer::new("1102,34915192,34915192,7,4,7,99,0", -5);
        let ans = comp.run_to_halt();
        assert_eq!(ans.to_string().len(), 16);
    }
    #[test]
    fn test_8() {
        let mut comp = Computer::new("109,1,9,2,204,-6,99", -5);
        let ans = comp.run_to_halt();
        assert_eq!(ans, 204);
    }
    #[test]
    fn test_9() {
        let mut comp = Computer::new("109,-1,204,1,99", -5);
        let ans = comp.run_to_halt();
        assert_eq!(ans, 109);
    }
    #[test]
    fn test_10() {
        let mut comp = Computer::new("109,1,203,11,209,8,204,1,99,10,0,42,0", -5);
        let ans = comp.run_to_halt();
        assert_eq!(ans, -5);
    }
}
