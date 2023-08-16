// use std::{
//     fmt::{Debug, Formatter},
//     str::Chars,
// };

// use itertools::Itertools;
// use to_binary::{BinaryError, BinaryString};

// pub fn solution(part: u8) -> usize {
//     let line = include_str!("../../../problem_inputs_2021/day_16_test.txt");
//     match part {
//         1 => solve01(line),
//         2 => solve02(line),
//         _ => 1,
//     }
// }

// fn solve01(line: &str) -> usize {
//     let line = BinaryString::from_hex(line).unwrap().0;
//     parse(&line);
//     0
// }

// fn solve02(lines: &str) -> usize {
//     0
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum State {
//     Operator,
//     Literal,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum LengthType {
//     BitLength,
//     PacketLength,
// }

// fn parse(given: &str) {
//     let mut contents: Vec<usize> = Vec::new();
//     let mut versions: Vec<usize> = Vec::new();
//     let mut state = State::Operator;
//     let mut bit_iterator = given.chars();
//     let version = bin_to_dec(&given[0..3]);
//     versions.push(version);
//     state = match bin_to_dec(&given[3..6]) {
//         4 => State::Literal,
//         _ => State::Operator,
//     };
//     bit_iterator = adv_iterator(bit_iterator, 6);
//     while bit_iterator.clone().count() > 0 {
//         if state == State::Literal {
//             let literal = parse_literal(&mut bit_iterator);
//             contents.push(literal);
//         } else {
//             let operation_state = match bit_iterator.next().unwrap() {
//                 '0' => LengthType::BitLength,
//                 '1' => LengthType::PacketLength,
//                 _ => panic!("Invalid length type"),
//             };
//         }
//     }
//     dbg!(version, state);
// }

// fn parse_literal(bits: &mut Chars) -> usize {
//     let mut val_str = String::new();
//     loop {
//         let last_group = bits.next().unwrap();
//         val_str.push_str(bits.take(4).collect::<String>().as_str());
//         if last_group == '0' {
//             return bin_to_dec(&val_str);
//         }
//     }
// }

// fn bin_to_dec(bin: &str) -> usize {
//     usize::from_str_radix(bin, 2).unwrap()
// }

// fn adv_iterator(mut iter: Chars, n: usize) -> Chars {
//     for _ in 0..n {
//         iter.next();
//     }
//     iter
// }
