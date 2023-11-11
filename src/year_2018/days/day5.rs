use fxhash::FxHashMap;

pub fn solution(part: u8) -> isize {
    let lines = include_str!("../../../problem_inputs_2019/day_5.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> isize {
    let mut cpu = CPU::new(1);
    for (i, val) in lines.split(",").enumerate() {
        cpu.raw_set(i as isize, val.parse::<isize>().unwrap());
    }
    while !cpu.halt {
        cpu.process();
    }
    dbg!(&cpu);
    cpu.output
}

fn solve02(lines: &str) -> isize {
    0
}
#[derive(Debug, Clone, Default)]
struct CPU {
    input: isize,
    output: isize,
    memory: FxHashMap<isize, isize>,
    instruction_pointer: isize,
    halt: bool,
}

impl CPU {
    fn new(input: isize) -> CPU {
        let mut a = CPU::default();
        a.input = input;
        a
    }
    fn raw_set(&mut self, location: isize, value: isize) {
        self.memory.insert(location, value);
    }
    fn set_at(&mut self, location: isize, value: isize) {
        let location = self.read(location);
        self.memory.insert(location, value);
    }
    fn get(&self, location: isize, mode: isize) -> isize {
        match mode {
            0 => self.read(location),
            1 => location,
            _ => panic!("Invalid mode"),
        }
    }

    fn read(&self, location: isize) -> isize {
        *self.memory.get(&location).unwrap_or(&0)
    }

    fn process(&mut self) {
        let instr = Instruction::from_memory_value(
            *self.memory.get(&self.instruction_pointer).unwrap_or(&0),
        );
        dbg!(instr);
        match instr.opcode {
            1 => {
                let a = self.get(self.read(self.instruction_pointer + 1), instr.mode_1);
                let b = self.get(self.read(self.instruction_pointer + 2), instr.mode_2);
                dbg!(a, b, a + b);
                self.set_at(self.read(self.instruction_pointer + 3), a + b);
                self.instruction_pointer += 4;
            }
            2 => {
                let a = self.get(self.read(self.instruction_pointer + 1), instr.mode_1);
                let b = self.get(self.read(self.instruction_pointer + 2), instr.mode_2);
                dbg!(a, b, a * b);
                self.set_at(self.read(self.instruction_pointer + 3), a * b);
                self.instruction_pointer += 4;
            }
            3 => {
                let location = self.get(self.read(self.instruction_pointer + 1), instr.mode_1);
                self.set_at(location, self.input);
                self.instruction_pointer += 2;
                dbg!(location);
            }
            4 => {
                self.output = self.get(self.instruction_pointer + 1, instr.mode_1);
                self.instruction_pointer += 2;
                dbg!("output", self.output);
            }
            99 => {
                self.halt = true;
            }
            _ => {
                // dbg! {&self};
                panic!("{}", format!("Invalid opcode: {}", instr.opcode))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: isize,
    mode_1: isize,
    mode_2: isize,
    mode_3: isize,
}

impl Instruction {
    fn from_memory_value(val: isize) -> Instruction {
        Instruction {
            opcode: val % 100,
            mode_1: (val / 100) % 10,
            mode_2: (val / 1000) % 10,
            mode_3: (val / 10000) % 10,
        }
    }
}
