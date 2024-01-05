use itertools::Itertools;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_7.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    lines.lines().filter(|l| test_validity(l)).count()
}

fn solve02(lines: &str) -> usize {
    0
}

fn test_validity(line: &str) -> bool {
    let a = line.split(|c| c == '[' || c == ']').collect_vec();
    for (i, s) in a.iter().enumerate() {
        return i % 2 == 0 && has_abba(s);
    }
    false
}

fn has_abba(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_validity_1() {
        assert_eq!(super::test_validity("abba[mnop]qrst"), true);
    }
    #[test]
    fn test_validity_2() {
        assert_eq!(super::test_validity("abcd[bddb]xyyx"), false);
    }
    #[test]
    fn test_validity_3() {
        assert_eq!(super::test_validity("aaaa[qwer]tyui"), false);
    }
    #[test]
    fn test_validity_4() {
        assert_eq!(super::test_validity("ioxxoj[asdfgh]zxcvbn"), true);
    }
}
