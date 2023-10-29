pub mod util;
pub mod year_2019;

fn main() {
    println!(
        "The solution to part 1 is: {}\n",
        year_2019::days::day2::solution(1)
    );
    println!(
        "The solution to part 2 is: {}",
        year_2019::days::day2::solution(2)
    );
}
