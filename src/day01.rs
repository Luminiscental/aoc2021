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

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_day01_examples() {
        let input = Day01::parse(EXAMPLE);
        let (input, part1) = Day01::solve_part1(input);
        let part2 = Day01::solve_part2(input);
        assert_eq!(part1, "7");
        assert_eq!(part2, "5");
    }
}

bench_day!(01);
