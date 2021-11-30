use super::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    type Input = String;
    type ProcessedInput = String;

    const DAY: usize = 1;

    fn parse(input: String) -> Self::Input {
        input
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        (input, String::new())
    }

    fn solve_part2(_input: Self::ProcessedInput) -> String {
        String::new()
    }
}
