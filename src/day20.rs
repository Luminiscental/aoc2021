use crate::day::Day;
use itertools::Itertools;

const KERNEL: [((i32, i32), i32); 9] = [
    ((1, 1), 1 << 0),
    ((0, 1), 1 << 1),
    ((-1, 1), 1 << 2),
    ((1, 0), 1 << 3),
    ((0, 0), 1 << 4),
    ((-1, 0), 1 << 5),
    ((1, -1), 1 << 6),
    ((0, -1), 1 << 7),
    ((-1, -1), 1 << 8),
];

macro_rules! subkernel {
    ($($indices:literal),+) => {
        (&[$(KERNEL[$indices]),+], 511 - ($((1 << $indices) | )+ 0))
    }
}

pub struct Image {
    foreground: Vec<bool>,
    background: bool,
    size: i32,
}

impl Image {
    fn enhance(&mut self, algorithm: &[bool]) {
        let new_size = self.size + 2;
        let new_background = algorithm[self.background as usize * 511];
        let mut new_foreground = vec![false; (new_size * new_size) as usize];
        let mut convolve = |x, y, (ker, offset): (&[_], _)| {
            let mut lookup = offset * self.background as i32;
            let idx = x - 1 + (y - 1) * self.size;
            for &((dx, dy), b) in ker.iter() {
                let fg = self.foreground[(idx + dx + dy * self.size) as usize];
                lookup |= b * (fg != self.background) as i32;
            }
            new_foreground[(x + y * new_size) as usize] =
                new_background != algorithm[lookup as usize];
        };

        convolve(0, 0, subkernel!(0));
        convolve(1, 0, subkernel!(0, 1));
        for i in 2..new_size - 2 {
            convolve(i, 0, subkernel!(0, 1, 2));
        }
        convolve(new_size - 2, 0, subkernel!(2, 1));
        convolve(new_size - 1, 0, subkernel!(2));
        convolve(0, 1, subkernel!(0, 3));
        convolve(1, 1, subkernel!(0, 1, 3, 4));
        for i in 2..new_size - 2 {
            convolve(i, 1, subkernel!(0, 1, 2, 3, 4, 5));
        }
        convolve(new_size - 2, 1, subkernel!(1, 2, 4, 5));
        convolve(new_size - 1, 1, subkernel!(2, 5));

        for y in 2..new_size - 2 {
            convolve(0, y, subkernel!(0, 3, 6));
            convolve(1, y, subkernel!(0, 1, 3, 4, 6, 7));
            for x in 2..new_size - 2 {
                convolve(x, y, subkernel!(0, 1, 2, 3, 4, 5, 6, 7, 8));
            }
            convolve(new_size - 2, y, subkernel!(1, 2, 4, 5, 7, 8));
            convolve(new_size - 1, y, subkernel!(2, 5, 8));
        }

        convolve(0, new_size - 2, subkernel!(6, 3));
        convolve(1, new_size - 2, subkernel!(3, 4, 6, 7));
        for i in 2..new_size - 2 {
            convolve(i, new_size - 2, subkernel!(3, 4, 5, 6, 7, 8));
        }
        convolve(new_size - 2, new_size - 2, subkernel!(4, 5, 7, 8));
        convolve(new_size - 1, new_size - 2, subkernel!(8, 5));
        convolve(0, new_size - 1, subkernel!(6));
        convolve(1, new_size - 1, subkernel!(6, 7));
        for i in 2..new_size - 2 {
            convolve(i, new_size - 1, subkernel!(6, 7, 8));
        }
        convolve(new_size - 2, new_size - 1, subkernel!(8, 7));
        convolve(new_size - 1, new_size - 1, subkernel!(8));

        self.foreground = new_foreground;
        self.background = new_background;
        self.size = new_size;
    }
}

pub struct Day20;

impl<'a> Day<'a> for Day20 {
    type Input = (Vec<bool>, Image);
    type ProcessedInput = Self::Input;

    const DAY: usize = 20;

    fn parse(input: &'a str) -> Self::Input {
        let (algorithm, image) = input.split("\n\n").next_tuple().unwrap();
        let algorithm = algorithm
            .trim()
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| c == '#')
            .collect();
        let mut size = 0;
        let image = image
            .trim()
            .lines()
            .flat_map(|line| {
                size = line.len() as i32;
                line.chars().map(|c| c == '#')
            })
            .collect();
        (
            algorithm,
            Image {
                foreground: image,
                background: false,
                size,
            },
        )
    }

    fn solve_part1((algorithm, mut image): Self::Input) -> (Self::ProcessedInput, String) {
        (0..2).for_each(|_| image.enhance(&algorithm));
        let ans = image.foreground.iter().filter(|&&b| b).count();
        ((algorithm, image), ans.to_string())
    }

    fn solve_part2((algorithm, mut image): Self::ProcessedInput) -> String {
        (0..48).for_each(|_| image.enhance(&algorithm));
        image
            .foreground
            .into_iter()
            .filter(|&b| b)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test_day20 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
    "};

    #[test]
    fn test_day20_examples() {
        let input = Day20::parse(EXAMPLE);
        let (input, part1) = Day20::solve_part1(input);
        assert_eq!(part1, "35");
        let part2 = Day20::solve_part2(input);
        assert_eq!(part2, "3351");
    }
}

bench_day!(20);
