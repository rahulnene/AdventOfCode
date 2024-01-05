use itertools::Itertools;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_6.txt");

    let mut char_record: Vec<String> = vec![String::new(); lines.lines().next().unwrap().len()];
    for line in lines.lines() {
        for (i, c) in line.chars().enumerate() {
            char_record[i].push(c);
        }
    }
    (
        solve(&char_record, most_common_char),
        solve(&char_record, least_common_char),
    )
}

fn solve(char_record: &[String], f: impl Fn(&String) -> char) -> usize {
    println!("{:?}", char_record.iter().map(f).collect::<String>());
    0
}

fn most_common_char(s: &String) -> char {
    let counts = s.chars().counts();
    *counts.iter().max_by_key(|f| f.1).unwrap().0
}

fn least_common_char(s: &String) -> char {
    let counts = s.chars().counts();
    *counts
        .iter()
        .sorted_unstable_by_key(|f| f.1)
        .next()
        .unwrap()
        .0
}
