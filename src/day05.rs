use super::day::Day;
use itertools::Itertools;

#[derive(Clone, Copy)]
pub struct Line {
    start: (i32, i32),
    end: (i32, i32),
    delta: (i32, i32),
}

impl Line {
    fn parse(string: &str) -> Self {
        let (sx, sy, ex, ey) = string
            .split(" -> ")
            .flat_map(|s| s.split(','))
            .map(|n| n.parse::<i32>().unwrap())
            .next_tuple()
            .unwrap();
        let delta = ((ex - sx).signum(), (ey - sy).signum());
        Self {
            start: (sx, sy),
            end: (ex + delta.0, ey + delta.1),
            delta,
        }
    }
}

impl Iterator for Line {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        (self.start != self.end).then(|| {
            let point = self.start;
            self.start.0 += self.delta.0;
            self.start.1 += self.delta.1;
            point
        })
    }
}

fn count_overlaps<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
    lines.flat_map(|&line| line).duplicates().count()
}

pub struct Day05;

impl<'a> Day<'a> for Day05 {
    type Input = Vec<Line>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 5;

    fn parse(input: &'a str) -> Self::Input {
        input.lines().map(Line::parse).collect()
    }

    fn solve_part1(lines: Self::Input) -> (Self::ProcessedInput, String) {
        let axis_aligned = |line: &&Line| line.delta.0 == 0 || line.delta.1 == 0;
        let overlaps = count_overlaps(lines.iter().filter(axis_aligned));
        (lines, overlaps.to_string())
    }

    fn solve_part2(lines: Self::ProcessedInput) -> String {
        count_overlaps(lines.iter()).to_string()
    }
}
