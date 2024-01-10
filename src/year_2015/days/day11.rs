use std::time::{Duration, Instant};

use itertools::Itertools;
pub fn solution() -> ((String, Duration), (String, Duration)) {
    let current_pw = "hepxcrrq";
    (solve01(&current_pw), solve02(&current_pw))
}

fn solve01(current_pw: &str) -> (String, Duration) {
    let now = Instant::now();
    let mut current_pw = current_pw.to_owned().into_bytes();
    increment_password(&mut current_pw);
    while !valid_password(&current_pw) {
        increment_password(&mut current_pw);
    }
    (String::from_utf8(current_pw).unwrap(), now.elapsed())
}

fn solve02(current_pw: &str) -> (String, Duration) {
    let now = Instant::now();
    let mut current_pw = current_pw.to_owned().into_bytes();
    increment_password(&mut current_pw);
    while !valid_password(&current_pw) {
        increment_password(&mut current_pw);
    }
    increment_password(&mut current_pw);
    while !valid_password(&current_pw) {
        increment_password(&mut current_pw);
    }
    (String::from_utf8(current_pw).unwrap(), now.elapsed())
}

fn increment_password(pw: &mut [u8]) {
    let mut i = pw.len() - 1;
    loop {
        if pw[i] == b'z' {
            pw[i] = b'a';
            i -= 1;
        } else {
            pw[i] += 1;
            break;
        }
    }
}

fn valid_password(pw: &[u8]) -> bool {
    pw.iter().tuple_windows().any(|(a, b, c)| {
        let a = *a as i32;
        let b = *b as i32;
        let c = *c as i32;
        a + 1 == b && b + 1 == c
    }) && !pw.iter().any(|&c| c == b'i' || c == b'o' || c == b'l')
        && pw
            .iter()
            .tuple_windows()
            .filter(|(a, b)| a == b)
            .unique()
            .count()
            >= 2
}
