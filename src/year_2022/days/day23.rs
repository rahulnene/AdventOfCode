pub fn solution(part: u8) -> isize {
    let lines = include_str!("../../../problem_inputs/day23.txt");
    match part {
        1 => solve01(lines),
        // 2 => solve02(lines),
        _ => -1,
    }
}

fn solve01(lines: &str) -> isize {
    let mut map = Map { locs: Vec::new() };
    for line in lines.lines() {
        map.locs.push(Row::from_line(line));
    }
    println!("{:?}", map);
    0
}
fn solve02(lines: &str) -> isize {
    0
}

#[derive(Debug, Clone, Copy)]
struct Row {
    locs: u128,
}

impl Row {
    fn from_line(line: &str) -> Self {
        line.chars()
            .into_iter()
            .map(|f| match f {
                '.' => 0,
                '#' => 1,
                _ => panic!("Invalid char"),
            })
            .fold(Row { locs: 0 }, |mut acc, f| {
                acc.locs <<= 1;
                acc.locs |= f;
                acc
            })
    }
}

#[derive(Debug, Clone)]
struct Map {
    locs: Vec<Row>,
}
