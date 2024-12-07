#![warn(clippy::all, clippy::pedantic, clippy::style, clippy::perf)]
#![allow(dead_code)]
#![allow(unused_variables)]

// pub mod year_2015;
// pub mod year_2016;
// pub mod year_2017;
// pub mod year_2018;
// pub mod year_2019;
// pub mod year_2020;
// pub mod year_2021;
// pub mod year_2022;
// pub mod year_2023;
pub mod year_2024;

use std::time::Duration;

use clap::{arg, Parser};

fn main() {
    let cli = Cli::parse();
    let year = cli.year;
    let date = cli.date;
    let test = match cli.test {
        0 => false,
        _ => true,
    };

    let ans: ((usize, Duration), (usize, Duration)) = get_solution(year, date, test);
    println!("{:?} => The solution to part 1 is: {}", ans.0 .1, ans.0 .0);
    println!("{:?} => The solution to part 2 is: {}", ans.1 .1, ans.1 .0);
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, default_value_t = 2024)]
    year: u32,
    #[arg(short, long)]
    date: u32,
    #[arg(short, long, action = clap::ArgAction::Count)]
    test: u8,
}

fn get_solution(year: u32, date: u32, test: bool) -> ((usize, Duration), (usize, Duration)) {
    match year {
        // 2015 => year_2015::get_solution_date(date, test),
        // 2016 => year_2016::get_solution_date(date, test),
        // 2017 => year_2017::get_solution_date(date, test),
        // 2018 => year_2018::get_solution_date(date, test),
        // 2019 => year_2019::get_solution_date(date, test),
        // 2020 => year_2020::get_solution_date(date, test),
        // 2021 => year_2021::get_solution_date(date, test),
        // 2022 => year_2022::get_solution_date(date, test),
        // 2023 => year_2023::get_solution_date(date, test),
        2024 => year_2024::get_solution_date(date, test),
        _ => panic!("Year not implemented"),
    }
}
