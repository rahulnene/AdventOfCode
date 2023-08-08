use crate::util::read_lines;
use itertools::Itertools;
use std::error::Error;

pub fn solution(part: u8) -> u32 {
    let lines = read_lines("./problem_inputs/day8.txt").unwrap();
    match part {
        1 => part1(lines),
        2 => part2(lines),
        _ => 0,
    }
}

fn part1(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    // let mut forest: Vec<Vec<u8>> = Vec::new();
    // lines.for_each(|line| {
    //     let mut forest_row: Vec<u8> = Vec::new();
    //     line.unwrap().chars().for_each(|c| {
    //         forest_row.push(c as u8);
    //     });
    //     forest.push(forest_row);
    // });
    // let (forest_height, forest_width) = (forest.len(), forest[0].len());
    // let mut forest_mask: Vec<Vec<bool>> = vec![vec![false; 31]; 323];
    // for row in 0..forest_height {
    //     for col in 0..forest_width {}
    // }
    todo!();
}

// fn test_sight(
//     forest: &Vec<Vec<u8>>,
//     forest_map: &mut Vec<Vec<bool>>,
//     row: usize,
//     col: usize,
// ) -> Result<bool, ()> {
//     let sight = (forest[row][col] > (*forest[row].get(col - 1))?);
// }

fn part2(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> u32 {
    todo!();
}
