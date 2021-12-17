use crate::{day::Day, util::BitSet};
use itertools::{iproduct, Itertools};
use std::ops::RangeInclusive;

type R = RangeInclusive<i32>;

fn observe_preimages<O: FnMut(R, R)>(xrange: &R, yrange: &R, mut observer: O) {
    let (xstart, xend) = (*xrange.start(), *xrange.end());
    let (ystart, yend) = (*yrange.start(), *yrange.end());
    let t_inv = |x| ((-1.0 + (1.0 + 8.0 * x as f32).sqrt()) / 2.0);
    (1..t_inv(xstart).ceil() as i32).for_each(|steps| {
        let f_inv = |y| y as f32 / steps as f32 + (steps - 1) as f32 / 2.0;
        observer(
            f_inv(xstart).ceil() as i32..=f_inv(xend).floor() as i32,
            f_inv(ystart).ceil() as i32..=f_inv(yend).floor() as i32,
        );
    });
    (t_inv(xstart).ceil() as i32..=t_inv(xend).floor() as i32).for_each(|steps| {
        let f_inv = |y| y as f32 / steps as f32 + (steps - 1) as f32 / 2.0;
        for v in f_inv(ystart).ceil() as i32..=-ystart {
            let enter_disc = (((2 * v + 1) * (2 * v + 1) - 8 * yend) as f32).sqrt();
            let exit_disc = (((2 * v + 1) * (2 * v + 1) - 8 * ystart) as f32).sqrt();
            if exit_disc - enter_disc >= 2.0
                || ((1.0 + enter_disc) / 2.0).ceil() == ((1.0 + exit_disc) / 2.0).floor()
            {
                observer(steps..=steps, v..=v);
            }
        }
    });
}

pub struct Day17;

impl<'a> Day<'a> for Day17 {
    type Input = (R, R);
    type ProcessedInput = (R, R);

    const DAY: usize = 17;

    fn parse(input: &'a str) -> Self::Input {
        let (xrange, yrange) = input.trim()[15..].split(", y=").next_tuple().unwrap();
        let (xmin, xmax) = xrange.split("..").next_tuple().unwrap();
        let (ymin, ymax) = yrange.split("..").next_tuple().unwrap();
        (
            xmin.parse().unwrap()..=xmax.parse().unwrap(),
            ymin.parse().unwrap()..=ymax.parse().unwrap(),
        )
    }

    fn solve_part1((xrange, yrange): Self::Input) -> (Self::ProcessedInput, String) {
        assert!(*xrange.start() > 0 && *yrange.end() < 0);
        let mut max_vel = 0;
        observe_preimages(&xrange, &yrange, |_, ry| max_vel = max_vel.max(*ry.end()));
        let ans = max_vel * (max_vel + 1) / 2;
        ((xrange, yrange), ans.to_string())
    }

    fn solve_part2((xrange, yrange): Self::ProcessedInput) -> String {
        let mut vels = BitSet::new();
        let pack = |x, y| (x + (y - yrange.start()) * xrange.end()) as u32;
        // I would love to sum up areas here, but sadly the preimages overlap...
        observe_preimages(&xrange, &yrange, |rx, ry| {
            iproduct!(rx, ry).for_each(|(x, y)| vels.insert(pack(x, y)))
        });
        vels.len().to_string()
    }
}

#[cfg(test)]
mod test_day17 {
    use super::*;

    #[test]
    fn test_day17_examples() {
        let input = Day17::parse("target area: x=20..30, y=-10..-5");
        let (input, part1) = Day17::solve_part1(input);
        assert_eq!(part1, "45");
        let part2 = Day17::solve_part2(input);
        assert_eq!(part2, "112");
    }
}

bench_day!(17);
