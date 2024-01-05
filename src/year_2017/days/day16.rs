use std::{collections::VecDeque, time::Instant};

use fxhash::FxHashMap;
use itertools::Itertools;
pub fn solution() -> (usize, usize) {
    let line = include_str!("../../../problem_inputs_2017/day_16.txt");
    let dance = parse_dance(line);
    (solve01(&dance), solve02(&dance))
}

fn solve01(dance: &[DanceType]) -> usize {
    let mut prom = Promenade::new();
    let now = Instant::now();
    prom.perform_dance(dance);
    println!("{:?}", now.elapsed());
    prom.print();
    0
}

fn solve02(dance: &[DanceType]) -> usize {
    let mut prom = Promenade::new();
    for _ in 0..10_000 {
        prom.perform_dance(dance);
    }
    prom.print();
    0
}

#[derive(Debug, Clone)]
struct Promenade {
    programs: VecDeque<char>,
}

impl Promenade {
    fn new() -> Self {
        let mut programs = VecDeque::with_capacity(16);
        for i in 0..16_u8 {
            programs.push_back((i + 97) as char);
        }
        Self { programs }
    }

    fn get_lineup_as_string(&self) -> String {
        self.programs.iter().collect()
    }

    fn spin(&mut self, x: usize) {
        self.programs.rotate_right(x);
    }

    fn exchange(&mut self, a: usize, b: usize) {
        self.programs.swap(a, b);
    }

    fn exchange_immediate(&mut self, a: char, b: char) {
        let a = self.programs.iter().position(|&x| x == a).unwrap();
        let b = self.programs.iter().position(|&x| x == b).unwrap();
        self.exchange(a, b);
    }

    fn print(&self) {
        self.programs.iter().for_each(|f| print!("{}", f));
        println!();
    }

    fn perform_dance(&mut self, dance: &[DanceType]) {
        for step in dance {
            match step {
                DanceType::Spin(x) => self.spin(*x),
                DanceType::Exchange(a, b) => self.exchange(*a, *b),
                DanceType::Partner(a, b) => self.exchange_immediate(*a, *b),
            }
        }
    }
}

fn get_mapping_between_dances(line_up1: &str, line_up2: &str) -> FxHashMap<usize, usize> {
    let mut mapping = FxHashMap::default();
    let a = line_up1
        .chars()
        .enumerate()
        .map(|f| {
            (
                f.0,
                line_up2.chars().find_position(|g| *g == f.1).unwrap().0,
            )
        })
        .collect_vec();
    for (i, j) in a {
        mapping.insert(i, j);
    }
    mapping
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spin() {
        let mut prom = Promenade::new();
        prom.spin(1);
        assert_eq!(prom.programs[0], 'p');
    }

    #[test]
    fn test_exchange() {
        let mut prom = Promenade::new();
        prom.exchange(0, 1);
        assert_eq!(prom.programs[0], 'b');
        assert_eq!(prom.programs[1], 'a');
    }

    #[test]
    fn test_exchange_immediate() {
        let mut prom = Promenade::new();
        prom.exchange_immediate('a', 'b');
        assert_eq!(prom.programs[0], 'b');
        assert_eq!(prom.programs[1], 'a');
    }
}

#[derive(Debug, Clone, Copy)]
enum DanceType {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_dance(line: &str) -> Dance {
    line.split(',')
        .map(|f| match f.chars().nth(0).unwrap() {
            's' => DanceType::Spin(f[1..].parse::<usize>().unwrap()),
            'x' => {
                let mut iter = f[1..].split('/');
                let a = iter.next().unwrap().parse::<usize>().unwrap();
                let b = iter.next().unwrap().parse::<usize>().unwrap();
                DanceType::Exchange(a, b)
            }
            'p' => {
                let mut iter = f[1..].split('/');
                let a = iter.next().unwrap().chars().nth(0).unwrap();
                let b = iter.next().unwrap().chars().nth(0).unwrap();
                DanceType::Partner(a, b)
            }
            _ => panic!("Invalid dance move"),
        })
        .collect()
}

type Dance = Vec<DanceType>;
