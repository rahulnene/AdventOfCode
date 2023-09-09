pub mod util;
pub mod year_2020;

fn main() {
    println!(
        "The solution to part 1 is: {}\n",
        year_2020::days::day9::solution(1)
    );
    println!(
        "The solution to part 2 is: {}",
        year_2020::days::day9::solution(2)
    );
}
