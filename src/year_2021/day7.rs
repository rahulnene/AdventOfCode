use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_7.txt");
    let mut given = lines
        .split(',')
        .map(|f| f.parse::<usize>().unwrap())
        .collect_vec();
    given.sort_unstable();
    match part {
        1 => solve01(&given),
        2 => solve02(&given),
        _ => 1,
    }
}

fn solve01(given: &[usize]) -> usize {
    let optimal = median(given);
    given.iter().map(|f| optimal.abs_diff(*f)).sum()
}

fn solve02(given: &[usize]) -> usize {
    let optimal = average(given).round() as usize;
    let fuel: usize = p2_fuel(given, optimal);
    let down_one: usize = p2_fuel(given, optimal - 1);
    let up_one: usize = p2_fuel(given, optimal + 1);
    fuel.min(down_one.min(up_one))
}

fn median(v: &[usize]) -> usize {
    let len = v.len();
    let ans = if len % 2 == 0 {
        (v[len / 2 - 1] + v[len / 2]) as f64 / 2.0
    } else {
        v[len / 2] as f64
    };
    ans.round() as usize
}

fn average(v: &[usize]) -> f64 {
    f64::round(v.iter().sum::<usize>() as f64 / v.len() as f64) - 1.0
}

fn p2_fuel(locs: &[usize], pos: usize) -> usize {
    locs.iter()
        .map(|f| {
            let fuel = (pos).abs_diff(*f);
            fuel * (fuel + 1) / 2
        })
        .sum()
}
