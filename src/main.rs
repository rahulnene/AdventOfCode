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
<<<<<<< HEAD
    let ans = year_2022::day16::solution();
=======
    let ans = year_2022::day7::solution();
>>>>>>> 999489142de4762b84f4c4a0854368f2632193c5
    println!("{:?} => The solution to part 1 is: {}", ans.0 .1, ans.0 .0);
    println!("{:?} => The solution to part 2 is: {}", ans.1 .1, ans.1 .0);
}
