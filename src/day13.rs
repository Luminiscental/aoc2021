use super::{day::Day, util::CollectArray};
use itertools::Itertools;
use std::{collections::VecDeque, mem};

fn fold(grid: &mut Vec<[u32; 2]>, fold: (u8, u32)) {
    for point in grid.iter_mut() {
        point[fold.0 as usize] = point[fold.0 as usize].min(2 * fold.1 - point[fold.0 as usize]);
    }
    *grid = mem::take(grid).into_iter().unique().collect();
}

pub struct Day13;

impl<'a> Day<'a> for Day13 {
    type Input = (Vec<[u32; 2]>, VecDeque<(u8, u32)>);
    type ProcessedInput = Self::Input;

    const DAY: usize = 13;

    fn parse(input: &'a str) -> Self::Input {
        let (paper, folds) = input.split("\n\n").next_tuple().unwrap();
        (
            paper
                .lines()
                .map(|s| s.split(',').map(|n| n.parse().unwrap()).collect_array())
                .collect(),
            folds
                .lines()
                .map(|s| {
                    let (axis, n) = s[11..].split('=').next_tuple().unwrap();
                    (axis.as_bytes()[0] - b'x', n.parse().unwrap())
                })
                .collect(),
        )
    }

    fn solve_part1((mut grid, mut folds): Self::Input) -> (Self::ProcessedInput, String) {
        fold(&mut grid, folds.pop_front().unwrap());
        let ans = grid.len();
        ((grid, folds), ans.to_string())
    }

    fn solve_part2((mut grid, folds): Self::ProcessedInput) -> String {
        folds.into_iter().for_each(|f| fold(&mut grid, f));
        let xmax = grid.iter().map(|point| point[0]).max().unwrap();
        let ymax = grid.iter().map(|point| point[1]).max().unwrap();
        (0..=ymax)
            .map(|y| -> String {
                (0..=xmax)
                    .map(|x| if grid.contains(&[x, y]) { '█' } else { ' ' })
                    .collect()
            })
            .join("\n")
    }
}
