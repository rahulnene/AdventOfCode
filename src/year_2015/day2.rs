use std::time::{Duration, Instant};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let lines = include_str!("../../../problem_inputs_2015/day_2.txt");
    let presents = lines.lines().map(Rectangle::from_str).collect::<Vec<_>>();
    (wrapping_paper(&presents), ribbon(&presents))
}

fn wrapping_paper(presents: &[Rectangle]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = presents.iter().map(calculate_surface_area).sum();
    (ans, now.elapsed())
}

fn ribbon(presents: &[Rectangle]) -> (usize, Duration) {
    let now = Instant::now();
    let ans = presents.iter().map(calculate_ribbon_length).sum();
    (ans, now.elapsed())
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rectangle {
    l: usize,
    w: usize,
    h: usize,
}

impl Rectangle {
    fn from_str(line: &str) -> Self {
        let mut nums = line.split('x').map(|n| n.parse::<usize>().unwrap());
        let l = nums.next().unwrap();
        let w = nums.next().unwrap();
        let h = nums.next().unwrap();
        Self { l, w, h }
    }
}

fn calculate_surface_area(rect: &Rectangle) -> usize {
    let mut sides = vec![rect.l * rect.w, rect.w * rect.h, rect.h * rect.l];
    sides.sort_unstable();
    sides.iter().map(|n| n * 2).sum::<usize>() + sides[0]
}

fn calculate_ribbon_length(rect: &Rectangle) -> usize {
    let mut sides = vec![rect.l, rect.w, rect.h];
    sides.sort_unstable();
    sides[0] * 2 + sides[1] * 2 + sides.iter().product::<usize>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_surface_area_1() {
        let rect = super::Rectangle { l: 2, w: 3, h: 4 };
        assert_eq!(super::calculate_surface_area(&rect), 58);
    }

    #[test]
    fn test_calculate_surface_area_2() {
        let rect = super::Rectangle { l: 1, w: 1, h: 10 };
        assert_eq!(super::calculate_surface_area(&rect), 43);
    }

    #[test]
    fn test_calculate_ribbon_1() {
        let rect = super::Rectangle { l: 2, w: 3, h: 4 };
        assert_eq!(super::calculate_ribbon_length(&rect), 34);
    }

    #[test]
    fn test_calculate_ribbon_2() {
        let rect = super::Rectangle { l: 1, w: 1, h: 10 };
        assert_eq!(super::calculate_ribbon_length(&rect), 14);
    }
}
