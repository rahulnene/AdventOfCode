use std::time::{Duration, Instant};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
pub fn solution() -> ((u16, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2015/day_7.txt");
    (solve01(&lines), solve02(&lines))
}

fn solve01(lines: &str) -> (u16, Duration) {
    let now = Instant::now();
    let mut circuit = Circuit::new();
    let instrs = lines.lines().map(|l| Instruction::parse(l)).collect_vec();
    for instr in instrs {
        circuit.process_instr(&instr);
    }
    (*circuit.wires.get("a").unwrap(), now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct String {
//     name: String,
//     value: u16,
// }

// impl String {
//     fn new(name: &str, value: u16) -> Self {
//         Self {
//             name: name.to_string(),
//             value,
//         }
//     }

//     fn parse(line: &str) -> Self {
//         let mut parts = line.split(" -> ");
//         let expr = parts.next().unwrap();
//         let name = parts.next().unwrap();
//         let value = match expr {
//             "1" => 1,
//             "0" => 0,
//             _ => 0,
//         };
//         Self {
//             name: name.to_string(),
//             value,
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    GetValueFromWire(String, String),
    GetValueFromSignal(u16, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, u16, String),
    RShift(String, u16, String),
    Not(String, String),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        if s.contains("AND") {
            let parts = s.split(" ").collect_vec();
            let w1 = parts[0];
            let w2 = parts[2];
            let dest = parts[4];
            return Self::And(w1.to_string(), w2.to_string(), dest.to_string());
        } else if s.contains("OR") {
            let parts = s.split(" ").collect_vec();
            let w1 = parts[0];
            let w2 = parts[2];
            let dest = parts[4];
            return Self::Or(w1.to_string(), w2.to_string(), dest.to_string());
        } else if s.contains("LSHIFT") {
            let parts = s.split(" ").collect_vec();
            let w1 = parts[0];
            let w2 = parts[2];
            let dest = parts[4];
            return Self::LShift(w1.to_string(), w2.parse().unwrap(), dest.to_string());
        } else if s.contains("RSHIFT") {
            let parts = s.split(" ").collect_vec();
            let w1 = parts[0];
            let w2 = parts[2];
            let dest = parts[4];
            return Self::RShift(w1.to_string(), w2.parse().unwrap(), dest.to_string());
        } else if s.contains("NOT") {
            let parts = s.split(" ").collect_vec();
            let w1 = parts[1];
            let dest = parts[3];
            return Self::Not(w1.to_string(), dest.to_string());
        } else {
            let parts = s.split(" ").collect_vec();
            let w1 = parts[0];
            let dest = parts[2];
            if w1.parse::<u16>().is_ok() {
                return Self::GetValueFromSignal(w1.parse().unwrap(), dest.to_string());
            } else {
                return Self::GetValueFromWire(w1.to_string(), dest.to_string());
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Circuit {
    wires: FxHashMap<String, u16>,
    instructions: Vec<Instruction>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            wires: FxHashMap::default(),
            instructions: Vec::new(),
        }
    }

    fn process_instr(&mut self, instr: &Instruction) {
        match instr {
            Instruction::GetValueFromWire(source, dest) => {
                let source_val = self.wires.get(source).or(Some(&0)).unwrap();
                self.wires.insert(dest.to_string(), *source_val);
            }
            Instruction::GetValueFromSignal(val, dest) => {
                self.wires.insert(dest.to_string(), *val);
            }
            Instruction::And(w1, w2, dest) => {
                let w1_val = self.wires.get(w1).or(Some(&0)).unwrap();
                let w2_val = self.wires.get(w2).or(Some(&0)).unwrap();
                self.wires.insert(dest.to_string(), w1_val & w2_val);
            }
            Instruction::Or(w1, w2, dest) => {
                let w1_val = self.wires.get(w1).or(Some(&0)).unwrap();
                let w2_val = self.wires.get(w2).or(Some(&0)).unwrap();
                self.wires.insert(dest.to_string(), w1_val | w2_val);
            }
            Instruction::LShift(source, val, dest) => {
                let source_val = self.wires.get(source).or(Some(&0)).unwrap();
                self.wires.insert(dest.to_string(), source_val << val);
            }
            Instruction::RShift(source, val, dest) => {
                let source_val = self.wires.get(source).or(Some(&0)).unwrap();
                self.wires.insert(dest.to_string(), source_val >> val);
            }
            Instruction::Not(source, dest) => {
                let source_val = self.wires.get(source).or(Some(&0)).unwrap();
                self.wires.insert(dest.to_string(), !source_val);
            }
        }
    }
}
