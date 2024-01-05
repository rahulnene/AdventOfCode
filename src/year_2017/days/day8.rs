use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2017/day_8.txt");
    let mut unique_registers = FxHashSet::default();
    let mut instrs = Vec::new();
    for line in lines.lines() {
        let instr = Instruction::parse(line);
        unique_registers.insert(instr.target_register);
        instrs.push(instr);
    }
    let mut memory = FxHashMap::default();
    for register_name in unique_registers {
        memory.insert(register_name, 0_isize);
    }

    let mut max_val = *memory.values().max().unwrap();
    for instr in instrs {
        process_instr(&mut memory, instr);

        max_val = max_val.max(*memory.values().max().unwrap());
    }
    (*memory.values().max().unwrap() as usize, max_val as usize)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    target_register: usize,
    delta: isize,
    comp_register: usize,
    condition: Comparison,
    value: isize,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let (
            target_register_str,
            delta_direction,
            delta_amount,
            _,
            comp_register,
            condition_str,
            comp_value,
        ) = s.split(' ').collect_tuple().unwrap();
        let target_register = hash_register_name(target_register_str);
        let delta = match delta_direction {
            "inc" => delta_amount.parse::<isize>().unwrap(),
            "dec" => -delta_amount.parse::<isize>().unwrap(),
            _ => panic!("Invalid delta direction"),
        };
        let comp_register = hash_register_name(comp_register);
        let condition = Comparison::parse(condition_str);
        let value = comp_value.parse::<isize>().unwrap();
        Instruction {
            target_register,
            delta,
            comp_register,
            condition,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Comparison {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

impl Comparison {
    fn parse(s: &str) -> Self {
        match s {
            ">" => Comparison::GreaterThan,
            "<" => Comparison::LessThan,
            ">=" => Comparison::GreaterThanOrEqual,
            "<=" => Comparison::LessThanOrEqual,
            "==" => Comparison::Equal,
            "!=" => Comparison::NotEqual,
            _ => panic!("Invalid comparison operator"),
        }
    }
}

fn hash_register_name(name: &str) -> usize {
    let mut hasher = DefaultHasher::default();
    let name = name.chars().map(|f| f as u8).collect_vec();
    name.hash(&mut hasher);
    hasher.finish() as usize
}

fn process_instr(memory: &mut FxHashMap<usize, isize>, instr: Instruction) {
    let comp_register_value = *memory.get(&instr.comp_register).unwrap();
    let comp_value = instr.value;
    let condition = instr.condition;
    let should_modify = match condition {
        Comparison::GreaterThan => comp_register_value > comp_value,
        Comparison::LessThan => comp_register_value < comp_value,
        Comparison::GreaterThanOrEqual => comp_register_value >= comp_value,
        Comparison::LessThanOrEqual => comp_register_value <= comp_value,
        Comparison::Equal => comp_register_value == comp_value,
        Comparison::NotEqual => comp_register_value != comp_value,
    };
    if should_modify {
        let target_register_value = memory.get_mut(&instr.target_register).unwrap();
        *target_register_value += instr.delta;
    }
}
