use serde_json::{from_str, Value};
use std::time::{Duration, Instant};

const JSON_STR: &str = include_str!("../../problem_inputs_2015/day_12.txt");
pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    (solve01(), solve02())
}

fn solve01() -> (isize, Duration) {
    let now = Instant::now();
    (find_number_sum(&JSON_STR), now.elapsed())
}

fn solve02() -> (isize, Duration) {
    let now = Instant::now();
    let ans = walk(&from_str(JSON_STR).unwrap());
    (ans, now.elapsed())
}

fn find_number_sum(s: &str) -> isize {
    let mut sum = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '-' || c.is_digit(10) {
            let mut num = String::new();
            num.push(c);
            while let Some(c) = chars.peek() {
                if c.is_digit(10) {
                    num.push(*c);
                    chars.next();
                } else {
                    break;
                }
            }
            sum += num.parse::<isize>().unwrap();
        }
    }
    sum
}

fn walk(json: &Value) -> isize {
    match json {
        Value::Object(map) => {
            if map.values().any(|v| v == "red") {
                0
            } else {
                map.values().map(walk).sum()
            }
        }
        Value::Array(arr) => arr.iter().map(walk).sum(),
        Value::Number(n) => n.as_i64().unwrap() as isize,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_numbers() {
        assert_eq!(find_number_sum("[1,2,3]"), 6);
        assert_eq!(find_number_sum("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(find_number_sum("[[[3]]]"), 3);
        assert_eq!(find_number_sum("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(find_number_sum("{\"a\":[-1,1]}"), 0);
        assert_eq!(find_number_sum("[-1,{\"a\":1}]"), 0);
        assert_eq!(find_number_sum("[]"), 0);
        assert_eq!(find_number_sum("{}"), 0);
    }
}
