pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2017/day_5.txt");
    match part {
        1 => solve(lines, |_| 1),
        2 => solve(lines, |f| if f >= 3 { -1 } else { 1 }),
        _ => 1,
    }
}

fn solve(lines: &str, f: impl Fn(isize) -> isize) -> usize {
    let mut offsets: Vec<isize> = lines.lines().map(|f| f.parse::<isize>().unwrap()).collect();
    let mut pointer: isize = 0;
    let mut steps = 0;
    while pointer >= 0 && (pointer as usize) < offsets.len() {
        let offset = offsets[pointer as usize];
        offsets[pointer as usize] += f(offset);
        pointer = pointer + offset;
        steps += 1;
    }
    steps
}
