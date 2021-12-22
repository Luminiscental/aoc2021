use crate::day::Day;
use hashbrown::HashSet;
use itertools::{iproduct, Itertools};
use std::{array::IntoIter, ops::RangeInclusive};

type R = RangeInclusive<i32>;
type Cuboid = (R, R, R);

fn split_up(
    c: &Cuboid,
    split: &Cuboid,
    cs: &mut Vec<Cuboid>,
    splits: &mut Vec<(bool, Cuboid)>,
    flag: bool,
) -> bool {
    if c.0.start() > split.0.end()
        || c.0.end() < split.0.start()
        || c.1.start() > split.1.end()
        || c.1.end() < split.1.start()
        || c.2.start() > split.2.end()
        || c.2.end() < split.2.start()
    {
        return false;
    }
    let int = |l: R, r: R| -> R {
        if l.start() < r.start() {
            if l.end() > r.end() {
                r
            } else {
                *r.start()..=*l.end()
            }
        } else if l.end() < r.end() {
            l
        } else {
            *l.start()..=*r.end()
        }
    };
    let parts = |i: R, o: R| -> (R, R) { (*o.start()..=*i.start() - 1, i.end() + 1..=*o.end()) };
    let xint = int(c.0.clone(), split.0.clone());
    let yint = int(c.1.clone(), split.1.clone());
    let zint = int(c.2.clone(), split.2.clone());
    let (x_c0, x_c1) = parts(xint.clone(), c.0.clone());
    let (y_c0, y_c1) = parts(yint.clone(), c.1.clone());
    let (z_c0, z_c1) = parts(zint.clone(), c.2.clone());
    let (x_s0, x_s1) = parts(xint.clone(), split.0.clone());
    let (y_s0, y_s1) = parts(yint.clone(), split.1.clone());
    let (z_s0, z_s1) = parts(zint.clone(), split.2.clone());
    for cc in IntoIter::new([
        (x_c0, c.1.clone(), c.2.clone()),
        (x_c1, c.1.clone(), c.2.clone()),
        (xint.clone(), y_c0, c.2.clone()),
        (xint.clone(), y_c1, c.2.clone()),
        (xint.clone(), yint.clone(), z_c0),
        (xint.clone(), yint.clone(), z_c1),
    ]) {
        if cc.0.is_empty() || cc.1.is_empty() || cc.2.is_empty() {
            continue;
        }
        cs.push(cc);
    }
    for sc in IntoIter::new([
        (x_s0, split.1.clone(), split.2.clone()),
        (x_s1, split.1.clone(), split.2.clone()),
        (xint.clone(), y_s0, split.2.clone()),
        (xint.clone(), y_s1, split.2.clone()),
        (xint.clone(), yint.clone(), z_s0),
        (xint.clone(), yint.clone(), z_s1),
    ]) {
        if sc.0.is_empty() || sc.1.is_empty() || sc.2.is_empty() {
            continue;
        }
        splits.push((flag, sc));
    }
    if flag {
        cs.push((xint, yint, zint));
    }
    true
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
                let xr = sx.parse().unwrap()..=ex.parse().unwrap();
                let yr = sy.parse().unwrap()..=ey.parse().unwrap();
                let zr = sz.parse().unwrap()..=ez.parse().unwrap();
                (flag == "on", (xr, yr, zr))
            })
            .collect()
    }

    fn solve_part1(instructions: Self::Input) -> (Self::ProcessedInput, String) {
        let mut reactor = HashSet::new();
        let outside = |r: &R| *r.end() < -50 || *r.start() > 50;
        for (flag, (xr, yr, zr)) in instructions.iter() {
            if outside(xr) || outside(yr) || outside(zr) {
                continue;
            }
            for (x, y, z) in iproduct!(xr.clone(), yr.clone(), zr.clone()) {
                if *flag {
                    reactor.insert((x, y, z));
                } else {
                    reactor.remove(&(x, y, z));
                }
            }
        }
        let mut count = 0;
        for (x, y, z) in iproduct!(-50..=50, -50..=50, -50..=50) {
            if reactor.contains(&(x, y, z)) {
                count += 1;
            }
        }
        (instructions, count.to_string())
    }

    fn solve_part2(mut instructions: Self::ProcessedInput) -> String {
        let mut reactor = Vec::<Cuboid>::new();
        instructions.reverse();
        'outer: while let Some((flag, cuboid)) = instructions.pop() {
            for i in 0..reactor.len() {
                let other = reactor[i].clone();
                if split_up(&other, &cuboid, &mut reactor, &mut instructions, flag) {
                    reactor.swap_remove(i);
                    continue 'outer;
                }
            }
            if flag {
                reactor.push(cuboid);
            }
        }
        let length = |r: R| (1 + r.end() - r.start()) as u64;
        let volume = |c: Cuboid| length(c.0) * length(c.1) * length(c.2);
        reactor.into_iter().map(volume).sum::<u64>().to_string()
    }
}
