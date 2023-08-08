use crate::util::read_lines;
use itertools::Itertools;

pub fn solution(part: u8) -> u32 {
    let lines = read_lines("./problem_inputs/day18.txt").unwrap();
    match part {
        1 => part1(lines),
        2 => part2(lines),
        _ => 0,
    }
}

fn part1(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    let cubes: Vec<Cube> = lines
        .flatten()
        .map(|line| {
            let (x, y, z) = line.split(',').collect_tuple().unwrap();
            Cube {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            }
        })
        .collect();
    surface_area(&cubes)
}

fn part2(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    todo!()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
}

fn is_adj(c1: Cube, c2: Cube) -> bool {
    u32::abs_diff(c1.x, c2.x) + u32::abs_diff(c1.y, c2.y) + u32::abs_diff(c1.z, c2.z) == 1
}

fn surface_area(sys: &[Cube]) -> u32 {
    let area: u32 = 6 * sys.len() as u32;
    let adj_area: u32 = sys
        .iter()
        .enumerate()
        .flat_map(|(i, cube)| {
            sys.iter()
                .skip(i + 1)
                .filter(|&other| is_adj(*cube, *other))
                .map(|_| 2)
        })
        .sum();
    area - adj_area
}
