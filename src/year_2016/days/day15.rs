use itertools::Itertools;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2017/day_15.txt");
    (
        solve(lines, 40_000_000, false),
        solve(lines, 5_000_000, true),
    )
}

fn solve(lines: &str, iters: usize, modulo_flag: bool) -> usize {
    let (a_seed, b_seed) = lines
        .lines()
        .map(|f| f.split(' ').last().unwrap().parse().unwrap())
        .collect_tuple()
        .unwrap();
    let mut prng_a = PRNG::new(a_seed, 16807);
    let mut prng_b = PRNG::new(b_seed, 48271);
    compare(modulo_flag, &mut prng_a, &mut prng_b, iters)
}

struct PRNG {
    current_value: usize,
    factor: usize,
}

impl PRNG {
    fn new(seed: usize, factor: usize) -> Self {
        PRNG {
            current_value: seed,
            factor,
        }
    }

    fn progress(&mut self, modulo_flag: bool, modulo: usize) -> usize {
        loop {
            self.current_value = (self.current_value * self.factor) % 2147483647;
            if self.current_value % modulo == 0 || !modulo_flag {
                return self.current_value;
            }
        }
    }

    fn get_lowest_16_bits(&self) -> usize {
        self.current_value & 0xFFFF
    }
}

fn compare(modulo_flag: bool, prng_a: &mut PRNG, prng_b: &mut PRNG, iterations: usize) -> usize {
    let mut matches = 0;
    for _ in 0..iterations {
        if prng_a.get_lowest_16_bits() == prng_b.get_lowest_16_bits() {
            matches += 1;
        }
        prng_a.progress(modulo_flag, 4);
        prng_b.progress(modulo_flag, 8);
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve01() {
        let mut prng_a = PRNG::new(65, 16807);
        let mut prng_b = PRNG::new(8921, 48271);
        assert_eq!(compare(false, &mut prng_a, &mut prng_b, 40_000_000), 588);
    }
    #[test]
    fn test_solve02() {
        let mut prng_a = PRNG::new(65, 16807);
        let mut prng_b = PRNG::new(8921, 48271);
        assert_eq!(compare(true, &mut prng_a, &mut prng_b, 5_000_000), 309);
    }
}
