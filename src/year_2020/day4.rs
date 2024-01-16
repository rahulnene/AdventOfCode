use regex::Regex;

pub fn solution(part: u8) -> usize {
    let lines = include_str!("../../../problem_inputs_2020/day_4.txt");
    match part {
        1 => solve01(lines),
        2 => solve02(lines),
        _ => 1,
    }
}

fn solve01(lines: &str) -> usize {
    lines
        .split("\n\n")
        .filter(|passport| check(passport))
        .count()
}

fn solve02(lines: &str) -> usize {
    let mut count = 0;
    for passport in lines.split("\n\n") {
        if check(passport) {
            if let Ok(a) = check_fields(passport) {
                count += a as usize;
            }
        }
    }
    count
}

fn check(passport: &str) -> bool {
    let req_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    !req_fields.iter().any(|field| !passport.contains(field))
}

fn check_fields(passport: &str) -> Result<bool, ()> {
    let byr = Regex::new(r"(byr:\d{4})").unwrap();
    let iyr = Regex::new(r"(iyr:\d{4})").unwrap();
    let eyr = Regex::new(r"(eyr:\d{4})").unwrap();
    let hgt = Regex::new(r"(hgt:\d*(cm|in))").unwrap();
    let hcl = Regex::new(r"(hcl:#[0-9a-f]{6})").unwrap();
    let ecl = Regex::new(r"(ecl:(amb|blu|brn|gry|grn|hzl|oth))").unwrap();
    let pid = Regex::new(r"(pid:\d{9}\b)").unwrap();

    let byr = byr.find(passport).ok_or(())?.as_str()[4..]
        .parse::<usize>()
        .unwrap();
    let iyr = iyr.find(passport).ok_or(())?.as_str()[4..]
        .parse::<usize>()
        .unwrap();
    let eyr = eyr.find(passport).ok_or(())?.as_str()[4..]
        .parse::<usize>()
        .unwrap();
    let hgt = hgt.find(passport).ok_or(())?.as_str()[4..].to_string();
    let _hcl = hcl.find(passport).ok_or(())?.as_str()[4..].to_string();
    let _ecl = ecl.find(passport).ok_or(())?.as_str()[4..].to_string();
    let _pid = pid.find(passport).ok_or(())?.as_str()[4..].to_string();
    let years_good =
        (1920..2002).contains(&byr) && (2010..=2020).contains(&iyr) && (2020..=2030).contains(&eyr);

    let hgt_good = match &hgt[(hgt.len() - 2)..] {
        "cm" => {
            let hgt = hgt[..(hgt.len() - 2)].parse::<usize>().unwrap();
            (150..=193).contains(&hgt)
        }
        "in" => {
            let hgt = hgt[..(hgt.len() - 2)].parse::<usize>().unwrap();
            (59..=76).contains(&hgt)
        }
        _ => false,
    };

    Ok(years_good && hgt_good)
}
