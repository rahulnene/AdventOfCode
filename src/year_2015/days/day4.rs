use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let seed = "yzbqklnj";
    (solve(seed, 5), solve(seed, 6))
}

fn solve(seed: &str, num_zeroes: usize) -> (usize, Duration) {
    let zeroes = "0".repeat(num_zeroes);
    let now = Instant::now();
    for i in 0.. {
        let input = format!("{}{}", seed, i);
        let digest = md5::compute(input);
        let hex = format!("{:x}", digest);
        if hex.starts_with(zeroes.as_str()) {
            return (i, now.elapsed());
        }
    }
    (0, now.elapsed())
}
