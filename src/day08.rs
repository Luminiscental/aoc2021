use super::{
    day::Day,
    util::{self, CollectArray},
};

pub struct Day08;

impl<'a> Day<'a> for Day08 {
    type Input = Vec<[&'a str; 14]>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 8;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| line.split(" | ").flat_map(|s| s.split(' ')).collect_array())
            .collect()
    }

    fn solve_part1(displays: Self::Input) -> (Self::ProcessedInput, String) {
        let occurences = displays
            .iter()
            .flat_map(|display| display[10..].iter())
            .filter(|s| [2, 3, 4, 7].contains(&s.len()))
            .count();
        (displays, occurences.to_string())
    }

    fn solve_part2(displays: Self::ProcessedInput) -> String {
        let pack = |s: &str| s.chars().map(|c| 1 << (c as usize - 'a' as usize)).sum();
        let find_nsegments = |slice: &[&str], n| pack(slice.iter().find(|s| s.len() == n).unwrap());
        let decode = |s, one: usize, four: usize| {
            let d: usize = pack(s);
            match (s.len(), (d & one).count_ones(), (d & four).count_ones()) {
                (2, _, _) => 1,
                (3, _, _) => 7,
                (4, _, _) => 4,
                (5, 1, 2) => 2,
                (5, 1, 3) => 5,
                (5, 2, _) => 3,
                (6, 1, _) => 6,
                (6, 2, 3) => 0,
                (6, 2, 4) => 9,
                (7, _, _) => 8,
                _ => panic!(),
            }
        };
        displays
            .iter()
            .map(|display| {
                let one = find_nsegments(&display[..10], 2);
                let four = find_nsegments(&display[..10], 4);
                util::unradix(display[10..].iter().map(|s| decode(s, one, four)).rev(), 10)
            })
            .sum::<usize>()
            .to_string()
    }
}

bench_day!(08);
