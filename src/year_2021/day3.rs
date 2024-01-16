use itertools::Itertools;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2021/day_3.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    let (gamma, epsilon) = calc_gamma_epsilon(&lines.lines().collect_vec());
    gamma * epsilon
}

fn count_ones_and_zeros(strings: &[&str]) -> Vec<usize> {
    let mut ones_count = vec![0; strings[0].len()];

    for string in strings {
        for (i, c) in string.chars().enumerate() {
            if '1' == c {
                ones_count[i] += 1;
            }
        }
    }
    ones_count
}

fn calc_gamma_epsilon(strings: &[&str]) -> (usize, usize) {
    let n = strings.len();
    let ones_count = &count_ones_and_zeros(strings);
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();
    for count in ones_count {
        let gamma = *count / n;
        let (ch_gamma, ch_epsilon) = if gamma > 0 { ('1', '0') } else { ('0', '1') };
        gamma_str.push(ch_gamma);
        epsilon_str.push(ch_epsilon);
    }
    (
        usize::from_str_radix(&gamma_str, 2).unwrap(),
        usize::from_str_radix(&epsilon_str, 2).unwrap(),
    )
}

fn solve02(lines: &str) -> usize {
    let mut nums = Vec::new();
    for line in lines.lines() {
        let mut num_str = String::new();
        for char in line.chars() {
            match char {
                '1' => num_str.push('1'),
                '0' => num_str.push('0'),
                _ => (),
            }
        }
        nums.push(num_str);
    }

    calc_carbon_rating(&nums) * calc_oxygen_rating(&nums)
}

fn calc_common_digit(nums: &[String], most_common: bool) -> Vec<String> {
    let mut nums = Vec::from(nums);
    let mut index: usize = 0;
    while nums.len() > 1 {
        let common_dig = {
            if (nums
                .iter()
                .filter(|num| num.chars().nth(index).unwrap() == '0')
                .count()
                > nums.len() / 2)
                == most_common
            {
                '0'
            } else {
                '1'
            }
        };
        let a = nums
            .iter()
            .filter(|num| num.chars().nth(index).unwrap() == common_dig)
            .map(std::string::ToString::to_string)
            .collect_vec();
        nums = a;
        index += 1;
    }
    nums
}

fn calc_oxygen_rating(nums: &[String]) -> usize {
    let nums = calc_common_digit(nums, true);
    usize::from_str_radix(nums.first().unwrap(), 2).unwrap()
}

fn calc_carbon_rating(nums: &[String]) -> usize {
    let nums = calc_common_digit(nums, false);
    usize::from_str_radix(nums.first().unwrap(), 2).unwrap()
}
