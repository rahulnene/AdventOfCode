pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2019/day_1.txt");
    match part {
        1 => lines
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .map(calculate_fuel01)
            .sum(),
        2 => lines
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .map(calculate_fuel02)
            .sum(),
        _ => 1,
    }
}

fn calculate_fuel01(original_mass: usize) -> usize {
    original_mass / 3 - 2
}

fn calculate_fuel02(original_mass: usize) -> usize {
    let mut mass = original_mass as isize;
    let mut total = 0;
    while mass > 0 {
        total += mass;
        mass = mass / 3 - 2;
    }
    total as usize - original_mass
}
