use serde_json::{from_str, json, Value};
use std::{
    any::Any,
    time::{Duration, Instant},
};
pub fn solution() -> ((isize, Duration), (isize, Duration)) {
    let json_str = include_str!("../../../problem_inputs_2015/day_12.txt");
    let json = json!(json_str);
    dbg!(read_value(json));
    (solve01(&json_str), solve02(&json_str))
}

fn solve01(json_str: &str) -> (isize, Duration) {
    let now = Instant::now();
    (find_number_sum(&json_str), now.elapsed())
}

fn solve02(json_str: &str) -> (isize, Duration) {
    let now = Instant::now();
    (find_number_sum(&json_str), now.elapsed())
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

fn read_value(json: Value) -> Box<dyn Any> {
    match json {
        Value::Null => Box::new(()),
        Value::Bool(b) => Box::new(b),
        Value::Number(x) => Box::new(x),
        Value::String(a) => Box::new(a.to_owned()),
        Value::Array(arr) => Box::new(
            arr.iter()
                .map(|v| read_value(v.clone()))
                .collect::<Vec<_>>(),
        ),
        Value::Object(obj) => Box::new(
            obj.to_owned()
                .iter()
                .map(|(k, v)| (k.to_owned(), read_value(v.clone())))
                .collect::<Vec<_>>(),
        ),
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
