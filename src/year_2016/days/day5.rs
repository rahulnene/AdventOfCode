use fxhash::FxHashSet;

pub fn solution() -> (usize, usize) {
    let seed = "uqwqemis";
    (solve01(seed), solve02(seed))
}

fn solve01(seed: &str) -> usize {
    let a = (0..)
        .filter_map(|i| {
            let to_hash = format!("{}{}", seed, i);
            let digest = format!("{:x}", &md5::compute(to_hash));
            if &digest[..5] == "00000" {
                Some(digest[5..6].to_string())
            } else {
                None
            }
        })
        .take(8)
        .collect::<String>();
    println!("Actual solution to Part 1 is: {}", a);
    0
}

fn solve02(seed: &str) -> usize {
    let mut seen_positions = FxHashSet::default();
    let a = (0..)
        .filter_map(|i| {
            let to_hash = format!("{}{}", seed, i);
            let digest = format!("{:x}", &md5::compute(to_hash));
            if &digest[..5] == "00000" {
                if digest[5..6].parse::<usize>().is_ok()
                    && digest[5..6].parse::<usize>().unwrap() < 8
                    && seen_positions.insert(digest[5..6].parse::<usize>().unwrap())
                {
                    Some((digest[5..6].to_string(), digest[6..7].to_string()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .take(8)
        .collect::<Vec<_>>();
    let mut password = vec![' '; 8];
    for (pos, c) in a.iter() {
        if let Ok(pos) = pos.parse::<usize>() {
            if pos < 8 && password[pos] == ' ' {
                password[pos] = c.chars().next().unwrap();
            }
        }
    }
    let ans = password.iter().collect::<String>();
    println!("Actual solution to Part 2 is: {:?}", ans);
    0
}
