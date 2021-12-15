use super::{day::Day, util};
use itertools::{iproduct, Itertools};

fn adjacents(p: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    [(1, 0), (0, 1), (usize::MAX, 0), (0, usize::MAX)]
        .iter()
        .map(move |d| (p.0.wrapping_add(d.0), p.1.wrapping_add(d.1)))
        .filter(|p| p.0 < 100 && p.1 < 100)
}

pub struct Day09;

impl<'a> Day<'a> for Day09 {
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
        for point in iproduct!(0..100, 0..100) {
            let height = heights[100 * point.0 + point.1];
            if adjacents(point).all(|p| heights[100 * p.0 + p.1] > height) {
                low_points.push(point);
                risk += height + 1;
            }
        }
        ((low_points, heights), risk.to_string())
    }

    fn solve_part2((low_points, heights): Self::ProcessedInput) -> String {
        let neighbours = |p| adjacents(p).filter(|&a| heights[100 * a.0 + a.1] != 9);
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

bench_day!(09);
