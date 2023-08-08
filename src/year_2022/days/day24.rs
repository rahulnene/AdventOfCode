use std::{
    iter::Sum,
    ops::{Add, AddAssign},
};

pub fn solution(part: u8) -> String {
    let lines = include_str!("../../../problem_inputs/day25.txt");
    match part {
        1 => SNAFU::to_str(lines.lines().map(|line| SNAFU::from_str(line)).sum()),
        _ => "Bad input".to_string(),
    }
}

#[derive(Debug, Clone, Copy)]
struct SNAFU {
    val: isize,
}

impl SNAFU {
    fn from_str(s: &str) -> Self {
        s.chars()
            .rev()
            .enumerate()
            .fold(SNAFU { val: 0 }, |acc, c| {
                let five_pow = 5_isize.pow(c.0 as u32);
                acc + SNAFU {
                    val: match c.1 {
                        '1' => 1 * five_pow,
                        '-' => -1 * five_pow,
                        '2' => 2 * five_pow,
                        '=' => -2 * five_pow,
                        '0' => 0,
                        _ => panic!("Invalid input"),
                    },
                }
            })
    }

    fn to_str(self) -> String {
        let mut val = self.val;
        let mut s = String::new();
        while val != 0 {
            let rem = val % 5;
            val /= 5;
            match rem {
                0 => s.push('0'),
                1 => s.push('1'),
                2 => s.push('2'),
                3 => {
                    val += 1;
                    s.push('=')
                }
                4 => {
                    val += 1;
                    s.push('-')
                }
                _ => panic!("Invalid input"),
            }
        }
        s.chars().rev().collect()
    }
}

impl Add for SNAFU {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.val += other.val;
        self
    }
}

impl Sum for SNAFU {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(SNAFU { val: 0 }, |acc, x| acc + x)
    }
}

impl AddAssign for SNAFU {
    fn add_assign(&mut self, other: Self) {
        self.val += other.val;
    }
}
