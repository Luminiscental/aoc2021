use super::{
    day::Day,
    util::{self, Summation},
};
use itertools::Itertools;

pub struct Day10;

impl<'a> Day<'a> for Day10 {
    type Input = Vec<&'a str>;
    type ProcessedInput = Vec<String>;

    const DAY: usize = 10;

    fn parse(input: &'a str) -> Self::Input {
        input.lines().collect()
    }

    fn solve_part1(lines: Self::Input) -> (Self::ProcessedInput, String) {
        fn validate(line: &str) -> Result<String, usize> {
            let mut stack = Vec::new();
            for c in line.chars() {
                if let Some(i) = "([{<".find(c) {
                    stack.push(b")]}>"[i]);
                } else if stack.pop() != Some(c as u8) {
                    return Err([3, 57, 1197, 25137][")]}>".find(c).unwrap()]);
                }
            }
            Ok(String::from_utf8(stack).unwrap())
        }
        let (completions, Summation(score)) = lines.into_iter().map(validate).partition_result();
        (completions, score.to_string())
    }

    fn solve_part2(completions: Self::ProcessedInput) -> String {
        let mut scores = completions
            .into_iter()
            .map(|completion| {
                completion
                    .chars()
                    .rev()
                    .map(|c| ")]}>".find(c).unwrap())
                    .fold(0, |s, n| 5 * s + n + 1)
            })
            .collect::<Vec<_>>();
        util::qselect(scores.len() / 2, &mut scores).to_string()
    }
}
