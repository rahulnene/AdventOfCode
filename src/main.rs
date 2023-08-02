pub mod days;

fn main() {
    let start = std::time::Instant::now();
    days::day1::solution();
    println!("Time elapsed: {:?}", start.elapsed());
}