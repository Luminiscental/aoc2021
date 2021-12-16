use crate::{day::Day, util::CastValues};
use itertools::Itertools;
use std::{collections::HashMap, iter, mem};

type Pair = (u8, u8);

fn reinforce(polymer: &mut HashMap<Pair, u64>, rules: &HashMap<Pair, u8>) {
    for (pair, count) in mem::take(polymer).into_iter() {
        *polymer.entry((pair.0, rules[&pair])).or_insert(0) += count;
        *polymer.entry((rules[&pair], pair.1)).or_insert(0) += count;
    }
}

fn diversity(last: u8, polymer: &HashMap<Pair, u64>) -> u64 {
    let mut counts = HashMap::new();
    for (pair, count) in polymer.iter().chain(iter::once((&(last, 0), &1))) {
        *counts.entry(pair.0).or_insert(0) += count;
    }
    let (min, max) = counts.into_values().minmax().into_option().unwrap();
    max - min
}

pub struct Day14;

impl<'a> Day<'a> for Day14 {
    type Input = (u8, HashMap<Pair, u64>, HashMap<Pair, u8>);
    type ProcessedInput = Self::Input;

    const DAY: usize = 14;

    fn parse(input: &'a str) -> Self::Input {
        let (template, rules) = input.split("\n\n").next_tuple().unwrap();
        (
            template.bytes().last().unwrap(),
            template.bytes().tuple_windows().counts().cast_values(),
            rules
                .lines()
                .map(|line| line.split(" -> ").next_tuple().unwrap())
                .map(|(pair, elmt)| ((pair.as_bytes()[0], pair.as_bytes()[1]), elmt.as_bytes()[0]))
                .collect(),
        )
    }

    fn solve_part1((last, mut polymer, rules): Self::Input) -> (Self::ProcessedInput, String) {
        (0..10).for_each(|_| reinforce(&mut polymer, &rules));
        let ans = diversity(last, &polymer);
        ((last, polymer, rules), ans.to_string())
    }

    fn solve_part2((last, mut polymer, rules): Self::ProcessedInput) -> String {
        (0..30).for_each(|_| reinforce(&mut polymer, &rules));
        diversity(last, &polymer).to_string()
    }
}

#[cfg(test)]
mod test_day14 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn test_day14_examples() {
        let input = Day14::parse(EXAMPLE);
        let (input, part1) = Day14::solve_part1(input);
        let part2 = Day14::solve_part2(input);
        assert_eq!(part1, "1588");
        assert_eq!(part2, "2188189693529");
    }
}

bench_day!(14);
