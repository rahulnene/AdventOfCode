#![warn(clippy::all, clippy::pedantic, clippy::style, clippy::perf)]
pub mod year_2015;
pub mod year_2016;
pub mod year_2017;
pub mod year_2018;
pub mod year_2019;
pub mod year_2020;
pub mod year_2021;
pub mod year_2022;
pub mod year_2023;

fn main() {
    let ans = year_2015::day24::solution();
    println!("{:?} => The solution to part 1 is: {}", ans.0 .1, ans.0 .0);
    println!("{:?} => The solution to part 2 is: {}", ans.1 .1, ans.1 .0);
}
