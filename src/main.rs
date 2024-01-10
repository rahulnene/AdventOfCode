#![warn(clippy::all, clippy::pedantic, clippy::style, clippy::perf)]
pub mod year_2023;

use year_2023::days;

fn main() {
    let ans = days::day1::solution();
    println!(
        "The solution to part 1 is: {:?}. Solved in {:?}.",
        ans.0 .0, ans.0 .1
    );
    println!(
        "The solution to part 2 is: {:?}. Solved in {:?}.",
        ans.1 .0, ans.1 .1
    );
}
