use crate::day::Day;
use hashbrown::HashMap;
use itertools::Itertools;

type Interval = (i32, i32); // exclusive
type Cuboid = (Interval, Interval, Interval);

fn remove(region: Cuboid, from: &mut HashMap<Cuboid, i32>) {
    let mut new = Vec::new();
    for (&cuboid, coeff) in from.iter_mut() {
        match cuboid_intersection(region, cuboid) {
            Some(x) if x == cuboid => *coeff = i32::MAX,
            Some(x) => new.push((x, -*coeff)),
            None => {}
        }
    }
    from.retain(|_, &mut v| v != 0 && v != i32::MAX);
    new.into_iter()
        .for_each(|(cuboid, coeff)| *from.entry(cuboid).or_insert(0) += coeff);
}

fn count_after(instructions: impl Iterator<Item = (bool, Cuboid)>) -> i64 {
    let mut sum = HashMap::new();
    for (flag, region) in instructions {
        remove(region, &mut sum);
        if flag {
            *sum.entry(region).or_insert(0) += 1;
        }
    }
    sum.into_iter()
        .map(|(cuboid, coeff)| coeff as i64 * volume(cuboid))
        .sum()
}

pub struct Day22;

impl<'a> Day<'a> for Day22 {
    type Input = Vec<(bool, Cuboid)>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 22;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let (flag, cuboid) = line.split_once(' ').unwrap();
                let (x, y, z) = cuboid.split(',').next_tuple().unwrap();
                let (sx, ex) = x[2..].split("..").next_tuple().unwrap();
                let (sy, ey) = y[2..].split("..").next_tuple().unwrap();
                let (sz, ez) = z[2..].split("..").next_tuple().unwrap();
                let xr = (sx.parse().unwrap(), ex.parse::<i32>().unwrap() + 1);
                let yr = (sy.parse().unwrap(), ey.parse::<i32>().unwrap() + 1);
                let zr = (sz.parse().unwrap(), ez.parse::<i32>().unwrap() + 1);
                (flag == "on", (xr, yr, zr))
            })
            .collect()
    }

    fn solve_part1(instructions: Self::Input) -> (Self::ProcessedInput, String) {
        let init_region = ((-50, 51), (-50, 51), (-50, 51));
        let ans =
            count_after(instructions.iter().filter_map(|&(flag, region)| {
                Some((flag, cuboid_intersection(region, init_region)?))
            }));
        (instructions, ans.to_string())
    }

    fn solve_part2(instructions: Self::ProcessedInput) -> String {
        count_after(instructions.into_iter()).to_string()
    }
}

fn length(interval: Interval) -> i64 {
    (interval.1 - interval.0) as i64
}

fn volume(cuboid: Cuboid) -> i64 {
    length(cuboid.0) * length(cuboid.1) * length(cuboid.2)
}

fn interval_intersection(lhs: Interval, rhs: Interval) -> Option<Interval> {
    let x = (lhs.0.max(rhs.0), lhs.1.min(rhs.1));
    (x.0 < x.1).then(|| x)
}

fn cuboid_intersection(lhs: Cuboid, rhs: Cuboid) -> Option<Cuboid> {
    Some((
        interval_intersection(lhs.0, rhs.0)?,
        interval_intersection(lhs.1, rhs.1)?,
        interval_intersection(lhs.2, rhs.2)?,
    ))
}
