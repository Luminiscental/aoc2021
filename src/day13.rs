use super::{day::Day, util::CollectArray};
use itertools::Itertools;
use std::{
    collections::{HashSet, VecDeque},
    mem,
};

fn fold(grid: &mut HashSet<[u16; 2]>, fold: (u8, u16)) {
    *grid = mem::take(grid)
        .into_iter()
        .map(|mut point| {
            point[fold.0 as usize] =
                u16::min(point[fold.0 as usize], 2 * fold.1 - point[fold.0 as usize]);
            point
        })
        .collect();
}

pub struct Day13;

impl<'a> Day<'a> for Day13 {
    type Input = (HashSet<[u16; 2]>, VecDeque<(u8, u16)>);
    type ProcessedInput = Self::Input;

    const DAY: usize = 13;

    fn parse(input: &'a str) -> Self::Input {
        let (paper, folds) = input.split("\n\n").next_tuple().unwrap();
        let parse_point = |s: &str| s.split(',').map(|n| n.parse().unwrap()).collect_array();
        let parse_fold = |s: &str| {
            let (axis, n) = s[11..].split('=').next_tuple().unwrap();
            (axis.as_bytes()[0] - b'x', n.parse().unwrap())
        };
        (
            paper.lines().map(parse_point).collect(),
            folds.lines().map(parse_fold).collect(),
        )
    }

    fn solve_part1((mut grid, mut folds): Self::Input) -> (Self::ProcessedInput, String) {
        fold(&mut grid, folds.pop_front().unwrap());
        let ans = grid.len();
        ((grid, folds), ans.to_string())
    }

    fn solve_part2((mut grid, folds): Self::ProcessedInput) -> String {
        folds.into_iter().for_each(|f| fold(&mut grid, f));
        let get_char = |point| if grid.contains(&point) { '█' } else { ' ' };
        (0..=5)
            .map(|y| -> String { (0..=38).map(|x| get_char([x, y])).collect() })
            .join("\n")
    }
}
