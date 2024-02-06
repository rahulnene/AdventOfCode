use std::time::{Duration, Instant};

const LINES: &str = include_str!("../../problem_inputs_2021/day_16.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let packet = decode(LINES.lines().next().unwrap());
    (solve01(&packet), solve02(&packet))
}

fn solve01(packet: &Packet) -> (usize, Duration) {
    let now = Instant::now();
    (version_sum(packet), now.elapsed())
}

fn solve02(packet: &Packet) -> (usize, Duration) {
    let now = Instant::now();
    (simplify_operator(packet), now.elapsed())
}

#[derive(Clone, Debug)]
pub enum Packet {
    Operator(Operator),
    Literal(Literal),
}

#[derive(Clone, Debug)]
pub struct Literal {
    pub header: Header,
    pub value: usize,
    size: usize,
}

#[derive(Clone, Debug)]
pub struct Operator {
    pub header: Header,
    pub children: Vec<Packet>,
    size: usize,
}

type Header = (usize, usize);

pub fn decode(message: &str) -> Packet {
    decode_packet(&str_to_bits(message))
}

fn decode_packet(bits: &[u8]) -> Packet {
    if get_packet_type(&bits[3..6]) == 4 {
        decode_literal(bits)
    } else {
        decode_operator(bits)
    }
}

fn decode_literal(bits: &[u8]) -> Packet {
    let rest = &bits[6..];
    let mut index = 0;
    let mut num = Vec::new();

    while index <= rest.len() - 5 {
        let next = index + 5;
        let chunk = &rest[index..next];
        let signal = chunk[0];

        num.extend(&chunk[1..5]);
        index = next;

        if signal == 0 {
            break;
        }
    }

    Packet::Literal(Literal {
        header: decode_header(bits),
        value: get_packet_type(&num),
        size: 6 + index,
    })
}

fn decode_operator(bits: &[u8]) -> Packet {
    let mode = bits[6];
    let content_start = if mode == 0 { 22 } else { 18 };
    let len = get_packet_type(&bits[7..content_start]) as usize;

    let mut children = Vec::new();
    let mut index = 0;

    while (mode == 0 && index < len) || (mode == 1 && children.len() < len) {
        let packet = decode_packet(&bits[(content_start + index)..]);

        match &packet {
            Packet::Literal(data) => index += data.size,
            Packet::Operator(data) => index += data.size,
        }

        children.push(packet);
    }

    Packet::Operator(Operator {
        header: decode_header(bits),
        children,
        size: content_start + index,
    })
}

fn decode_header(bits: &[u8]) -> Header {
    (get_packet_type(&bits[0..3]), get_packet_type(&bits[3..6]))
}

fn str_to_bits(message: &str) -> Vec<u8> {
    message
        .chars()
        .flat_map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            c => panic!("unexpected token in message: {}", c),
        })
        .collect()
}

fn get_packet_type(bits: &[u8]) -> usize {
    bits.iter().fold(0, |acc, &b| acc * 2 + (b as usize))
}

pub fn simplify_operator(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(x) => x.value,
        Packet::Operator(data) => {
            let values: Vec<usize> = data
                .children
                .iter()
                .map(|child| match child {
                    Packet::Literal(child_data) => child_data.value as usize,
                    Packet::Operator(child_data) => {
                        simplify_operator(&Packet::Operator(child_data.clone()))
                    }
                })
                .collect();

            match data.header.1 {
                0 => values.iter().sum(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => (values[0] > values[1]) as usize,
                6 => (values[0] < values[1]) as usize,
                7 => (values[0] == values[1]) as usize,
                c => panic!("unknown type_id {}", c),
            }
        }
    }
}

fn version_sum(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(x) => x.header.0,
        Packet::Operator(data) => {
            data.children
                .iter()
                .fold(data.header.0, |acc, curr| match curr {
                    Packet::Literal(child_data) => acc + child_data.header.0,
                    Packet::Operator(child_data) => {
                        acc + version_sum(&Packet::Operator(child_data.clone()))
                    }
                })
        }
    }
}
