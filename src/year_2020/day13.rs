use std::time::{Duration, Instant};

use lazy_static::lazy_static;

const LINES: &str = include_str!("../../problem_inputs_2020/day_13.txt");

lazy_static! {
    static ref BUSES: Vec<Option<usize>> = LINES
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().ok())
        .collect::<Vec<Option<usize>>>();
}

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(&BUSES), solve02(&BUSES))
}

fn solve01(buses: &[Option<usize>]) -> (usize, Duration) {
    let now = std::time::Instant::now();
    let departure = LINES.lines().nth(0).unwrap().parse::<usize>().unwrap();
    let (min_wait_time, bus) = buses
        .iter()
        .filter_map(|&b| b)
        .map(|bus| (wait_time(departure, bus), bus))
        .min_by_key(|&(wait_time, _)| wait_time)
        .unwrap();

    let ans = min_wait_time * bus;
    (ans, now.elapsed())
}

fn solve02(buses: &[Option<usize>]) -> (usize, Duration) {
    let now = Instant::now();
    let offset_and_moduli = buses
        .iter()
        .enumerate()
        .filter_map(|(i, bus)| bus.map(|val| Modulus::new(val * i - i, val)))
        .collect::<Vec<Modulus>>();
    (chinese_remainder_theorem(offset_and_moduli), now.elapsed())
}

fn wait_time(departure: usize, bus: usize) -> usize {
    (bus - (departure % bus)) % bus
}

#[derive(Debug, Clone, Copy)]
struct Modulus {
    remainder: usize,
    modulus: usize,
}

impl Modulus {
    fn new(rem: usize, modulo: usize) -> Self {
        Self {
            remainder: rem,
            modulus: modulo,
        }
    }
}

fn chinese_remainder_theorem(moduli: Vec<Modulus>) -> usize {
    let product = moduli.iter().map(|m| m.modulus).product::<usize>();
    let mut sum = 0;
    for m in moduli {
        let p = product / m.modulus;
        sum += (m.remainder) % product * (mod_inv(p, m.modulus) * p) % product;
    }
    sum % product
}

fn mod_inv(a: usize, m: usize) -> usize {
    let mut a = a as i64;
    let mut m = m as i64;
    let m0 = m;
    let mut x0 = 0;
    let mut x1 = 1;
    if m == 1 {
        return 0;
    }
    while a > 1 {
        let q = a / m;
        let mut t = m;
        m = a % m;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 {
        x1 += m0;
    }
    x1 as usize
}
