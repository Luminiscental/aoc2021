use super::{day::Day, util};
use itertools::{iproduct, Itertools};

fn adjacents<const WIDTH: usize, const HEIGHT: usize>(
    (r, c): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    [(1, 0), (0, 1), (usize::MAX, 0), (0, usize::MAX)]
        .iter()
        .map(move |&(dr, dc)| (r.wrapping_add(dr), c.wrapping_add(dc)))
        .filter(|&(r, c)| r < HEIGHT && c < WIDTH)
}

pub struct Day09Generic<const WIDTH: usize, const HEIGHT: usize>;
pub type Day09 = Day09Generic<100, 100>;

impl<'a, const WIDTH: usize, const HEIGHT: usize> Day<'a> for Day09Generic<WIDTH, HEIGHT> {
    type Input = Vec<u32>;
    type ProcessedInput = (Vec<(usize, usize)>, Vec<u32>);

    const DAY: usize = 9;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect()
    }

    fn solve_part1(heights: Self::Input) -> (Self::ProcessedInput, String) {
        let mut low_points = Vec::new();
        let mut risk = 0;
        for (r, c) in iproduct!(0..HEIGHT, 0..WIDTH) {
            let height = heights[WIDTH * r + c];
            if adjacents::<WIDTH, HEIGHT>((r, c)).all(|(nr, nc)| heights[WIDTH * nr + nc] > height)
            {
                low_points.push((r, c));
                risk += height + 1;
            }
        }
        ((low_points, heights), risk.to_string())
    }

    fn solve_part2((low_points, heights): Self::ProcessedInput) -> String {
        let neighbours =
            |p| adjacents::<WIDTH, HEIGHT>(p).filter(|&(r, c)| heights[WIDTH * r + c] != 9);
        low_points
            .into_iter()
            .map(|point| util::bfs(point, neighbours, |_| {}).len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn test_day09_examples() {
        let input = Day09Generic::<10, 5>::parse(EXAMPLE);
        let (input, part1) = Day09Generic::<10, 5>::solve_part1(input);
        let part2 = Day09Generic::<10, 5>::solve_part2(input);
        assert_eq!(part1, "15");
        assert_eq!(part2, "1134");
    }
}

bench_day!(09);
