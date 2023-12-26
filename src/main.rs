pub mod util;
pub mod year_2023;

fn main() {
    println!(
        "The solution to part 1 is: {}\n",
        year_2023::days::day19::solution(1)
    );
    println!(
        "The solution to part 2 is: {}",
        year_2023::days::day19::solution(2)
    );
}
