pub mod day1;

fn main() {
    let start = std::time::Instant::now();
    day1::solution();
    println!("Time elapsed: {:?}", start.elapsed());
}