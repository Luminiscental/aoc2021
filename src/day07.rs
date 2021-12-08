use super::{day::Day, util};

pub struct Day07;

impl<'a> Day<'a> for Day07 {
    type Input = Vec<usize>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 7;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect()
    }

    fn solve_part1(mut input: Self::Input) -> (Self::ProcessedInput, String) {
        let argmin = *util::qselect(input.len() / 2, &mut input);
        let fuel = input.iter().map(|n| n.abs_diff(argmin)).sum::<usize>();
        (input, fuel.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let rounded_mean =
            (input.iter().sum::<usize>() as f32 / input.len() as f32).round() as usize;
        [rounded_mean - 1, rounded_mean, rounded_mean + 1]
            .iter()
            .map(|&candidate| {
                input
                    .iter()
                    .map(|n| n.abs_diff(candidate))
                    .map(|n| n * (n + 1) / 2)
                    .sum::<usize>()
            })
            .min()
            .unwrap()
            .to_string()
    }
}

/*
 * part1:
 *   We're minimizing f(x) = \sum|h-x| over heights h of the crabs. It's a well-
 *   known fact that this is minimized at the median (and if the length is even
 *   then both candidate medians are optimal) which can be seen by considering
 *   what happens as x changes from h-\epsilon to h+\epsilon.
 *
 * part2:
 *   Now we're minimizing f(x) = \sum T(|h-x|) where T(n) = 1 + ... + n is the
 *   triangle number with well-known formula T(n) = (n^2+n)/2. Hence it's enough
 *   to minimize g(x) = \sum|h-x| + \sum(h-x)^2. As a sum of convex functions,
 *   this is convex, and so its minimum occurs at the point where its derivative
 *   changes sign (possibly this is a jump discontinuity of the derivative like
 *   in part1). Now g'(x) = \sum sign(h-x) + 2\sum(h-x), and if N is the number
 *   of crabs then the first term is bounded by N. But plugging x=mean(h)+0.5
 *   and x=mean(h)-0.5 into the second term gives N and -N, so that the sign-
 *   change must happen between mean(h)-0.5 and mean(h)+0.5. Then the closest
 *   integers to the minimum on either side will be round(mean(h)) and one of
 *   its adjacents, and one of these must be the integer-valued minimum by
 *   convexity.
 */
