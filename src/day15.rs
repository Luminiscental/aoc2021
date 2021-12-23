use crate::{day::Day, util};
use itertools::iproduct;

pub struct Day15;

fn min_risk(width: usize, grid: &[u8]) -> usize {
    let risk = |(x, y)| grid[x + y * width] as usize;
    util::dijkstra(
        (0, 0),
        |p| util::grid_neighbours(p, width, width).map(|n| (risk(n), n)),
        |p| p == (width - 1, width - 1),
    )
    .unwrap()
}

impl<'a> Day<'a> for Day15 {
    type Input = (usize, Vec<u8>);
    type ProcessedInput = Self::Input;

    const DAY: usize = 15;

    fn parse(input: &'a str) -> Self::Input {
        let width = input.lines().next().unwrap().len();
        let grid = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        (width, grid)
    }

    fn solve_part1((width, grid): Self::Input) -> (Self::ProcessedInput, String) {
        let ans = min_risk(width, &grid);
        ((width, grid), ans.to_string())
    }

    fn solve_part2((width, grid): Self::ProcessedInput) -> String {
        min_risk(
            5 * width,
            &iproduct!(0..5, 0..width, 0..5, 0..width)
                .map(|(r, y, c, x)| grid[x + y * width] + r + c)
                .map(|risk| 1 + (risk - 1) % 9)
                .collect::<Vec<_>>(),
        )
        .to_string()
    }
}

#[cfg(test)]
mod test_day15 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "};

    #[test]
    fn test_day15_examples() {
        let input = Day15::parse(EXAMPLE);
        let (input, part1) = Day15::solve_part1(input);
        let part2 = Day15::solve_part2(input);
        assert_eq!(part1, "40");
        assert_eq!(part2, "315");
    }
}

bench_day!(15);
