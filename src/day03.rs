use super::day::Day;

use std::ops::Not;

fn most_common_bit(values: &[usize], pos: usize) -> usize {
    values
        .iter()
        .copied()
        .map(|n| (n >> pos) & 1)
        .map(|b| 2 * (b as i32) - 1)
        .sum::<i32>()
        .is_negative()
        .not()
        .into()
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

impl Day for Day03 {
    type Input = (usize, Vec<usize>);
    type ProcessedInput = Self::Input;

    const DAY: usize = 3;

    fn parse(input: String) -> Self::Input {
        let width = input.lines().next().unwrap().len();
        let values = input
            .lines()
            .map(|line| usize::from_str_radix(line, 2).unwrap())
            .collect();
        (width, values)
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let (width, values) = input;
        let gamma = (0..width)
            .map(|n| most_common_bit(&values, n))
            .zip(itertools::iterate(1, |i| 2 * i))
            .map(|p| p.0 * p.1)
            .sum::<usize>();
        let epsilon = (!gamma) & ((1 << width) - 1);
        ((width, values), (gamma * epsilon).to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let (width, values) = input;
        let generator = bit_filter(width, &values, |n, candidates| {
            most_common_bit(candidates, n)
        });
        let scrubber = bit_filter(width, &values, |n, candidates| {
            1 - most_common_bit(candidates, n)
        });
        (generator * scrubber).to_string()
    }
}
