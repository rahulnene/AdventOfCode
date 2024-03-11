use divisors::{self, get_divisors};
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::time::{Duration, Instant};
const INPUT: usize = 34000000;

pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    (solve(calculate_presents), solve(calculate_presents_p2))
}
fn solve(sum_fun: impl Fn(usize) -> usize + Sync) -> (usize, Duration) {
    let mut lower_bound = 1;
    let mut i = 2;
    while lower_bound <= INPUT / 11 {
        lower_bound *= i;
        i += 1;
    }
    lower_bound /= i - 1;
    let now = Instant::now();
    let ans = (lower_bound..900000)
        .into_par_iter()
        .step_by(1 * 2 * 3 * 4 * 5 * 6 * 7)
        .by_exponential_blocks()
        .find_map_first(|house| {
            let pres = sum_fun(house);
            if pres >= INPUT {
                Some(house)
            } else {
                None
            }
        })
        .unwrap();
    (ans, now.elapsed())
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
