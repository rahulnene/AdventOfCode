#![warn(clippy::all, clippy::pedantic, clippy::style, clippy::perf)]
pub mod year_2018;

use year_2018::days;

fn main() {
    println!("The solution to part 1 is: {:?}", days::day11::solution(1));
    println!("\n");
    println!("The solution to part 2 is: {:?}", days::day11::solution(2));
}
