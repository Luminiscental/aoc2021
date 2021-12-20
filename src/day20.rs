use crate::{day::Day, util};
use hashbrown::HashSet;
use itertools::Itertools;

const KERNEL: [(i32, i32); 9] = [
    (1, 1),
    (0, 1),
    (-1, 1),
    (1, 0),
    (0, 0),
    (-1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
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

    fn enhance(&mut self, table: &[bool]) {
        let mut new_foreground = HashSet::with_capacity(self.foreground.len());
        let new_background = table[if self.background { 511 } else { 0 }];
        let yrange = self.ymin - 1..=self.ymax + 1;
        let xrange = self.xmin - 1..=self.xmax + 1;
        for y in yrange {
            for x in xrange.clone() {
                let lit = |p| self.background != self.foreground.contains(&p);
                let lookup = util::unradix(
                    KERNEL.iter().map(|(dx, dy)| lit((x + dx, y + dy)).into()),
                    2,
                );
                if table[lookup as usize] != new_background {
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
        (0..2).for_each(|_| image.enhance(&table));
        let ans = image.foreground.len();
        ((table, image), ans.to_string())
    }

    fn solve_part2((table, mut image): Self::ProcessedInput) -> String {
        (0..48).for_each(|_| image.enhance(&table));
        image.foreground.len().to_string()
    }
}
