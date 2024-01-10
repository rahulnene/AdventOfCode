use std::time::{Instant, Duration};
pub fn solution() -> ((usize, Duration), (usize, Duration)) {
    let (target_row, target_col) = (2978, 3083);
    (solve01(target_row, target_col), solve02(target_row, target_col))
}

fn solve01(target_row: usize, target_col: usize) -> (usize, Duration) {
    let now = Instant::now();
    let mut code = 20151125;
    let target_index = convert_to_index(target_row, target_col);
    for _ in 1..target_index {
        code = next_code(code);
    }
    (code, now.elapsed())
}

fn solve02(target_row: usize, target_col: usize) -> (usize, Duration) {
    let now = Instant::now();
    (0, now.elapsed())
}

fn convert_to_index(row: usize, col: usize) -> usize {
    let n = row + col;
    let triangular = n*(n-1)/2;
    triangular - row + 1
}

fn next_code(code: usize) -> usize {
    (code * 252533) % 33554393
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn index_test_1() {
        assert_eq!(convert_to_index(1, 1), 1);
    }
    #[test]
    fn index_test_2() {
        assert_eq!(convert_to_index(4, 2), 12);
    }
    #[test]
    fn index_test_3() {
        assert_eq!(convert_to_index(3, 3), 13);
    }
}