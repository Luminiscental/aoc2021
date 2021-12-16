use crate::day::Day;

pub struct Day06;

fn simulate(populations: &mut [u64; 9], time: u32) {
    for _ in 0..time {
        populations.rotate_left(1);
        populations[6] += populations[8];
    }
}

impl<'a> Day<'a> for Day06 {
    type Input = [u64; 9];
    type ProcessedInput = Self::Input;

    const DAY: usize = 6;

    fn parse(input: &'a str) -> Self::Input {
        let mut populations = [0; 9];
        for timer in input.trim().split(',').map(|n| n.parse::<usize>().unwrap()) {
            populations[timer] += 1;
        }
        populations
    }

    fn solve_part1(mut populations: Self::Input) -> (Self::ProcessedInput, String) {
        simulate(&mut populations, 80);
        (populations, populations.iter().sum::<u64>().to_string())
    }

    fn solve_part2(mut populations: Self::ProcessedInput) -> String {
        simulate(&mut populations, 256 - 80);
        populations.iter().sum::<u64>().to_string()
    }
}

#[cfg(test)]
mod test_day06 {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_day06_examples() {
        let input = Day06::parse(EXAMPLE);
        let (input, part1) = Day06::solve_part1(input);
        let part2 = Day06::solve_part2(input);
        assert_eq!(part1, "5934");
        assert_eq!(part2, "26984457539");
    }
}

bench_day!(06);

/*
 * Alternate solution:
 *
 * ```python
 * import numpy as np
 *
 * steps = np.matrix([[0, 0, 0, 0, 0, 0, 1, 0, 1],
 *                    [1, 0, 0, 0, 0, 0, 0, 0, 0],
 *                    [0, 1, 0, 0, 0, 0, 0, 0, 0],
 *                    [0, 0, 1, 0, 0, 0, 0, 0, 0],
 *                    [0, 0, 0, 1, 0, 0, 0, 0, 0],
 *                    [0, 0, 0, 0, 1, 0, 0, 0, 0],
 *                    [0, 0, 0, 0, 0, 1, 0, 0, 0],
 *                    [0, 0, 0, 0, 0, 0, 1, 0, 0],
 *                    [0, 0, 0, 0, 0, 0, 0, 1, 0]])
 * coeffs80 = (steps ** 80) @ np.array([1] * 9)
 * coeffs256 = (steps ** 256) @ np.array([1] * 9)
 * ```
 *
 * Then part1 = coeffs80.dot(populations), part2 = coeffs256.dot(populations).
 */
