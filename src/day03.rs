use super::{day::Day, util};

fn most_common_bit(values: &[usize], pos: usize) -> usize {
    let ones = values.iter().filter(|&n| (n >> pos) & 1 != 0).count();
    (2 * ones >= values.len()).into()
}

fn bit_filter(
    width: usize,
    values: &[usize],
    selector: impl Fn(usize, &[usize]) -> usize,
) -> usize {
    let mut candidates = values.to_vec();
    for i in (0..width).rev() {
        let bit = selector(i, &candidates);
        candidates.retain(|n| (n >> i) & 1 == bit);
        if candidates.len() == 1 {
            return candidates[0];
        }
    }
    panic!()
}

pub struct Day03;

impl<'a> Day<'a> for Day03 {
    type Input = (usize, Vec<usize>);
    type ProcessedInput = Self::Input;

    const DAY: usize = 3;

    fn parse(input: &'a str) -> Self::Input {
        let width = input.lines().next().unwrap().len();
        let values = input
            .lines()
            .map(|line| usize::from_str_radix(line, 2).unwrap())
            .collect();
        (width, values)
    }

    fn solve_part1((width, values): Self::Input) -> (Self::ProcessedInput, String) {
        let gamma = util::unradix((0..width).map(|n| most_common_bit(&values, n)), 2);
        let epsilon = (!gamma) & ((1 << width) - 1);
        ((width, values), (gamma * epsilon).to_string())
    }

    fn solve_part2((width, values): Self::ProcessedInput) -> String {
        let generator = bit_filter(width, &values, |n, c| most_common_bit(c, n));
        let scrubber = bit_filter(width, &values, |n, c| 1 - most_common_bit(c, n));
        (generator * scrubber).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn test_day03_examples() {
        let input = Day03::parse(EXAMPLE);
        let (input, part1) = Day03::solve_part1(input);
        let part2 = Day03::solve_part2(input);
        assert_eq!(part1, "198");
        assert_eq!(part2, "230");
    }
}

bench_day!(03);
