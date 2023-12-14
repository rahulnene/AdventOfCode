use fxhash::FxHashMap;

pub fn solution(part: usize) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_15.txt");
    match part {
        1 => solve(lines, 2020),
        2 => solve(lines, 30000000),
        _ => 1,
    }
}

fn solve(lines: &str, turn: usize) -> usize {
    let mut said_numbers = lines
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .enumerate()
        .fold(FxHashMap::default(), |mut said_numbers, (i, n)| {
            said_numbers.insert(n, (i + 1, 0_usize));
            said_numbers
        });
    let mut last_number = lines
        .trim_end()
        .split(',')
        .map(|n| n.parse().unwrap())
        .last()
        .unwrap_or_default();
    for turn in (said_numbers.len() + 1)..=turn {
        let (last_turn, prev_turn) = said_numbers.get(&last_number).unwrap();
        if *prev_turn == 0 {
            last_number = 0;
        } else {
            last_number = last_turn - prev_turn;
        }
        if let Some((last_turn, _)) = said_numbers.get(&last_number) {
            said_numbers.insert(last_number, (turn, *last_turn));
        } else {
            said_numbers.insert(last_number, (turn, 0));
        }
    }
    said_numbers
        .iter()
        .find_map(|(k, v)| if v.0 == turn { Some(*k) } else { None })
        .unwrap()
}
