#![feature(int_abs_diff)]
#![feature(type_alias_impl_trait)]

use paste::paste;
use std::env;

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod util;

use day::Day;
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;
use day10::Day10;

macro_rules! solve {
    ($day:literal) => {{
        paste! {
            match [<Day $day>]::get_input() {
                Ok(input) => [<Day $day>]::solve_and_print(&input),
                Err(err) => eprintln!("{}", err),
            }
        }
    }};
    ($day:literal, $($days:literal),+) => {{
        solve!($day);
        solve!($($days),+)
    }}
}

macro_rules! match_days {
    ($day_string:ident, $($days:literal),+) => {{
        match $day_string {
            "all" => solve!($($days),+),
            day => match day.parse::<usize>() {
                Err(err) => eprintln!("Expected day number (or \"all\") as argument ({})", err),
                $(Ok($days) => solve!($days)),+,
                Ok(n) if (1..=25).contains(&n) => todo!(),
                Ok(_) => eprintln!("That's not a day of advent!"),
            }
        }
    }}
}

fn main() {
    match env::args().nth(1).as_deref() {
        None => solve!(10), // latest
        Some(day) => match_days!(day, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10),
    }
}
