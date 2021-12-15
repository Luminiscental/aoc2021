use super::{
    day::Day,
    util::{self, BitSet},
};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

/// dijkstra with cost reduced by manhattan distance (aka A*)
fn grid_search<F: Fn((u32, u32)) -> u32>(risk: F, width: u32) -> Option<u32> {
    let pack = |(x, y)| (x + y * width) as usize;
    let mut queue = PriorityQueue::new();
    let mut seen = BitSet::with_capacity((width * width) as usize);
    queue.push((0, 0), Reverse(0));
    while let Some((pos, Reverse(cost))) = queue.pop() {
        if pos == (width - 1, width - 1) {
            return Some(2 * (width - 1) + cost);
        }
        seen.insert(pack(pos));
        for n in util::grid_neighbours(pos, width, width).filter(|&n| !seen.contains(pack(n))) {
            queue.push_increase(n, Reverse(cost + risk(n) + pos.0 + pos.1 - n.0 - n.1));
        }
    }
    None
}

pub struct Day15;

impl<'a> Day<'a> for Day15 {
    type Input = (u32, Vec<u32>);
    type ProcessedInput = Self::Input;

    const DAY: usize = 15;

    fn parse(input: &'a str) -> Self::Input {
        let width = input.lines().next().unwrap().len() as u32;
        let grid = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        (width, grid)
    }

    fn solve_part1((width, grid): Self::Input) -> (Self::ProcessedInput, String) {
        let ans = grid_search(|(x, y)| grid[(x + y * width) as usize], width).unwrap();
        ((width, grid), ans.to_string())
    }

    fn solve_part2((width, grid): Self::ProcessedInput) -> String {
        grid_search(
            |(x, y)| {
                let base_cost = grid[((x % width) + (y % width) * width) as usize];
                1 + (x / width + y / width + base_cost - 1) % 9
            },
            5 * width,
        )
        .unwrap()
        .to_string()
    }
}

#[cfg(test)]
mod test {
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
