use super::{day::Day, util::Ignore};
use itertools::Itertools;
use std::collections::HashSet;

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
            .map(|n| n.parse().unwrap())
            .next_tuple()
            .unwrap();
        Self {
            start: (sx, sy),
            end: (ex, ey),
            delta: (ex - sx, ey - sy),
        }
    }

    fn for_overlaps<F: FnMut((i32, i32))>(&self, o: &Line, mut f: F) {
        let denom = self.delta.1 * o.delta.0 - self.delta.0 * o.delta.1;
        if denom == 0 {
            if self.delta.0 * (o.start.1 - self.start.1)
                == self.delta.1 * (o.start.0 - self.start.0)
            {
                let endpoints = |s: i32, e: i32, os: i32, oe: i32, d: i32, od: i32| {
                    if d == 0 {
                        (s, e)
                    } else {
                        let o_fst = ((os + oe) + d * od * (os - oe)) / 2;
                        let o_lst = os + oe - o_fst;
                        (d * (d * s).max(d * o_fst), d * (d * e).min(d * o_lst))
                    }
                };
                let (dx, dy) = (self.delta.0.signum(), self.delta.1.signum());
                let (odx, ody) = (o.delta.0.signum(), o.delta.1.signum());
                let (sx, ex) = endpoints(self.start.0, self.end.0, o.start.0, o.end.0, dx, odx);
                let (sy, ey) = endpoints(self.start.1, self.end.1, o.start.1, o.end.1, dy, ody);
                let (nx, ny) = (dx * (ex - sx), dy * (ey - sy));
                if nx >= 0 && ny >= 0 {
                    (0..=nx.max(ny)).for_each(|i| f((sx + i * dx, sy + i * dy)));
                }
            }
        } else {
            let t = self.start.0 * o.delta.1
                + o.start.0 * (self.start.1 - o.end.1)
                + o.end.0 * (o.start.1 - self.start.1);
            let s = -(self.start.0 * (o.start.1 - self.end.1)
                + self.end.0 * (self.start.1 - o.start.1)
                + o.start.0 * self.delta.1);
            let inside = |t| (0 <= t && t <= denom) || (denom <= t && t <= 0);
            let exact = self.delta.0 * self.delta.1 * o.delta.0 * o.delta.1 == 0
                || ((t * self.delta.0 % denom == 0) && (s * o.delta.0 % denom == 0));
            if inside(t) && inside(s) && exact {
                f((
                    self.start.0 + t * self.delta.0 / denom,
                    self.start.1 + t * self.delta.1 / denom,
                ));
            }
        }
    }
}

fn count_overlaps<'a>(lines: impl Clone + Iterator<Item = &'a Line>) -> usize {
    let mut overlaps = HashSet::new();
    lines
        .tuple_combinations::<(_, _)>()
        .for_each(|ls| ls.0.for_overlaps(ls.1, |p| overlaps.insert(p).ignore()));
    overlaps.len()
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
