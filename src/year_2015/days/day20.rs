use divisors::get_divisors;
use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let target = 34000000;
    (solve01(target), solve02(target))
}

fn solve01(target: usize) -> (usize, Duration) {
    let now = Instant::now();
    for house in 700000.. {
        let presents = sum_of_divisorsp1(house);
        if presents >= target {
            dbg!(presents);
            return (house, now.elapsed());
        }
    }
    (0, now.elapsed())
}

fn solve02(target: usize) -> (usize, Duration) {
    let now = Instant::now();
    for house in 700000.. {
        let presents = sum_of_divisorsp2(house);
        if presents >= target {
            dbg!(presents);
            return (house, now.elapsed());
        }
    }
    (0, now.elapsed())
}
fn sum_of_divisorsp1(n: usize) -> usize {
    10 * (1 + get_divisors(n).iter().sum::<usize>() + n)
}

fn sum_of_divisorsp2(n: usize) -> usize {
    let divisor_count = get_divisors(n).len();
    11 * (1
        + get_divisors(n).iter().take(49).sum::<usize>()
        + if divisor_count < 49 { n } else { 0 })
}
