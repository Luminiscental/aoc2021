use super::day::Day;
use itertools::Itertools;
use std::{collections::HashMap, mem};

type Pair = (u8, u8);

pub struct Input {
    first_elmt: u8,
    last_elmt: u8,
    polymer: HashMap<Pair, usize>,
    rules: HashMap<Pair, u8>,
}

fn reinforce(polymer: &mut HashMap<Pair, usize>, rules: &HashMap<Pair, u8>) {
    for (pair, count) in mem::take(polymer).into_iter() {
        *polymer.entry((pair.0, rules[&pair])).or_insert(0) += count;
        *polymer.entry((rules[&pair], pair.1)).or_insert(0) += count;
    }
}

fn diversity(first_elmt: u8, last_elmt: u8, polymer: &HashMap<Pair, usize>) -> usize {
    let mut counts = HashMap::new();
    for (pair, count) in polymer.iter() {
        *counts.entry(pair.0).or_insert(0) += count;
        *counts.entry(pair.1).or_insert(0) += count;
    }
    *counts.entry(first_elmt).or_insert(0) += 1;
    *counts.entry(last_elmt).or_insert(0) += 1;
    counts.values_mut().for_each(|n| *n /= 2);
    let (min, max) = counts.into_values().minmax().into_option().unwrap();
    max - min
}

pub struct Day14;

impl<'a> Day<'a> for Day14 {
    type Input = Input;
    type ProcessedInput = Self::Input;

    const DAY: usize = 14;

    fn parse(input: &'a str) -> Self::Input {
        let (template, rules) = input.split("\n\n").next_tuple().unwrap();
        Input {
            first_elmt: template.bytes().next().unwrap(),
            last_elmt: template.bytes().last().unwrap(),
            polymer: template.bytes().tuple_windows().counts(),
            rules: rules
                .lines()
                .map(|line| line.split(" -> ").next_tuple().unwrap())
                .map(|(pair, elmt)| ((pair.as_bytes()[0], pair.as_bytes()[1]), elmt.as_bytes()[0]))
                .collect(),
        }
    }

    fn solve_part1(mut input: Self::Input) -> (Self::ProcessedInput, String) {
        (0..10).for_each(|_| reinforce(&mut input.polymer, &input.rules));
        let ans = diversity(input.first_elmt, input.last_elmt, &input.polymer);
        (input, ans.to_string())
    }

    fn solve_part2(mut input: Self::ProcessedInput) -> String {
        (0..30).for_each(|_| reinforce(&mut input.polymer, &input.rules));
        diversity(input.first_elmt, input.last_elmt, &input.polymer).to_string()
    }
}
