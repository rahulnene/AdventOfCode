#![warn(clippy::all, clippy::pedantic, clippy::style, clippy::perf)]
pub mod year_2016;

use std::time::Instant;

use year_2016::days;

fn main() {
    let now = Instant::now();
    let ans = days::day1::solution();
    println!(
        "The solution to part 1 is: {:?}\nThe solution to part 1 is: {:?}",
        ans.0, ans.1
    );
    println!("Time: {:?}", now.elapsed());
}
