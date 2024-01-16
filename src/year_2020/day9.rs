use fxhash::FxHashSet;
use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_9.txt");
    let preamble_length = 25;
    match part {
        1 => solve01(lines, preamble_length),
        2 => solve02(lines, preamble_length),
        _ => 1,
    }
}

fn solve01(lines: &str, preamble_length: u8) -> usize {
    let mut preamble = lines
        .lines()
        .take(preamble_length as usize)
        .map(|f| f.parse::<usize>().unwrap())
        .collect_vec();
    let mut sums = calculateSums(&preamble);
    let mut lines = lines.lines().skip(preamble_length as usize);
    while let Some(n) = lines.next() {
        let n = n.parse::<usize>().unwrap();
        if !sums.contains(&n) {
            // dbg!(preamble, sums);
            return n;
        }
        preamble.push(n);
        preamble.remove(0);
        sums = calculateSums(&preamble);
    }
    0
}

fn solve02(lines: &str, preamble_length: u8) -> usize {
    let target = solve01(lines, preamble_length);
    let numbers = lines
        .lines()
        .map(|f| f.parse::<usize>().unwrap())
        .collect_vec();
    for i in 2..numbers.len() {
        for j in 0..numbers.len() - i {
            let slice = &numbers[j..j + i];
            let sum: usize = slice.iter().sum();
            if sum == target {
                return slice.iter().min().unwrap() + slice.iter().max().unwrap();
            }
        }
    }
    0
}

fn calculateSums(preamble: &[usize]) -> FxHashSet<usize> {
    let mut sums = FxHashSet::default();
    preamble
        .iter()
        .cartesian_product(preamble.iter())
        .for_each(|(a, b)| {
            if *a != *b {
                sums.insert(a + b);
            }
        });
    sums
}
