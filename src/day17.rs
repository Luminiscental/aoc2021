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
        // assumes xrange contains a triangular number
        let max_vel = -yrange.start() - 1;
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

/*
 * How observe_preimages works:
 *
 *   The problem is essentially asking for the set of velocities (v_x, v_y) such
 *   that the trajectory enters the target at some step. In both directions, our
 *   velocity decreases by 1 each step, but in the x direction this stops at 0,
 *   whereas in the y direction it continues indefinitely.
 *
 *   For n=1,2,3,... define f_n(v) = v + (v-1) + (v-2) + ... + (v-n+1) which is
 *   the distance travelled after n steps starting with velocity v (in the y
 *   direction at least). Then define a modified function g_n(v) by
 *   g_n(v) = f_{max(v, n)}(v), which models the stopping that happens in the x
 *   direction; if we take more than v steps we still get f_v(v).
 *
 *   Then (g_n(v_x), f_n(v_y)) gives the point reached after n steps with
 *   initial velocities (v_x, v_y), so the set of velocities which eventually
 *   hit the target is the union over n of the preimages of the target region
 *   under (g_n, f_n).
 *
 *   Now there are two possible ways for (g_n(v_x), f_n(v_y)) to equal a given
 *   point (x, y). If n < v_x, then g_n(v_x) = f_n(v_x), so (x, y) is the same
 *   image under the simpler function (f_n, f_n), which is easily dealt with
 *   since f_n is an invertible linear function (so we can just apply it to the
 *   endpoints of the target ranges in x and y):
 *
 *       f_n(v) = v + (v-1) + (v-2) + ... + (v-n+1)
 *              = n*v - (1 + 2 + ... + (n-1))
 *              = n*v - n*(n-1)/2
 *
 *       f_n^{-1}(x) = x/n + (n-1)/2
 *
 *   Otherwise, n >= v_x so g_n(v_x) = g_{v_x}(v_x) is a triangular number (if
 *   k = v_x, then the kth triangular number t(k) = k + (k-1) + (k-2) + ...
 *   + 2 + 1 = g_k(k)). Hence the other preimages will correspond to triangular
 *   numbers within the x range of our target region (in particular there are
 *   not likely to be many of these edge case x values). So fix some velocity
 *   v_x corresponding to one of the triangular numbers g_{v_k}(v_k) inside the
 *   target x range. We simply want to find the velocities v_y such that
 *   f_m(v_y) is in the target y range for some m >= v_k, since in that case
 *   g_m(v_k) = g_{v_k}(v_k) is in the target x range by assumption. For
 *   f_m(v_y) to lie in the target range, we need v_y to be at least
 *   f_m^{-1}(target_y_min) since f_m(x) is an increasing function of x, and
 *   hence at least f_{v_k}^{-1}(target_y_min) since f_m(x) is an increasing
 *   function of m also, and we're assuming m >= v_k. This gives a lower bound
 *   on the values of v_y that need checking. To get the upper bound, notice
 *   that if v_y is larger than the furthest extent of the target region from
 *   y=0, then we are guaranteed to miss the entire target when we fall down,
 *   since we reach y=0 in correspondence with the initial state, and then take
 *   a step of -v_y. Hence we can iterate through this range of values of v_y,
 *   and for each value check whether it hits the target by solving the
 *   quadratic equations f_m(v_y) = target_y_min and f_m(v_y) = target_y_max for
 *   m, and checking whether an integer value of m lies inbetween those roots.
 */
