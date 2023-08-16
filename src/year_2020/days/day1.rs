pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_1.txt");
    let mut numbers: Vec<usize> = lines
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    numbers.sort();
    match part {
        1 => solve01(&numbers),
        2 => solve02(&numbers),
        _ => 1,
    }
}

fn solve01(numbers: &[usize]) -> usize {
    if let Some((a, b)) = two_sum(&numbers, 2020) {
        return a * b;
    }
    0
}

fn solve02(numbers: &[usize]) -> usize {
    for num in numbers {
        if let Some((a, b)) = two_sum(&numbers, 2020 - num) {
            return a * b * num;
        }
    }
    0
}

fn two_sum(numbers: &[usize], target: usize) -> Option<(usize, usize)> {
    let (mut low, mut hi) = (0, numbers.len() - 1);
    while low < hi {
        let sum = numbers[low] + numbers[hi];
        if sum == target {
            return Some((numbers[low], numbers[hi]));
        } else if sum < target {
            low += 1;
        } else {
            hi -= 1;
        }
    }
    None
}
