use super::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

const START: [u8; 2] = [0, 0];
const END: [u8; 2] = [!0, !0];

fn count_paths(edge_map: &HashMap<[u8; 2], Vec<[u8; 2]>>, allow_dups: bool) -> usize {
    let mut count = 0;
    let mut queue = VecDeque::new();
    queue.push_front((Vec::new(), allow_dups));
    while let Some(path) = queue.pop_back() {
        for &cave in edge_map[path.0.last().unwrap_or(&START)].iter() {
            if cave == END {
                count += 1;
            } else if cave[0].is_ascii_uppercase() || !path.0.contains(&cave) {
                queue.push_front(([path.0.clone(), vec![cave]].concat(), path.1));
            } else if path.1 {
                queue.push_front(([path.0.clone(), vec![cave]].concat(), false));
            }
        }
    }
    count
}

pub struct Day12;

impl<'a> Day<'a> for Day12 {
    type Input = HashMap<[u8; 2], Vec<[u8; 2]>>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 12;

    fn parse(input: &'a str) -> Self::Input {
        let mut map = HashMap::new();
        let mut add_edge = |s, e| {
            if e != START {
                map.entry(s).or_insert_with(Vec::new).push(e)
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
        map
    }

    fn solve_part1(edge_map: Self::Input) -> (Self::ProcessedInput, String) {
        let count = count_paths(&edge_map, false);
        (edge_map, count.to_string())
    }

    fn solve_part2(edge_map: Self::ProcessedInput) -> String {
        count_paths(&edge_map, true).to_string()
    }
}
