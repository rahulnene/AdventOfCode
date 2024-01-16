use itertools::Itertools;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_3.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    lines
        .lines()
        .map(|tri_str| {
            tri_str
                .split_ascii_whitespace()
                .collect_tuple()
                .map(|t| check_triangle(t))
                .unwrap()
        })
        .filter(|b| *b)
        .count()
}

fn solve02(lines: &str) -> usize {
    0
}

fn check_triangle((a, b, c): (&str, &str, &str)) -> bool {
    let a = a.parse::<usize>().unwrap();
    let b = b.parse::<usize>().unwrap();
    let c = c.parse::<usize>().unwrap();
    a + b > c && a + c > b && b + c > a
}
