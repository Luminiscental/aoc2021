use super::day::Day;
use std::collections::HashSet;

/// dijkstra with cost reduced by distance (aka A*)
fn grid_search<F: Fn((u32, u32)) -> u32>(risk: F, width: u32) -> Option<u32> {
    let distance = |(x, y)| 2 * width - 2 - x - y;
    let neighbours = |(x, y): (u32, u32)| {
        [(1, 0), (0, 1), (u32::MAX, 0), (0, u32::MAX)]
            .iter()
            .map(move |d| (x.wrapping_add(d.0), y.wrapping_add(d.1)))
            .filter(|&p| p.0 < width && p.1 < width)
    };
    let mut queue = vec![((0, 0), 0)];
    let mut seen = HashSet::new();
    while let Some((pos, cost)) = queue.pop() {
        if pos == (width - 1, width - 1) {
            return Some(distance((0, 0)) + cost);
        }
        seen.insert(pos);
        for n in neighbours(pos).filter(|n| !seen.contains(n)) {
            queue.push((n, cost + risk(n) + distance(n) - distance(pos)));
        }
        queue.sort_unstable_by_key(|&(_, cost)| u32::MAX - cost);
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
