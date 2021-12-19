use crate::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Point = (i32, i32, i32);

struct Orientation {
    roll: u8,
    face: u8,
}

impl Orientation {
    fn iter_all() -> impl Iterator<Item = Self> {
        (0..4).flat_map(|roll| (0..6).map(move |face| Self { roll, face }))
    }

    fn apply(&self, point: Point) -> Point {
        let cos = [1, 0, -1, 0][self.roll as usize];
        let sin = [0, 1, 0, -1][self.roll as usize];
        let rolled = (
            point.0,
            cos * point.1 + sin * point.2,
            -sin * point.1 + cos * point.2,
        );
        match self.face {
            0 => rolled,
            1 => (-rolled.1, rolled.0, rolled.2),
            2 => (-rolled.2, rolled.1, rolled.0),
            3 => (-rolled.0, -rolled.1, rolled.2),
            4 => (rolled.1, -rolled.0, rolled.2),
            5 => (rolled.2, rolled.1, -rolled.0),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Translation(i32, i32, i32);

impl Translation {
    fn onto(target: Point, origin: Point) -> Self {
        Self(
            target.0 - origin.0,
            target.1 - origin.1,
            target.2 - origin.2,
        )
    }

    fn manhattan_between(lhs: Self, rhs: Self) -> u32 {
        lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1) + lhs.2.abs_diff(rhs.2)
    }

    fn apply(&self, point: Point) -> Point {
        (point.0 + self.0, point.1 + self.1, point.2 + self.2)
    }
}

fn align(onto: &[Point], from: &[Point]) -> Option<(Vec<Point>, Translation)> {
    for orientation in Orientation::iter_all() {
        let mut bad = HashMap::<Point, Vec<Point>>::new();
        for &point in from[11..].iter() {
            for &ref_point in onto[11..].iter() {
                if bad.get(&point).map_or(false, |v| v.contains(&ref_point)) {
                    continue;
                }
                let translation = Translation::onto(ref_point, orientation.apply(point));
                let transform = |p| translation.apply(orientation.apply(p));
                let alignment = from
                    .iter()
                    .map(|&p| (p, transform(p)))
                    .filter(|(_p, tp)| onto.contains(tp))
                    .collect::<Vec<_>>();
                if alignment.len() >= 12 {
                    return Some((from.iter().map(|&p| transform(p)).collect(), translation));
                }
                for (p, tp) in alignment.into_iter() {
                    bad.entry(p).or_insert_with(Vec::new).push(tp);
                }
            }
        }
    }
    None
}

pub struct Day19;

impl<'a> Day<'a> for Day19 {
    type Input = Vec<Vec<Point>>;
    type ProcessedInput = Vec<Translation>;

    const DAY: usize = 19;

    fn parse(input: &'a str) -> Self::Input {
        let parse_point = |point| {
            str::split(point, ',')
                .next_tuple()
                .and_then(|(x, y, z)| Some((x.parse().ok()?, y.parse().ok()?, z.parse().ok()?)))
                .unwrap()
        };
        let parse_scan = |scan| str::lines(scan).skip(1).map(parse_point).collect();
        input.split("\n\n").map(parse_scan).collect()
    }

    fn solve_part1(scanners: Self::Input) -> (Self::ProcessedInput, String) {
        let mut found = HashMap::with_capacity(scanners.len());
        let mut translations = vec![Translation(0, 0, 0)];
        found.insert(0, scanners[0].clone());
        while found.len() < scanners.len() {
            for i in (0..scanners.len()).filter(|i| !found.contains_key(i)) {
                if let Some((beacons, translation)) =
                    found.values().find_map(|bs| align(bs, &scanners[i]))
                {
                    found.insert(i, beacons);
                    translations.push(translation);
                    break;
                }
            }
        }
        let beacons = found.into_values().flatten().collect::<HashSet<_>>();
        (translations, beacons.len().to_string())
    }

    fn solve_part2(positions: Self::ProcessedInput) -> String {
        positions
            .into_iter()
            .tuple_combinations()
            .map(|(a, b)| Translation::manhattan_between(a, b))
            .max()
            .unwrap()
            .to_string()
    }
}
