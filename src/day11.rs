use super::day::Day;

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

fn flash(flasher: usize, energy_levels: &mut [u32]) -> usize {
    let mut flashes = 1;
    energy_levels[flasher] = u32::MAX;
    for_adjacents(flasher, |i| {
        if energy_levels[i] != u32::MAX {
            energy_levels[i] += 1;
            if energy_levels[i] > 8 {
                flashes += flash(i, energy_levels);
            }
        }
    });
    flashes
}

fn step(energy_levels: &mut [u32]) -> usize {
    let mut flashes = 0;
    for i in 0..100 {
        if energy_levels[i] == 9 {
            flashes += flash(i, energy_levels);
        }
    }
    energy_levels
        .iter_mut()
        .for_each(|o| *o = o.wrapping_add(1));
    flashes
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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    fn test_day11_examples() {
        let input = Day11::parse(EXAMPLE);
        let (input, part1) = Day11::solve_part1(input);
        let part2 = Day11::solve_part2(input);
        assert_eq!(part1, "1656");
        assert_eq!(part2, "195");
    }
}

bench_day!(11);
