#![warn(clippy::all, clippy::pedantic, clippy::style, clippy::perf)]
pub mod year_2017;

use year_2017::days;

fn main() {
    let ans = days::day15::solution();
    println!(
        "The solution to part 1 is: {:?}\nThe solution to part 1 is: {:?}",
        ans.0, ans.1
    );
}
