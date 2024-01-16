use itertools::Itertools;

pub fn solution() -> (usize, usize) {
    let lines = include_str!("../../../problem_inputs_2016/day_4.txt");
    (solve01(lines), solve02(lines))
}

fn solve01(lines: &str) -> usize {
    lines.lines().map(|r| get_sector_id(r)).sum()
}

fn solve02(lines: &str) -> usize {
    lines
        .lines()
        .map(|r| decrypt_room_name(r))
        .find(|s| s.0.contains("north"))
        .unwrap()
        .1
}

fn get_sector_id(r: &str) -> usize {
    let mut c = r.split('-').collect::<Vec<_>>();
    let last_part = c.pop().unwrap();
    let parts = last_part.split('[').collect::<Vec<_>>();
    let check_sum: usize = parts[0].parse().unwrap();

    let counts = c.join("").chars().counts();
    let sorted_counts = counts
        .iter()
        .sorted_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let top_five = sorted_counts.take(5).map(|r| r.0).collect::<String>();

    if top_five == &parts[1][..5] {
        check_sum
    } else {
        0
    }
}

fn decrypt_room_name(r: &str) -> (String, usize) {
    let mut c = r.split('-').collect::<Vec<_>>();
    let last_part = c.pop().unwrap();
    let parts = last_part.split('[').collect::<Vec<_>>();
    let check_sum: usize = parts[0].parse().unwrap();

    let counts = c.join("").chars().counts();
    let sorted_counts = counts
        .iter()
        .sorted_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let top_five = sorted_counts.take(5).map(|r| r.0).collect::<String>();

    if top_five != &parts[1][..5] {
        return ("".to_string(), 0);
    } else {
        let sector_id = check_sum;
        let mut decrypted_name = String::new();
        for word in r.split('-') {
            for c in word.chars() {
                let mut new_char = c as u8 + (sector_id % 26) as u8;
                if new_char > 122 {
                    new_char = new_char - 26;
                }
                decrypted_name.push(new_char as char);
            }
            decrypted_name.push(' ');
        }
        (decrypted_name, parts[0].parse().unwrap())
    }
}

// fn get_sector_id(r: &str) -> usize {
//     let c = r.split('-').collect_vec();
//     let check_sum: usize = c[c.len() - 1]
//         .split('[')
//         .take(1)
//         .collect::<String>()
//         .parse()
//         .unwrap();
//     c[..c.len() - 1]
//         .join("")
//         .chars()
//         .counts()
//         .iter()
//         .sorted_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))
//         .take(5)
//         .map(|r| r.0)
//         .collect::<String>()
//         .eq(&c[c.len() - 1].split('[').nth(1).unwrap()[..5])
//         .then(|| check_sum)
//         .unwrap_or(0)
// }
