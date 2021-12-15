use super::day::Day;

use itertools::Itertools;

pub struct Day02;

impl<'a> Day<'a> for Day02 {
    type Input = Vec<(i32, i32)>;
    type ProcessedInput = Vec<(i32, i32)>;

    const DAY: usize = 2;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let (dir, amount) = line.split(' ').next_tuple().unwrap();
                let amount = amount.parse::<i32>().unwrap();
                match dir {
                    "forward" => (amount, 0),
                    "up" => (0, -amount),
                    "down" => (0, amount),
                    _ => panic!(),
                }
            })
            .collect()
    }

    fn solve_part1(instructions: Self::Input) -> (Self::ProcessedInput, String) {
        let (mut hor, mut ver) = (0, 0);
        for (dh, dv) in instructions.iter() {
            hor += dh;
            ver += dv;
        }
        (instructions, (hor * ver).to_string())
    }

    fn solve_part2(instructions: Self::ProcessedInput) -> String {
        let (mut hor, mut ver, mut aim) = (0, 0, 0);
        for (dh, dv) in instructions.iter() {
            hor += dh;
            ver += aim * dh;
            aim += dv;
        }
        (hor * ver).to_string()
    }
}

bench_day!(02);
