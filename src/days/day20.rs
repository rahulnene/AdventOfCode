pub fn solution(part: u8) -> isize {
    let start = std::time::Instant::now();
    let lines = include_str!("../../problem_inputs/day20.txt");
    println!("Parsed in {:?}", start.elapsed());
    match part {
        1 => decode(lines, 1, 1),
        2 => decode(lines, 811589153, 10),
        _ => 0,
    }
}

type IndexAndValue = (usize, isize);

fn decode(lines: &str, key: isize, rounds: u16) -> isize {
    let start = std::time::Instant::now();
    let mut array: Vec<IndexAndValue> = lines
        .lines()
        .map(|s| s.parse::<isize>().unwrap() * key)
        .enumerate()
        .collect();
    let original = array.clone();
    for _ in 0..rounds {
        original.iter().enumerate().for_each(|(_, num)| {
            process(&mut array, num);
        });
    }
    println!("Decoded in {:?}", start.elapsed());
    get_coords(&array)
}

fn get_coords(arr: &Vec<IndexAndValue>) -> isize {
    let len = arr.len();
    let zero_pos = arr.iter().position(|x| x.1 == 0).unwrap();
    let x1 = arr[(zero_pos + 1_000) % len].1;
    let x2 = arr[(zero_pos + 2_000) % len].1;
    let x3 = arr[(zero_pos + 3_000) % len].1;
    x1 + x2 + x3
}

fn process(arr: &mut Vec<IndexAndValue>, to_process: &IndexAndValue) {
    let len = arr.len();
    let current_index = arr.iter().position(|x| x.0 == to_process.0).unwrap();
    let val = arr[current_index].1;
    let new_index = current_index as isize + val;
    let new_index = new_index.rem_euclid(len as isize - 1);
    let temp: (usize, isize) = arr.remove(current_index);
    arr.insert(new_index as usize, temp);
}
