use crate::{
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
        let pack = |s: &str| s.chars().map(|c| 1 << (c as u32 - 'a' as u32)).sum();
        let find_nsegments = |slice: &[&str], n| pack(slice.iter().find(|s| s.len() == n).unwrap());
        let decode = |s, one: u32, four: u32| {
            let d: u32 = pack(s);
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
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day08 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn test_day08_examples() {
        let input = Day08::parse(EXAMPLE);
        let (input, part1) = Day08::solve_part1(input);
        let part2 = Day08::solve_part2(input);
        assert_eq!(part1, "26");
        assert_eq!(part2, "61229");
    }
}

bench_day!(08);
