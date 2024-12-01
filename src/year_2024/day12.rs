use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::time::{Duration, Instant};
const LINES: &str = include_str!("../../problem_inputs_2023/day_12_test.txt");

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let inputs = LINES.lines().collect::<Vec<_>>();
    let ans = inputs.par_iter().fold(
        || 0,
        |acc, i| {
            let (springs, true_record) = i.split(' ').collect_tuple().unwrap();
            let springs: SpringRecord = springs
                .chars()
                .filter_map(|c| match c {
                    '#' => Some(Spring::Hash),
                    '.' => Some(Spring::Dot),
                    '?' => Some(Spring::QMark),
                    _ => None,
                })
                .collect_vec();
            let true_record: Vec<usize> = true_record
                .trim()
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect_vec();
            let variations = get_variations(&springs);
            acc + variations
                .par_iter()
                .map(|s| check_if_valid(s, &true_record))
                .filter(|a| *a)
                .count()
        },
    );
    (ans.sum(), now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    // let inputs = LINES.lines().collect::<Vec<_>>();
    // let ans = inputs.par_iter().fold(
    //     || 0,
    //     |acc, i| {
    //         let (springs, true_record) = i.split(' ').collect_tuple().unwrap();
    //         let springs: SpringRecord = springs
    //             .chars()
    //             .filter_map(|c| match c {
    //                 '#' => Some(Spring::Hash),
    //                 '.' => Some(Spring::Dot),
    //                 '?' => Some(Spring::QMark),
    //                 _ => None,
    //             })
    //             .collect_vec();
    //         let springs = extend_for_p2(&springs);
    //         let true_record: Vec<usize> = true_record
    //             .trim()
    //             .split(',')
    //             .map(|n| n.parse().unwrap())
    //             .collect_vec();
    //         let variations = get_variations(&springs);
    //         acc + variations
    //             .par_iter()
    //             .map(|s| check_if_valid(s, &true_record))
    //             .filter(|a| *a)
    //             .count()
    //     },
    // );
    // dbg!(ans.sum::<usize>());
    (0, now.elapsed())
}

fn check_if_valid(springs: &[Spring], true_record: &[usize]) -> bool {
    let test_record = calculate_record(&springs);
    test_record == true_record
}

fn calculate_record(record: &[Spring]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut current = 0;
    for c in record.iter() {
        match c {
            Spring::Hash => {
                current += 1;
            }
            Spring::Dot => {
                if current > 0 {
                    result.push(current);
                    current = 0;
                }
            }
            _ => panic!("Invalid character"),
        }
    }
    if current > 0 {
        result.push(current);
    }
    result
}

fn get_variations(s: &[Spring]) -> Vec<Vec<Spring>> {
    let mut stack: Vec<(Vec<Spring>, usize)> = vec![(Vec::new(), 0)];
    let mut variations = Vec::new();

    while let Some((mut variation, index)) = stack.pop() {
        if index == s.len() {
            variations.push(variation);
        } else {
            match s[index] {
                Spring::QMark => {
                    let mut dot_variation = variation.clone();
                    dot_variation.push(Spring::Dot);
                    stack.push((dot_variation, index + 1));

                    variation.push(Spring::Hash);
                    stack.push((variation, index + 1));
                }
                spring => {
                    variation.push(spring);
                    stack.push((variation, index + 1));
                }
            }
        }
    }

    variations
}

type SpringRecord = Vec<Spring>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Spring {
    Hash,
    Dot,
    QMark,
}

fn extend_for_p2(orig: &[Spring]) -> SpringRecord {
    let mut result = Vec::with_capacity(orig.len() * 5 + 5);
    for _ in 0..2 {
        result.extend(orig);
        result.push(Spring::QMark);
    }
    result.extend(orig);
    result
}
