use super::{
    day::Day,
    util::{self, Summation},
};
use itertools::Itertools;

pub struct Day10;

impl<'a> Day<'a> for Day10 {
    type Input = impl 'a + Iterator<Item = &'a str>;
    type ProcessedInput = Vec<String>;

    const DAY: usize = 10;

    fn parse(input: &'a str) -> Self::Input {
        input.lines()
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
        let (completions, Summation(score)) = lines.map(validate).partition_result();
        (completions, score.to_string())
    }

    fn solve_part2(completions: Self::ProcessedInput) -> String {
        let mut scores = completions
            .into_iter()
            .map(|s| util::unradix(s.chars().map(|c| 1 + ")]}>".find(c).unwrap()), 5))
            .collect::<Vec<_>>();
        util::qselect(scores.len() / 2, &mut scores).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn test_day10_examples() {
        let input = Day10::parse(EXAMPLE);
        let (input, part1) = Day10::solve_part1(input);
        let part2 = Day10::solve_part2(input);
        assert_eq!(part1, "26397");
        assert_eq!(part2, "288957");
    }
}

bench_day!(10);
