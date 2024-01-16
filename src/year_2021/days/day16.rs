use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let line = include_str!("../../../problem_inputs_2021/day_16_test.txt");
    (solve01(line), solve02(line))
}

fn solve01(line: &str) -> (usize, Duration) {
    let now = Instant::now();
    let bin_input = hex_to_binary(line);
    // dbg!(&bin_input);
    let packet = Packet::parse(&bin_input);
    dbg!(packet);
    (0, now.elapsed())
}

fn solve02(line: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn hex_to_binary(s: &str) -> Vec<bool> {
    let mut v = Vec::with_capacity(s.len() * 4);
    for char in s.chars() {
        let binary = hex_to_binary_char(&char);
        v.extend_from_slice(&binary);
    }
    v
}

fn hex_to_binary_char(c: &char) -> [bool; 4] {
    match c {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'A' => [true, false, true, false],
        'B' => [true, false, true, true],
        'C' => [true, true, false, false],
        'D' => [true, true, false, true],
        'E' => [true, true, true, false],
        'F' => [true, true, true, true],
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketContents,
}

#[derive(Debug, Clone)]
enum PacketContents {
    Literal(usize),
    Operator(OperatorPacket),
}

impl Packet {
    fn parse(packet: &[bool]) -> Self {
        let version = Self::parse_version(packet);
        let type_id = Self::parse_type_id(packet);
        let data = Self::parse_data(packet, type_id);
        Self {
            version,
            type_id,
            data,
        }
    }

    fn parse_version(packet: &[bool]) -> u8 {
        packet[0..3].iter().fold(0, |acc, &b| (acc << 1) + b as u8)
    }
    fn parse_type_id(packet: &[bool]) -> u8 {
        packet[3..6].iter().fold(0, |acc, &b| (acc << 1) + b as u8)
    }

    fn parse_data(packet: &[bool], type_id: u8) -> PacketContents {
        match type_id {
            4 => PacketContents::Literal(Self::parse_literal(packet)),
            _ => PacketContents::Operator(Self::parse_operator(packet)),
        }
    }

    fn parse_literal(packet: &[bool]) -> usize {
        let packet_iter = packet.iter().skip(6);
        loop {
            let iter = packet.chunks_exact(5);
            let remainder = iter.remainder();
            let chunks = iter
                .take_while(|l| l.first().unwrap() == &true)
                .collect_vec();
        }
        0
    }
    fn parse_operator(packet: &[bool]) -> OperatorPacket {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone)]
struct LiteralPacket {
    data: usize,
}

#[derive(Debug, Clone)]
struct OperatorPacket {
    metadata: OperatorMetaData,
    packets: Vec<Packet>,
}

#[derive(Debug, Clone, Copy)]
enum OperatorMetaData {
    Length(usize),
    SubPacketCount(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_test_1() {
        let s = "D2F";
        let v = hex_to_binary(s);
        assert_eq!(
            v,
            vec![true, true, false, true, false, false, true, false, true, true, true, true]
        )
    }
}
