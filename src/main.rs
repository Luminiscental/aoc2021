use std::env;

mod day;
mod day01;

use day::Day;
use day01::Day01;

macro_rules! solve {
    ($day:ident) => {{
        match $day::get_input() {
            Ok(input) => $day::solve_and_print(input),
            Err(err) => eprintln!("{}", err),
        }
    }};
    ($day:ident, $($days:ident),+) => {{
        solve!($day);
        solve!($days)
    }};
}

fn main() {
    match env::args().nth(1).as_deref() {
        None => solve!(Day01), // latest
        Some("all") => solve!(Day01),
        Some(day) => match day.parse::<usize>() {
            Err(err) => eprintln!("Expected day number (or \"all\") as argument ({})", err),
            Ok(1) => solve!(Day01),
            Ok(n) if (1..=25).contains(&n) => todo!(),
            Ok(_) => eprintln!("That's not a day of advent!",),
        },
    }
}
