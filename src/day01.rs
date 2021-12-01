use super::day::Day;
use itertools::izip;

pub struct Day01 {}

impl Day for Day01 {
    type Input = Vec<i32>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 1;

    fn parse(input: String) -> Self::Input {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let deltas = izip!(input.iter(), input.iter().skip(1)).map(|(a, b)| b - a);
        let increases = deltas.filter(|&d| d > 0).count();
        (input, increases.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let windows = izip!(input.iter(), input.iter().skip(1), input.iter().skip(2));
        let sums: Vec<_> = windows.map(|(a, b, c)| a + b + c).collect();
        let deltas = izip!(sums.iter(), sums.iter().skip(1)).map(|(a, b)| b - a);
        let increases = deltas.filter(|&d| d > 0).count();
        increases.to_string()
    }
}
