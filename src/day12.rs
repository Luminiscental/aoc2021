use super::day::Day;
use itertools::Itertools;
use std::collections::HashMap;

type Cave = [u8; 2];

const START: Cave = [0, 0];
const END: Cave = [!0, !0];

fn calculate_weights(edges: HashMap<Cave, Vec<Cave>>) -> HashMap<Cave, HashMap<Cave, usize>> {
    let mut weighted_edges = HashMap::new();
    let mut add_edge = |start, end| {
        *weighted_edges
            .entry(start)
            .or_insert_with(HashMap::new)
            .entry(end)
            .or_insert(0) += 1
    };
    for &small_cave in edges
        .keys()
        .filter(|&&cave| cave == START || cave[0].is_ascii_lowercase())
    {
        for &end in edges[&small_cave].iter() {
            if end[0].is_ascii_uppercase() {
                for &next_small_cave in edges[&end].iter() {
                    add_edge(small_cave, next_small_cave);
                }
            } else {
                add_edge(small_cave, end);
            }
        }
    }
    weighted_edges
}

fn completions(
    path: Vec<Cave>,
    small_edges: &HashMap<Cave, HashMap<Cave, usize>>,
    allow_dups: bool,
) -> usize {
    small_edges[path.last().unwrap_or(&START)]
        .iter()
        .map(|(&cave, &weight)| {
            if cave == END {
                weight
            } else if !path.contains(&cave) {
                weight * completions([path.clone(), vec![cave]].concat(), small_edges, allow_dups)
            } else if allow_dups {
                weight * completions([path.clone(), vec![cave]].concat(), small_edges, false)
            } else {
                0
            }
        })
        .sum()
}

pub struct Day12;

impl<'a> Day<'a> for Day12 {
    type Input = HashMap<Cave, HashMap<Cave, usize>>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 12;

    fn parse(input: &'a str) -> Self::Input {
        let mut edges = HashMap::new();
        let mut add_edge = |s, e| {
            if e != START {
                edges.entry(s).or_insert_with(Vec::new).push(e)
            }
        };
        for line in input.lines() {
            let (a, b) = line
                .split('-')
                .map(|cave| match cave {
                    "start" => START,
                    "end" => END,
                    _ => [cave.as_bytes()[0], cave.as_bytes()[1]],
                })
                .next_tuple()
                .unwrap();
            add_edge(a, b);
            add_edge(b, a);
        }
        calculate_weights(edges)
    }

    fn solve_part1(small_edges: Self::Input) -> (Self::ProcessedInput, String) {
        let count = completions(Vec::new(), &small_edges, false);
        (small_edges, count.to_string())
    }

    fn solve_part2(small_edges: Self::ProcessedInput) -> String {
        completions(Vec::new(), &small_edges, true).to_string()
    }
}
