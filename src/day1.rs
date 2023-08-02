use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Elf {
    index: u32,
    calories: u32,
}

fn main() {
    if let Ok(lines) = read_lines("./problem1.txt") {
        let mut current_elf = Elf {
            index: 0,
            calories: 0,
        };
        let mut max_calories: Vec<u32> = vec![0; 3];
        for line in lines {
            if let Ok(food_information) = line {
                match food_information.as_str() {
                    "" => {
                        current_elf = Elf {
                            index: current_elf.index + 1,
                            calories: 0,
                        }
                    }
                    _ => current_elf.calories += food_information.parse::<u32>().unwrap(),
                }
                if current_elf.calories > max_calories[0] {
                    max_calories.push(current_elf.calories);
                    max_calories.remove(0);
                    max_calories.sort_unstable();
                }
            }
        }
        println!("Calorie sum is {:?}", max_calories.iter().sum::<u32>());
        dbg!(max_calories);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
