use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2020/day_14.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve(set_mem_p1), solve(set_mem_p2))
}

fn solve(set_mem: impl Fn(&mut Memory, usize, usize)) -> (usize, Duration) {
    let now = Instant::now();
    let mut memory = Memory::new();
    for line in LINES.lines() {
        let mut parts = line.split(" = ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        if left == "mask" {
            memory.set_mask(right);
        } else {
            let addr = left[4..left.len() - 1].parse::<usize>().unwrap();
            let value = right.parse::<usize>().unwrap();
            set_mem(&mut memory, addr, value);
        }
    }
    let ans = memory.mem.values().map(|v| bits_to_int(*v)).sum();
    (ans, now.elapsed())
}

#[derive(Debug)]
struct Memory {
    mask: Mask,
    mem: FxHashMap<ThirtySixBits, ThirtySixBits>,
}

impl Memory {
    fn new() -> Self {
        Self {
            mask: [MaskValue::X; 36],
            mem: FxHashMap::default(),
        }
    }

    fn set_mask(&mut self, mask_str: &str) {
        self.mask = str_to_mask(mask_str);
    }
}

fn set_mem_p1(mem: &mut Memory, addr: usize, value: usize) {
    let address = int_to_bits(addr);
    let value = apply_mask_p1(&mem.mask, &int_to_bits(value));
    mem.mem.insert(address, value);
}

fn set_mem_p2(mem: &mut Memory, addr: usize, value: usize) {
    let value = int_to_bits(value);
    let addr = int_to_bits(addr);
    let possible_addresses = apply_mask_p2(&mem.mask, &addr);
    for address in possible_addresses {
        mem.mem.insert(address, value);
    }
}
fn str_to_mask(s: &str) -> Mask {
    let mut mask = [MaskValue::X; 36];
    for (i, c) in s.chars().enumerate() {
        match c {
            '0' => mask[i] = MaskValue::Zero,
            '1' => mask[i] = MaskValue::One,
            'X' => mask[i] = MaskValue::X,
            _ => (),
        }
    }
    mask
}

fn int_to_bits(n: usize) -> ThirtySixBits {
    let mut mem_value = [false; 36];
    for (i, c) in format!("{:036b}", n).chars().enumerate() {
        match c {
            '0' => mem_value[i] = false,
            '1' => mem_value[i] = true,
            _ => (),
        }
    }
    mem_value
}

fn bits_to_int(mem_value: ThirtySixBits) -> usize {
    let mut n = 0;
    for (i, &b) in mem_value.iter().enumerate() {
        if b {
            n += 2_usize.pow(35 - i as u32);
        }
    }
    n
}

fn apply_mask_p1(mask: &Mask, mem_value: &ThirtySixBits) -> ThirtySixBits {
    let mut new_mem_value = [false; 36];
    for (i, &mask_value) in mask.iter().enumerate() {
        new_mem_value[i] = match mask_value {
            MaskValue::Zero => false,
            MaskValue::One => true,
            MaskValue::X => mem_value[i],
        };
    }
    new_mem_value
}

fn apply_mask_p2(mask: &Mask, mem_value: &ThirtySixBits) -> Vec<ThirtySixBits> {
    let mut new_mem_value = [MaskValue::X; 36];
    for (i, &mask_value) in mask.iter().enumerate() {
        new_mem_value[i] = match mask_value {
            MaskValue::Zero => match mem_value[i] {
                true => MaskValue::One,
                false => MaskValue::Zero,
            },
            MaskValue::One => MaskValue::One,
            MaskValue::X => MaskValue::X,
        };
    }
    generate_variations(new_mem_value.to_vec())
}

type ThirtySixBits = [bool; 36];
type Mask = [MaskValue; 36];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MaskValue {
    Zero,
    One,
    X,
}
fn generate_variations(input: Vec<MaskValue>) -> Vec<ThirtySixBits> {
    let mut variations: Vec<Vec<MaskValue>> = vec![vec![]]; // Initialize with an empty vector

    for &mask in &input {
        let mut new_variations: Vec<Vec<MaskValue>> = Vec::new();

        for variation in &variations {
            match mask {
                MaskValue::One => {
                    let mut variation_clone = variation.clone();
                    variation_clone.push(MaskValue::One);
                    new_variations.push(variation_clone);
                }
                MaskValue::Zero => {
                    let mut variation_clone = variation.clone();
                    variation_clone.push(MaskValue::Zero);
                    new_variations.push(variation_clone);
                }
                MaskValue::X => {
                    let mut variation_clone_one = variation.clone();
                    variation_clone_one.push(MaskValue::One);
                    new_variations.push(variation_clone_one);

                    let mut variation_clone_zero = variation.clone();
                    variation_clone_zero.push(MaskValue::Zero);
                    new_variations.push(variation_clone_zero);
                }
            }
        }

        variations = new_variations;
    }

    variations
        .iter()
        .map(|v| mask_vals_to_thirty_six_bits(v))
        .collect()
}

fn mask_vals_to_thirty_six_bits(input: &[MaskValue]) -> ThirtySixBits {
    let mut thirty_six_bits = [false; 36];
    for (i, &mask_val) in input.iter().enumerate() {
        thirty_six_bits[i] = match mask_val {
            MaskValue::One => true,
            MaskValue::Zero => false,
            MaskValue::X => unreachable!("X should not be in the input"),
        };
    }
    thirty_six_bits
}
