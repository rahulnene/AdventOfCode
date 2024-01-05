pub fn solution() -> (usize, usize) {
    let line = include_str!("../../../problem_inputs_2017/day_11.txt");
    (solve01(line), solve02(line))
}

fn solve01(line: &str) -> usize {
    let mut position = (0, 0);
    line.split(',').for_each(|hex| {
        let delta = hex_to_delta(hex);
        position.0 += delta.0;
        position.1 += delta.1;
    });
    dbg!(position);
    (position.0.abs().max(position.1.abs())) as usize
}

fn solve02(lines: &str) -> usize {
    0
}

fn hex_to_delta(hex: &str) -> (isize, isize) {
    match hex {
        "n" => (1, -1),
        "ne" => (1, 0),
        "se" => (0, 1),
        "s" => (-1, 1),
        "sw" => (-1, 0),
        "nw" => (0, -1),
        _ => panic!("Invalid hex direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve01() {
        assert_eq!(solve01("ne,ne,ne"), 3);
    }

    #[test]
    fn test_solve02() {
        assert_eq!(solve01("ne,ne,sw,sw"), 0);
    }
    #[test]
    fn test_solve03() {
        assert_eq!(solve01("ne,ne,s,s"), 2);
    }
    #[test]
    fn test_solve04() {
        assert_eq!(solve01("se,sw,se,sw,sw"), 3);
    }
}
