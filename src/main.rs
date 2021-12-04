use paste::paste;
use std::env;

mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod util;

use day::Day;
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;

macro_rules! solve {
    ($day:literal) => {{
        paste! {
            match [<Day $day>]::get_input() {
                Ok(input) => [<Day $day>]::solve_and_print(input),
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
        None => solve!(04), // latest
        Some(day) => match_days!(day, 01, 02, 03, 04),
    }
}
