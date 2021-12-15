use super::{day::Day, util};
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node((u32, u32), u32);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// dijkstra with cost reduced by manhattan distance (aka A*)
fn grid_search<F: Fn((u32, u32)) -> u32>(risk: F, width: u32) -> Option<u32> {
    let mut queue = BinaryHeap::new();
    let mut seen = vec![0u64; ((width * width + 63) / 64) as usize];
    queue.push(Node((0, 0), 0));
    while let Some(Node(pos, cost)) = queue.pop() {
        if pos == (width - 1, width - 1) {
            return Some(2 * (width - 1) + cost);
        }
        let packed = (pos.0 + pos.1 * width) as usize;
        seen[packed / 64] |= 1 << (packed % 64);
        for n in util::grid_neighbours(pos, width, width).filter(|n| {
            let packed = (n.0 + n.1 * width) as usize;
            seen[packed / 64] & 1 << (packed % 64) == 0
        }) {
            queue.push(Node(n, cost + risk(n) + pos.0 + pos.1 - (n.0 + n.1)));
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
