pub fn solution(part: u8) -> String {
    let lines = include_str!("../../../problem_inputs/day25.txt");
    match part {
        1 => solve01(lines),
        _ => "Bad input".to_string(),
    }
}

pub fn solve01(lines: &str) -> String {
    let mut sum: isize = 0;
    for line in lines.lines() {
        sum += SNAFU::from_str(line).val;
    }
    SNAFU::to_str(sum)
}

#[derive(Debug)]
struct SNAFU {
    val: isize,
}

impl SNAFU {
    fn from_str(s: &str) -> Self {
        s.chars()
            .rev()
            .enumerate()
            .fold(SNAFU { val: 0 }, |acc, c| {
                let mut new = acc;
                match c.1 {
                    '1' => new.val += 1 * 5_isize.pow(c.0 as u32),
                    '-' => new.val -= 1 * 5_isize.pow(c.0 as u32),
                    '2' => new.val += 2 * 5_isize.pow(c.0 as u32),
                    '=' => new.val -= 2 * 5_isize.pow(c.0 as u32),
                    '0' => (),
                    _ => {
                        dbg!(c);
                        panic!("Invalid input")
                    }
                }
                new
            })
    }

    fn to_str(mut val: isize) -> String {
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
