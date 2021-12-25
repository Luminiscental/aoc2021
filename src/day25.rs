use crate::day::Day;
use itertools::iproduct;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    East,
    South,
    Empty,
    MovedEast,
    MovedSouth,
    MovedEmpty,
}

fn step(width: usize, height: usize, cucumbers: &mut [TileState]) -> bool {
    let mut stepped = false;
    for (y, x) in iproduct!(0..height, 0..width) {
        if cucumbers[x + y * width] == TileState::East {
            let step_x = (x + 1) % width;
            if cucumbers[step_x + y * width] == TileState::Empty {
                cucumbers[step_x + y * width] = TileState::MovedEast;
                cucumbers[x + y * width] = TileState::MovedEmpty;
                stepped = true;
            }
        }
    }
    cucumbers
        .iter_mut()
        .filter(|s| **s == TileState::MovedEmpty)
        .for_each(|s| *s = TileState::Empty);
    for (y, x) in iproduct!(0..height, 0..width) {
        if cucumbers[x + y * width] == TileState::South {
            let step_y = (y + 1) % height;
            if cucumbers[x + step_y * width] == TileState::Empty {
                cucumbers[x + step_y * width] = TileState::MovedSouth;
                cucumbers[x + y * width] = TileState::MovedEmpty;
                stepped = true;
            }
        }
    }
    for s in cucumbers.iter_mut() {
        match s {
            TileState::MovedEast => *s = TileState::East,
            TileState::MovedSouth => *s = TileState::South,
            TileState::MovedEmpty => *s = TileState::Empty,
            _ => {}
        }
    }
    stepped
}

pub struct Day25;

impl<'a> Day<'a> for Day25 {
    type Input = (usize, usize, Vec<TileState>);
    type ProcessedInput = ();

    const DAY: usize = 25;

    fn parse(input: &'a str) -> Self::Input {
        let width = input.find('\n').unwrap();
        let cucumbers = input
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '>' => TileState::East,
                    'v' => TileState::South,
                    _ => TileState::Empty,
                })
            })
            .collect::<Vec<_>>();
        let height = cucumbers.len() / width;
        (width, height, cucumbers)
    }

    fn solve_part1((width, height, mut cucumbers): Self::Input) -> (Self::ProcessedInput, String) {
        let mut steps = 1;
        while step(width, height, &mut cucumbers) {
            steps += 1;
        }
        ((), steps.to_string())
    }

    fn solve_part2(_: Self::ProcessedInput) -> String {
        "Merry Christmas!".to_string()
    }
}
