use super::{day::Day, util};

pub struct Day07;

impl<'a> Day<'a> for Day07 {
    type Input = Vec<usize>;
    type ProcessedInput = (usize, Vec<usize>);

    const DAY: usize = 7;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect()
    }

    fn solve_part1(mut input: Self::Input) -> (Self::ProcessedInput, String) {
        let lower_median = *util::qselect(input.len() / 2, &mut input);
        let fuel = input
            .iter()
            .map(|n| n.abs_diff(lower_median))
            .sum::<usize>();
        ((lower_median, input), fuel.to_string())
    }

    fn solve_part2((lower_median, input): Self::ProcessedInput) -> String {
        let mean = input.iter().sum::<usize>() as f32 / input.len() as f32;
        let range = if mean > lower_median as f32 {
            lower_median..=mean.floor() as usize
        } else {
            mean.ceil() as usize..=lower_median + 1
        };
        range
            .map(|h| {
                input
                    .iter()
                    .map(|n| n.abs_diff(h))
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
 *   to minimize g(x) = \sum|h-x| + \sum(h-x)^2. The first term is minimized by
 *   the median as in part1, and the second is minimized by the mean by basic
 *   calculus. Since both are convex, we can conclude that the minimum is
 *   between the median and the mean.
 *
 *   One can also note that T(n) = (n+0.5)^2/2 - 1/8, so we could view it as
 *   minimizing g(x) = \sum(|h-x|+0.5)^2, but this is different from \sum(h-x)^2
 *   by an error of \sum|h-x|, which is significant. As it happens the answer is
 *   always an integer adjacent to the mean, and I don't understand why.
 */
