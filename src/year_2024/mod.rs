use std::time::Duration;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
// pub mod day8;
// pub mod day9;
// pub mod day10;
// pub mod day11;
// pub mod day12;
// pub mod day13;
// pub mod day14;
// pub mod day15;
// pub mod day16;
// pub mod day17;
// pub mod day18;
// pub mod day19;
// pub mod day20;
// pub mod day21;
// pub mod day22;
// pub mod day23;
// pub mod day24;
// pub mod day25;

pub(crate) fn get_solution_date(date: u32, test: bool) -> ((usize, Duration), (usize, Duration)) {
    match date {
        1 => day1::solution(test),
        2 => day2::solution(test),
        3 => day3::solution(test),
        4 => day4::solution(test),
        5 => day5::solution(test),
        6 => day6::solution(test),
        7 => day7::solution(test),
        // 8 => day8::solution(test),
        // 9 => day9::solution(test),
        // 10 => day10::solution(test),
        // 11 => day11::solution(test),
        // 12 => day12::solution(test),
        // 13 => day13::solution(test),
        // 14 => day14::solution(test),
        // 15 => day15::solution(test),
        // 16 => day16::solution(test),
        // 17 => day17::solution(test),
        // 18 => day18::solution(test),
        // 19 => day19::solution(test),
        // 20 => day20::solution(test),
        // 21 => day21::solution(test),
        // 22 => day22::solution(test),
        // 23 => day23::solution(test),
        // 24 => day24::solution(test),
        // 25 => day25::solution(test),
        _ => panic!("Day not implemented"),
    }
}
