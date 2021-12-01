use super::day::Day;

pub struct Day02;

impl Day for Day02 {
    type Input = String;
    type ProcessedInput = String;

    const DAY: usize = 2;

    fn parse(input: String) -> Self::Input {
        input
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        (input, String::new())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        String::new()
    }
}
