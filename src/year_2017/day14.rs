use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    // let given = "uugsqrei";
    let given = "uugsqrei";
    (solve01(given), solve02(given))
}

fn solve01(given: &str) -> (usize, Duration) {
    let now = Instant::now();
    let mut ans = 0;
    for i in 0..128 {
        let input = format!("{}-{}", given, i);
        let hash: String = hash(input.as_bytes());
        let bits = count_ones_in_hex(&hash);
        ans += bits;
    }
    (ans, now.elapsed())
}

fn solve02(lines: &str) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn hash(s: &[u8]) -> String {
    let mut numbers = Vec::from_iter(0..256);
    let mut current_position = 0;
    let mut skip_size = 0;
    let mut lengths: Vec<usize> = s.iter().map(|x| *x as usize).collect();
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
    ans
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
            temp.push(numbers[(*current_position + i) % 256]);
        }
        temp.reverse();
        for i in 0..*length {
            numbers[(*current_position + i) % 256] = temp[i];
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

fn count_ones_in_hex(s: &str) -> usize {
    let mut ans = 0;
    for c in s.chars() {
        let n = match c {
            '0' => 0,
            '1' => 1,
            '2' => 1,
            '3' => 2,
            '4' => 1,
            '5' => 2,
            '6' => 2,
            '7' => 3,
            '8' => 1,
            '9' => 2,
            'a' => 2,
            'b' => 3,
            'c' => 2,
            'd' => 3,
            'e' => 3,
            'f' => 4,
            _ => panic!("Invalid hex character"),
        };
        ans += n;
    }
    ans
}
