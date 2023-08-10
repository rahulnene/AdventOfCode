pub mod year_2021;
// pub mod year_2022;
pub mod util;

fn main() {
    println!(
        "The solution to part 1 is: {}\n",
        year_2021::days::day9::solution(1)
    );
    println!(
        "The solution to part 2 is: {}",
        year_2021::days::day9::solution(2)
    );
}
