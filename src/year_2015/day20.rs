use divisors::{self, get_divisors};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::time::{Duration, Instant};
const INPUT: usize = 34000000;

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (usize, Duration) {
    let now = Instant::now();
    let mut ans: Vec<usize> = (100000..1000000)
        .par_bridge()
        .map(|house| (house, calculate_presents(house)))
        .filter(|(_, pres)| *pres >= INPUT)
        .map(|c| c.0)
        .collect();
    ans.sort_unstable();
    (ans[0], now.elapsed())
}

fn solve02() -> (usize, Duration) {
    let now = Instant::now();
    let mut ans: Vec<usize> = (100000..1000000)
        .par_bridge()
        .map(|house| (house, calculate_presents_p2(house)))
        .filter(|(_, pres)| *pres >= INPUT)
        .map(|c| c.0)
        .collect();
    ans.sort_unstable();
    (ans[0], now.elapsed())
}

fn calculate_presents(house_num: usize) -> usize {
    (get_divisors(house_num).iter().map(|&x| x).sum::<usize>() + (house_num + 1)) * 10
}
fn calculate_presents_p2(house_num: usize) -> usize {
    let mut divisors = get_divisors(house_num);
    divisors.push(house_num);
    let sum = divisors
        .iter()
        .filter(|factor| house_num / **factor <= 50)
        .unique()
        .sum::<usize>();
    sum * 11
}
