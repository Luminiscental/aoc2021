use crate::day::Day;
use hashbrown::HashMap;
use itertools::Itertools;
use std::{hash::Hash, ops::AddAssign};

const DIRAC_ROLL_SUMS: [(u8, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

trait State {
    type Score;

    fn with_positions(positions: [u8; 2]) -> Self;
    fn player(&self) -> u8;
    fn swap_players(&mut self);
    fn score(&self, player: u8) -> Self::Score;
    fn set_score(&mut self, player: u8, score: Self::Score);
    fn position(&self, player: u8) -> u8;
    fn set_position(&mut self, player: u8, position: u8);
}

fn play<S: State>(
    roll_sum: u8,
    win: <S as State>::Score,
    mut state: S,
) -> Result<S, <S as State>::Score>
where
    <S as State>::Score: Copy + Ord + AddAssign + From<u8>,
{
    let player = state.player();
    let mut pos = state.position(player);
    pos += roll_sum;
    pos %= 10;
    let mut score = state.score(player);
    score += (1 + pos).into();
    state.set_position(player, pos);
    state.set_score(player, score);
    state.swap_players();
    if score >= win {
        Err(state.score(state.player()))
    } else {
        Ok(state)
    }
}

fn dirac_wins_from<S: Copy + Eq + Hash + State>(state: S) -> [u64; 2]
where
    <S as State>::Score: Copy + Ord + AddAssign + From<u8>,
{
    fn dirac_wins_from_memo<S: Copy + Eq + Hash + State>(
        state: S,
        memo: &mut HashMap<S, [u64; 2]>,
    ) -> [u64; 2]
    where
        <S as State>::Score: Copy + Ord + AddAssign + From<u8>,
    {
        match memo.get(&state) {
            Some(&wins) => wins,
            None => {
                let mut wins = [0, 0];
                for &(roll_sum, universes) in DIRAC_ROLL_SUMS.iter() {
                    let then_wins = match play(roll_sum, 21.into(), state) {
                        Ok(state) => dirac_wins_from_memo(state, memo),
                        Err(_) => [(1 - state.player()) as u64, state.player() as u64],
                    };
                    wins[0] += universes * then_wins[0];
                    wins[1] += universes * then_wins[1];
                }
                memo.insert(state, wins);
                wins
            }
        }
    }
    // the total number of possible states is 2*10*10*21*21 = 88200, and any
    // pair of starting positions reaches between 20000 to 30000 of these.
    dirac_wins_from_memo(state, &mut HashMap::with_capacity(30000))
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
        let mut state = u64::with_positions(positions);
        let roll_sums = (0..10).rev().cycle().skip(3);
        let roll_counts = (3..).step_by(3);
        for (roll_count, roll_sum) in roll_counts.zip(roll_sums) {
            match play(roll_sum, 1000, state) {
                Ok(next_state) => state = next_state,
                Err(score) => return (positions, (roll_count * score as u32).to_string()),
            }
        }
        unreachable!()
    }

    fn solve_part2(positions: Self::ProcessedInput) -> String {
        dirac_wins_from(u32::with_positions(positions))
            .iter()
            .max()
            .unwrap()
            .to_string()
    }
}

impl State for u64 {
    type Score = u16;

    fn with_positions(positions: [u8; 2]) -> Self {
        (positions[0] as Self) << 8 | (positions[1] as Self) << 12
    }

    fn player(&self) -> u8 {
        (self & 1) as u8
    }

    fn swap_players(&mut self) {
        *self ^= 1;
    }

    fn score(&self, player: u8) -> Self::Score {
        ((self >> (32 + 16 * player)) & 0xffff) as Self::Score
    }

    fn set_score(&mut self, player: u8, score: Self::Score) {
        *self &= !(0xffff << (32 + 16 * player));
        *self |= (score as Self) << (32 + 16 * player);
    }

    fn position(&self, player: u8) -> u8 {
        ((self >> (8 + 4 * player)) & 0xf) as u8
    }

    fn set_position(&mut self, player: u8, position: u8) {
        *self &= !(0xf << (8 + 4 * player));
        *self |= (position as Self) << (8 + 4 * player);
    }
}

impl State for u32 {
    type Score = u8;

    fn with_positions(positions: [u8; 2]) -> Self {
        (positions[0] as Self) << 8 | (positions[1] as Self) << 12
    }

    fn player(&self) -> u8 {
        (self & 1) as u8
    }

    fn swap_players(&mut self) {
        *self ^= 1;
    }

    fn score(&self, player: u8) -> Self::Score {
        ((self >> (16 + 8 * player)) & 0xff) as Self::Score
    }

    fn set_score(&mut self, player: u8, score: Self::Score) {
        *self &= !(0xff << (16 + 8 * player));
        *self |= (score as Self) << (16 + 8 * player);
    }

    fn position(&self, player: u8) -> u8 {
        ((self >> (8 + 4 * player)) & 0xf) as u8
    }

    fn set_position(&mut self, player: u8, position: u8) {
        *self &= !(0xf << (8 + 4 * player));
        *self |= (position as Self) << (8 + 4 * player);
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
