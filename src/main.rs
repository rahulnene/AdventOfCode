pub mod days;
pub mod util;

fn main() {
    let start = std::time::Instant::now();
    println!("The solution to part 1 is: {}", days::day1::solution(1));
    println!("The solution to part 2 is: {}", days::day1::solution(2));
    
    println!("The solution to part 1 is: {}", days::day2::solution(1));
    println!("The solution to part 2 is: {}", days::day2::solution(2));
    
    println!("The solution to part 1 is: {}", days::day3::solution(1));
    println!("The solution to part 2 is: {}", days::day3::solution(2));
    
    println!("The solution to part 1 is: {}", days::day4::solution(1));
    println!("The solution to part 2 is: {}", days::day4::solution(2));
    
    println!("The solution to part 1 is: {}", days::day6::solution(1));
    println!("The solution to part 2 is: {}", days::day6::solution(2));
    
    println!("The solution to part 1 is: {}", days::day10::solution());
    
    println!("The solution to part 1 is: {}", days::day11::solution(1));

    println!("The solution to part 1 is: {}", days::day13::solution(1));
    println!("The solution to part 2 is: {}", days::day13::solution(2));

    
    println!("The solution to part 1 is: {}", days::day18::solution(1));
    println!("Time elapsed: {:?}", start.elapsed());
}