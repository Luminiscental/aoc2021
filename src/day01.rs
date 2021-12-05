use super::day::Day;

pub struct Day01;

fn convoluted_increases(values: &[i32], window_size: usize) -> usize {
    values
        .iter()
        .zip(values[window_size..].iter())
        .filter(|(curr, next)| next > curr)
        .count()
}

impl<'a> Day<'a> for Day01 {
    type Input = Vec<i32>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 1;

    fn parse(input: &'a str) -> Self::Input {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }

    fn solve_part1(depths: Self::Input) -> (Self::ProcessedInput, String) {
        let increases = convoluted_increases(&depths, 1);
        (depths, increases.to_string())
    }

    fn solve_part2(depths: Self::ProcessedInput) -> String {
        convoluted_increases(&depths, 3).to_string()
    }
}
