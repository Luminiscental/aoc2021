use crate::day::Day;
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

#[cfg(test)]
mod test_day12 {
    use super::*;
    use indoc::indoc;

    // modified to have double character caves
    const SMALL_EXAMPLE: &str = indoc! {"
        start-AA
        start-bb
        AA-cc
        AA-bb
        bb-dd
        AA-end
        bb-end
    "};

    const MEDIUM_EXAMPLE: &str = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "};

    const LARGE_EXAMPLE: &str = indoc! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    "};

    fn assert_example(example: &str, output1: &str, output2: &str) {
        let input = Day12::parse(example);
        let (input, part1) = Day12::solve_part1(input);
        let part2 = Day12::solve_part2(input);
        assert_eq!(part1, output1);
        assert_eq!(part2, output2);
    }

    #[test]
    fn test_day12_examples() {
        assert_example(SMALL_EXAMPLE, "10", "36");
        assert_example(MEDIUM_EXAMPLE, "19", "103");
        assert_example(LARGE_EXAMPLE, "226", "3509");
    }
}

bench_day!(12);
