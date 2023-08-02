pub mod days;
pub mod util;

fn main() {
    let start = std::time::Instant::now();
    days::day2::solution();
    println!("Time elapsed: {:?}", start.elapsed());
}