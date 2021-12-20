use crate::{day::Day, util::CollectArray};
use itertools::Itertools;
use std::{
    collections::VecDeque,
    mem,
};
use hashbrown::HashSet;

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

fn decode(char_idx: u16, grid: &HashSet<[u16; 2]>) -> char {
    let b = char_idx * 5;
    let check = |x, y| grid.contains(&[x, y]) as u8;
    match (check(b, 0), check(b + 3, 0), check(b, 5), check(b + 3, 5)) {
        (0, 0, 0, 0) => 'C',
        (0, 0, 0, 1) => 'G',
        (0, 0, 1, 1) => 'A',
        (0, 1, 0, 0) => 'J',
        (1, 0, 1, 0) if check(b + 1, 5) == 1 => 'B',
        (1, 0, 1, 0) => 'P',
        (1, 0, 1, 1) if check(b + 1, 0) == 1 => 'R',
        (1, 0, 1, 1) => 'L',
        (1, 1, 0, 0) => 'U',
        (1, 1, 1, 0) => 'F',
        (1, 1, 1, 1) if check(b + 3, 4) == 1 => 'H',
        (1, 1, 1, 1) if check(b + 1, 3) == 1 => 'Z',
        (1, 1, 1, 1) if check(b + 1, 0) == 1 => 'E',
        (1, 1, 1, 1) => 'K',
        _ => panic!(),
    }
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
        (0..=7).map(|i| decode(i, &grid)).collect()
    }
}

#[cfg(test)]
mod test_day13 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn test_day13_examples() {
        let input = Day13::parse(EXAMPLE);
        let (_, part1) = Day13::solve_part1(input);
        assert_eq!(part1, "17");
    }
}

bench_day!(13);
