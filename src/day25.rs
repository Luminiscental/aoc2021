use crate::day::Day;

// TODO: 4 tiles can be packed into one u8

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    East,
    South,
    Moved,
    Empty,
}

fn step(width: usize, height: usize, cucumbers: &mut [TileState]) -> bool {
    let mut stepped = false;
    for y in 0..height {
        let mut x = 0;
        while x < width {
            if cucumbers[x + y * width] == TileState::East {
                let step_x = (x + 1) % width;
                if cucumbers[step_x + y * width] == TileState::Empty {
                    cucumbers[step_x + y * width] = TileState::East;
                    cucumbers[x + y * width] = TileState::Moved;
                    stepped = true;
                    x += 1;
                }
            }
            x += 1;
        }
    }
    cucumbers
        .iter_mut()
        .filter(|s| **s == TileState::Moved)
        .for_each(|s| *s = TileState::Empty);
    for x in 0..width {
        let mut y = 0;
        while y < height {
            if cucumbers[x + y * width] == TileState::South {
                let step_y = (y + 1) % height;
                if cucumbers[x + step_y * width] == TileState::Empty {
                    cucumbers[x + step_y * width] = TileState::South;
                    cucumbers[x + y * width] = TileState::Moved;
                    stepped = true;
                    y += 1;
                }
            }
            y += 1;
        }
    }
    cucumbers
        .iter_mut()
        .filter(|s| **s == TileState::Moved)
        .for_each(|s| *s = TileState::Empty);
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

#[cfg(test)]
mod test_day25 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>
    "};

    #[test]
    fn test_day25_examples() {
        let input = Day25::parse(EXAMPLE);
        let (_, part1) = Day25::solve_part1(input);
        assert_eq!(part1, "58");
    }
}

bench_day!(25);
