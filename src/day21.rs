use crate::day::Day;
use hashbrown::HashMap;
use itertools::Itertools;
use std::{hash::Hash, ops::AddAssign};

const DIRAC_ROLL_SUMS: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State<S: Copy + Eq + Hash> {
    positions: [u8; 2],
    scores: [S; 2],
    player: u8,
}

fn play<S: Copy + Eq + Hash + Ord + AddAssign + From<u8>>(
    roll_sum: u8,
    win: S,
    mut state: State<S>,
) -> Result<State<S>, S> {
    state.positions[state.player as usize] += roll_sum;
    state.positions[state.player as usize] %= 10;
    state.scores[state.player as usize] += (1 + state.positions[state.player as usize]).into();
    if state.scores[state.player as usize] >= win {
        Err(state.scores[(1 - state.player) as usize])
    } else {
        state.player = 1 - state.player;
        Ok(state)
    }
}

fn dirac_wins_from(state: State<u8>) -> [u64; 2] {
    fn dirac_wins_from_memo(state: State<u8>, memo: &mut HashMap<State<u8>, [u64; 2]>) -> [u64; 2] {
        match memo.get(&state) {
            Some(&wins) => wins,
            None => {
                let mut wins = [0, 0];
                for &(roll_sum, universes) in DIRAC_ROLL_SUMS.iter() {
                    let then_wins = match play(roll_sum, 21, state) {
                        Ok(state) => dirac_wins_from_memo(state, memo),
                        Err(_) => [(1 - state.player) as u64, state.player as u64],
                    };
                    wins[0] += universes * then_wins[0];
                    wins[1] += universes * then_wins[1];
                }
                memo.insert(state, wins);
                wins
            }
        }
    }
    dirac_wins_from_memo(state, &mut HashMap::new())
}

pub struct Day21;

impl<'a> Day<'a> for Day21 {
    type Input = [u8; 2];
    type ProcessedInput = Self::Input;

    const DAY: usize = 21;

    fn parse(input: &'a str) -> Self::Input {
        let (p1, p2) = input.lines().next_tuple().unwrap();
        let (p1, p2): (u8, u8) = (p1[28..].parse().unwrap(), p2[28..].parse().unwrap());
        [p1 - 1, p2 - 1]
    }

    fn solve_part1(positions: Self::Input) -> (Self::ProcessedInput, String) {
        let mut state = State {
            positions,
            scores: [0, 0],
            player: 0,
        };
        let roll_sums = (6..).step_by(9).map(|n| (n % 10) as u8);
        let roll_counts = (3..).step_by(3);
        for (roll_count, roll_sum) in roll_counts.zip(roll_sums) {
            match play(roll_sum, 1000u32, state) {
                Ok(next_state) => state = next_state,
                Err(score) => return (positions, (roll_count * score).to_string()),
            }
        }
        unreachable!()
    }

    fn solve_part2(positions: Self::ProcessedInput) -> String {
        dirac_wins_from(State {
            positions,
            scores: [0, 0],
            player: 0,
        })
        .iter()
        .max()
        .unwrap()
        .to_string()
    }
}

#[cfg(test)]
mod test_day21 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Player 1 starting position: 4
        Player 2 starting position: 8
    "};

    #[test]
    fn test_day21_examples() {
        let input = Day21::parse(EXAMPLE);
        let (input, part1) = Day21::solve_part1(input);
        assert_eq!(part1, "739785");
        let part2 = Day21::solve_part2(input);
        assert_eq!(part2, "444356092776315");
    }
}

bench_day!(21);
