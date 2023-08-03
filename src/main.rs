pub mod days;
pub mod util;

fn main() {
    let start = std::time::Instant::now();
    println!("The solution to part 1 is: {}", days::day12::solution(1));
    println!("The solution to part 2 is: {}", days::day12::solution(2));
    println!("Time elapsed: {:?}", start.elapsed());
}