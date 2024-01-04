use itertools::Itertools;
pub fn solution(part: usize) -> usize {
    let lines = include_str!("../../../problem_inputs_2018/day_16.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let lines = lines.split_once("\n\n\n").unwrap().0;
    let mut cpu = CPU::default();
    cpu.load("3, 2, 2, 1");
    let opcodes = vec![
        Opcode::Add(OpcodeType::Immediate),
        Opcode::Add(OpcodeType::Register),
        Opcode::Mul(OpcodeType::Immediate),
        Opcode::Mul(OpcodeType::Register),
        Opcode::bAND(OpcodeType::Immediate),
        Opcode::bAND(OpcodeType::Register),
        Opcode::bOR(OpcodeType::Immediate),
        Opcode::bOR(OpcodeType::Register),
        Opcode::Assign(OpcodeType::Immediate),
        Opcode::Assign(OpcodeType::Register),
        Opcode::GreaterThan(OpcodeType::Immediate, OpcodeType::Immediate),
        Opcode::GreaterThan(OpcodeType::Immediate, OpcodeType::Register),
        Opcode::GreaterThan(OpcodeType::Register, OpcodeType::Immediate),
        Opcode::GreaterThan(OpcodeType::Register, OpcodeType::Register),
        Opcode::Equality(OpcodeType::Immediate, OpcodeType::Immediate),
        Opcode::Equality(OpcodeType::Immediate, OpcodeType::Register),
        Opcode::Equality(OpcodeType::Register, OpcodeType::Immediate),
        Opcode::Equality(OpcodeType::Register, OpcodeType::Register),
    ];
    // Split the lines into separate tests
    lines
        .split("\n\n")
        .map(|test| {
            // Split the test into lines and load the first line into the cpu
            let test_lines = test.split('\n').collect_vec();
            cpu.load(test_lines[0]);

            // Create a new RegisterBank with the third line of the test
            let target = RegisterBank::new(test_lines[2]);
            let (_, ca, cb, cc) = test_lines[1]
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            // Count the number of opcodes that, when processed with the cpu, result in the target RegisterBank
            opcodes
                .iter()
                .filter(|possible_opcode| {
                    let result = cpu
                        .process(**possible_opcode, (ca, cb, cc))
                        .unwrap_or_default();
                    result == target
                })
                .count()
        })
        .filter(|opcode_count| *opcode_count >= 3)
        .count() // Count the number of tests that have at least 3 valid opcodes
}

fn solve02(lines: &str) -> usize {
    0
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct RegisterBank {
    reg_0: usize,
    reg_1: usize,
    reg_2: usize,
    reg_3: usize,
}

impl RegisterBank {
    fn read_pointer(&self, num: usize) -> usize {
        match num {
            0 => self.reg_0,
            1 => self.reg_1,
            2 => self.reg_2,
            3 => self.reg_3,
            _ => panic!("invalid pointer"),
        }
    }

    fn new(source: &str) -> Self {
        let (a, b, c, d) = source.split(',').collect_tuple().unwrap();
        let a = a.chars().last().unwrap().to_digit(10).unwrap() as usize;
        let b: usize = b.trim().parse().unwrap();
        let c: usize = c.trim().parse().unwrap();
        let d = d.trim().chars().take(1).collect_vec()[0]
            .to_digit(10)
            .unwrap() as usize;
        RegisterBank {
            reg_0: a,
            reg_1: b,
            reg_2: c,
            reg_3: d,
        }
    }

    fn set(&mut self, a: usize, b: usize, c: usize, d: usize) {
        self.reg_0 = a;
        self.reg_1 = b;
        self.reg_2 = c;
        self.reg_3 = d;
    }

    fn set_register(&mut self, register_num: usize, val: usize) {
        match register_num {
            0 => self.reg_0 = val,
            1 => self.reg_1 = val,
            2 => self.reg_2 = val,
            3 => self.reg_3 = val,
            _ => panic!("invalid pointer"),
        };
    }
}

type Command = (usize, usize, usize);

#[derive(Clone, Copy, Debug, Default)]
struct CPU {
    registers: RegisterBank,
}

#[derive(Clone, Copy, Debug)]
enum Register {
    A(usize),
    B(usize),
    C(usize),
    D(usize),
}

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Add(OpcodeType),
    Mul(OpcodeType),
    bAND(OpcodeType),
    bOR(OpcodeType),
    Assign(OpcodeType),
    GreaterThan(OpcodeType, OpcodeType),
    Equality(OpcodeType, OpcodeType),
}

#[derive(Clone, Copy, Debug)]
enum OpcodeType {
    Immediate,
    Register,
}

impl CPU {
    fn load(&mut self, registers: &str) {
        let (a, b, c, d) = registers.split(',').collect_tuple().unwrap();
        let a = a.chars().last().unwrap().to_digit(10).unwrap() as usize;
        let b: usize = b.trim().parse().unwrap();
        let c: usize = c.trim().parse().unwrap();
        let d = d.trim().chars().take(1).collect_vec()[0]
            .to_digit(10)
            .unwrap() as usize;
        self.registers.set(a, b, c, d)
    }

    fn process(&self, code: Opcode, command: Command) -> Option<RegisterBank> {
        let mut registers = self.registers;
        match code {
            Opcode::Add(register_type) => match register_type {
                OpcodeType::Immediate => match command {
                    (a, b, c) => registers.set_register(c, registers.read_pointer(a) + b),
                },
                OpcodeType::Register => match command {
                    (a, b, c) => registers
                        .set_register(c, registers.read_pointer(a) + registers.read_pointer(b)),
                },
            },
            Opcode::Mul(register_type) => match register_type {
                OpcodeType::Immediate => match command {
                    (a, b, c) => registers.set_register(c, registers.read_pointer(a) * b),
                },
                OpcodeType::Register => match command {
                    (a, b, c) => registers
                        .set_register(c, registers.read_pointer(a) * registers.read_pointer(b)),
                },
            },
            Opcode::bAND(register_type) => match register_type {
                OpcodeType::Immediate => match command {
                    (a, b, c) => registers.set_register(c, registers.read_pointer(a) & b),
                },
                OpcodeType::Register => match command {
                    (a, b, c) => registers
                        .set_register(c, registers.read_pointer(a) & registers.read_pointer(b)),
                },
            },
            Opcode::bOR(register_type) => match register_type {
                OpcodeType::Immediate => match command {
                    (a, b, c) => registers.set_register(c, registers.read_pointer(a) | b),
                },
                OpcodeType::Register => match command {
                    (a, b, c) => registers
                        .set_register(c, registers.read_pointer(a) | registers.read_pointer(b)),
                },
            },
            Opcode::Assign(register_type) => match register_type {
                OpcodeType::Immediate => match command {
                    (a, _, c) => registers.set_register(c, a),
                },
                OpcodeType::Register => match command {
                    (a, _, c) => registers.set_register(c, registers.read_pointer(a)),
                },
            },
            Opcode::GreaterThan(register_type_a, register_type_b) => {
                match (register_type_a, register_type_b) {
                    (OpcodeType::Immediate, OpcodeType::Register) => match command {
                        (a, b, c) => registers
                            .set_register(c, if a > registers.read_pointer(b) { 1 } else { 0 }),
                    },
                    (OpcodeType::Register, OpcodeType::Immediate) => match command {
                        (a, b, c) => registers
                            .set_register(c, if registers.read_pointer(a) > b { 1 } else { 0 }),
                    },
                    (OpcodeType::Register, OpcodeType::Register) => match command {
                        (a, b, c) => registers.set_register(
                            c,
                            if registers.read_pointer(a) > registers.read_pointer(b) {
                                1
                            } else {
                                0
                            },
                        ),
                    },
                    _ => return None,
                }
            }
            Opcode::Equality(register_type_a, register_type_b) => {
                match (register_type_a, register_type_b) {
                    (OpcodeType::Immediate, OpcodeType::Register) => match command {
                        (a, b, c) => registers
                            .set_register(c, if a == registers.read_pointer(b) { 1 } else { 0 }),
                    },
                    (OpcodeType::Register, OpcodeType::Immediate) => match command {
                        (a, b, c) => registers
                            .set_register(c, if registers.read_pointer(a) == b { 1 } else { 0 }),
                    },
                    (OpcodeType::Register, OpcodeType::Register) => match command {
                        (a, b, c) => registers.set_register(
                            c,
                            if registers.read_pointer(a) == registers.read_pointer(b) {
                                1
                            } else {
                                0
                            },
                        ),
                    },
                    _ => return None,
                }
            }
        }
        Some(registers)
    }
}
