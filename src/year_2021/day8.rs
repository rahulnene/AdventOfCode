use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_8.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let mut count = 0;
    for line in lines.lines() {
        let output = line.split_once(" | ").unwrap().1;
        let a = output
            .trim()
            .split(' ')
            .map(|f| f.to_string())
            .collect_vec();
        for i in 0..4 {
            match a[i].len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            };
        }
    }
    count
}

fn solve02(lines: &str) -> usize {
    let mut sum = 0;
    for line in lines.lines() {
        let mut ans: usize = 0;
        let output = line.split_once(" | ").unwrap().1;
        let a = output
            .trim()
            .split(' ')
            .map(|f| f.to_string())
            .collect_vec();
        for i in 0..4 {
            ans += match a[i].as_str() {
                "ab" => 10_usize.pow((4 - i) as u32),
                "gcdfa" => 2 * 10_usize.pow((4 - i) as u32),
                "fbcad" => 3 * 10_usize.pow((4 - i) as u32),
                "eafb" => 4 * 10_usize.pow((4 - i) as u32),
                "cdfbe" => 5 * 10_usize.pow((4 - i) as u32),
                "cdfgeb" => 6 * 10_usize.pow((4 - i) as u32),
                "dab" => 7 * 10_usize.pow((4 - i) as u32),
                "acedgfb" => 8 * 10_usize.pow((4 - i) as u32),
                "cefabd" => 9 * 10_usize.pow((4 - i) as u32),
                _ => {
                    dbg!(&a[i]);
                    unreachable!()
                }
            };
        }
        dbg!(ans);
        sum += ans;
    }
    sum
}
