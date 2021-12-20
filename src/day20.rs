use crate::{day::Day, util::BitSet};
use hashbrown::HashSet;
use itertools::Itertools;

const KERNEL: [((i32, i32), u16); 9] = [
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

pub struct Image {
    foreground: HashSet<(i32, i32)>,
    background: bool,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Image {
    fn new(foreground: HashSet<(i32, i32)>, background: bool) -> Self {
        let (xmin, xmax) = foreground
            .iter()
            .map(|p| p.0)
            .minmax()
            .into_option()
            .unwrap();
        let (ymin, ymax) = foreground
            .iter()
            .map(|p| p.1)
            .minmax()
            .into_option()
            .unwrap();
        Self {
            foreground,
            background,
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    fn enhance(&mut self, table: &[bool], buffer: &mut BitSet) {
        let (xmin, xmax) = (self.xmin - 2, self.xmax + 2);
        let (ymin, ymax) = (self.ymin - 2, self.ymax + 2);
        let width = xmax - xmin;
        let height = ymax - ymin;
        buffer.clear();
        buffer.reserve(((width + 1) * (height + 1)) as usize);
        for j in 0..=height {
            for i in 0..=width {
                if self.background != self.foreground.contains(&(xmin + i, ymin + j)) {
                    buffer.insert((i + j * width) as u32);
                }
            }
        }
        let mut new_foreground = HashSet::with_capacity(self.foreground.len());
        let new_background = table[self.background as usize * 511];
        for j in 1..=height - 1 {
            for i in 1..=width - 1 {
                let mut lookup = 0;
                for ((dx, dy), bit) in KERNEL.iter() {
                    lookup |= bit
                        * unsafe { buffer.contains_unchecked((i + dx + (j + dy) * width) as u32) }
                            as u16;
                }
                if table[lookup as usize] != new_background {
                    let (x, y) = (xmin + i, ymin + j);
                    new_foreground.insert((x, y));
                    self.xmin = self.xmin.min(x);
                    self.xmax = self.xmax.max(x);
                    self.ymin = self.ymin.min(y);
                    self.ymax = self.ymax.max(y);
                }
            }
        }
        self.foreground = new_foreground;
        self.background = new_background;
    }
}

pub struct Day20;

impl<'a> Day<'a> for Day20 {
    type Input = (Vec<bool>, Image);
    type ProcessedInput = Self::Input;

    const DAY: usize = 20;

    fn parse(input: &'a str) -> Self::Input {
        let (table, image) = input.split("\n\n").next_tuple().unwrap();
        let table = table
            .trim()
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| c == '#')
            .collect();
        let image = image
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (c == '#').then(|| (x as i32, y as i32)))
                    .flatten()
            })
            .collect();
        (table, Image::new(image, false))
    }

    fn solve_part1((table, mut image): Self::Input) -> (Self::ProcessedInput, String) {
        let mut buffer = BitSet::new();
        (0..2).for_each(|_| image.enhance(&table, &mut buffer));
        let ans = image.foreground.len();
        ((table, image), ans.to_string())
    }

    fn solve_part2((table, mut image): Self::ProcessedInput) -> String {
        let mut buffer = BitSet::new();
        (0..48).for_each(|_| image.enhance(&table, &mut buffer));
        image.foreground.len().to_string()
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
