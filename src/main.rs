pub mod days;
pub mod util;

fn main() {
    let start = std::time::Instant::now();
    println!("The solution to part 1 is: {}", days::day10::solution());
    println!("Time elapsed: {:?}", start.elapsed());
}