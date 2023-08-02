use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solution(part: u8) -> u32 {
    if let Ok(lines) = read_lines("./problem_inputs/day1.txt") {
        let mut current_elf_calories = 0;
        let mut max_calories: Vec<u32> = vec![0; 3];
        for line in lines.flatten() {
            match line.as_str() {
                "" => {
                    max_calories.push(current_elf_calories);
                    max_calories.sort_unstable();
                    max_calories.reverse();
                    max_calories.truncate(3);
                    current_elf_calories = 0;
                }
                _ => current_elf_calories += line.parse::<u32>().unwrap(),
            }
        }
        if part == 1 {
            return max_calories[0];
        } else {
            return max_calories.iter().sum::<u32>();
        }
    }
    0
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
