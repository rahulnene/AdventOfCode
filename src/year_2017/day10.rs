use std::time::{Duration, Instant};

const TOTAL_NUM: usize = 256;

pub fn solution() -> ((usize, Duration), (String, Duration)) {
    let lines = include_str!("../../problem_inputs_2017/day_10.txt");
    (solve01(lines), solve02(lines.as_bytes()))
}

fn solve01(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut numbers = Vec::from_iter(0..TOTAL_NUM);
    let mut current_position = 0;
    let mut skip_size = 0;
    let lengths: Vec<usize> = lines.split(',').map(|x| x.parse().unwrap()).collect();
    twist_once(
        &mut numbers,
        &mut current_position,
        &mut skip_size,
        &lengths,
    );
    let result = numbers[0] * numbers[1];
    (result, now.elapsed())
}

fn solve02(lines: &[u8]) -> (String, Duration) {
    let now = Instant::now();
    let mut numbers = Vec::from_iter(0..TOTAL_NUM);
    let mut current_position = 0;
    let mut skip_size = 0;
    let mut lengths: Vec<usize> = lines.iter().map(|x| *x as usize).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    for _ in 0..64 {
        twist_once(
            &mut numbers,
            &mut current_position,
            &mut skip_size,
            &lengths,
        );
    }

    let ans = densify_and_to_hex(&numbers);
    (ans, now.elapsed())
}

fn twist_once(
    numbers: &mut [usize],
    current_position: &mut usize,
    skip_size: &mut usize,
    lengths: &[usize],
) {
    for length in lengths {
        let mut temp = Vec::new();
        for i in 0..*length {
            temp.push(numbers[(*current_position + i) % TOTAL_NUM]);
        }
        temp.reverse();
        for i in 0..*length {
            numbers[(*current_position + i) % TOTAL_NUM] = temp[i];
        }
        *current_position += length + *skip_size;
        *skip_size += 1;
    }
}

fn densify_and_to_hex(sparse: &[usize]) -> String {
    sparse
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &v| acc ^ v as usize))
        .map(|v| format!("{:02x}", v))
        .collect()
}
