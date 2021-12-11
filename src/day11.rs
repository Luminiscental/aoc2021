use super::day::Day;
use std::collections::BTreeSet;

fn for_adjacents<F: FnMut(usize)>(i: usize, mut f: F) {
    macro_rules! for_each {
        ($($xs:expr),+) => {{
            $(f((i as i32 + $xs) as usize));+
        }}
    }
    match (i / 10, i % 10) {
        (0, 0) => for_each!(1, 10, 11),
        (0, 9) => for_each!(-1, 10, 9),
        (9, 0) => for_each!(1, -10, -9),
        (9, 9) => for_each!(-1, -10, -11),
        (0, _) => for_each!(1, -1, 10, 11, 9),
        (9, _) => for_each!(1, -1, -10, -11, -9),
        (_, 0) => for_each!(1, -10, -9, 10, 11),
        (_, 9) => for_each!(-1, -10, -11, 10, 9),
        (_, _) => for_each!(1, -1, 10, -10, 9, -9, 11, -11),
    }
}

fn step(energy_levels: &mut Vec<u32>) -> usize {
    let mut flashed = BTreeSet::new();
    energy_levels.iter_mut().for_each(|o| *o += 1);
    let mut to_flash: BTreeSet<_> = (0..100).filter(|&i| energy_levels[i] > 9).collect();
    while let Some(flasher) = to_flash.pop_last() {
        flashed.insert(flasher);
        for_adjacents(flasher, |i| {
            energy_levels[i] += 1;
            if energy_levels[i] > 9 && !flashed.contains(&i) {
                to_flash.insert(i);
            }
        });
    }
    flashed.iter().for_each(|&i| energy_levels[i] = 0);
    flashed.len()
}

pub struct Day11;

impl<'a> Day<'a> for Day11 {
    type Input = Vec<u32>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 11;

    fn parse(input: &'a str) -> Self::Input {
        input.chars().filter_map(|c| c.to_digit(10)).collect()
    }

    fn solve_part1(mut energy_levels: Self::Input) -> (Self::ProcessedInput, String) {
        let flashes = (0..100).map(|_| step(&mut energy_levels)).sum::<usize>();
        (energy_levels, flashes.to_string())
    }

    fn solve_part2(mut energy_levels: Self::ProcessedInput) -> String {
        (101..)
            .find(|_| step(&mut energy_levels) == 100)
            .unwrap()
            .to_string()
    }
}
