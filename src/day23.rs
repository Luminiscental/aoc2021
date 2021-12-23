use crate::{
    day::Day,
    util::{self, CollectArray},
};
use itertools::Itertools;

pub struct Day23;

impl<'a> Day<'a> for Day23 {
    type Input = [DoorLayer; 2];
    type ProcessedInput = Self::Input;

    const DAY: usize = 23;

    fn parse(input: &'a str) -> Self::Input {
        let (outer, inner) = input.lines().skip(2).next_tuple().unwrap();
        let line_chars = |line: &str| line[3..].chars().step_by(2).take(4).collect_array();
        [outer, inner].map(line_chars).map(DoorLayer::from_chars)
    }

    fn solve_part1(doors: Self::Input) -> (Self::ProcessedInput, String) {
        let initial_state = State {
            hall: Hall::default(),
            doors,
        };
        let ans = util::dijkstra(initial_state, State::iter_moves, State::is_goal).unwrap();
        (doors, ans.to_string())
    }

    fn solve_part2(doors: Self::ProcessedInput) -> String {
        let initial_state = State {
            hall: Hall::default(),
            doors: [
                doors[0],
                DoorLayer::from_chars(['D', 'C', 'B', 'A']),
                DoorLayer::from_chars(['D', 'B', 'A', 'C']),
                doors[1],
            ],
        };
        util::dijkstra(initial_state, State::iter_moves, State::is_goal)
            .unwrap()
            .to_string()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State<const DEPTH: usize> {
    hall: Hall,
    doors: [DoorLayer; DEPTH],
}

impl<const DEPTH: usize> State<DEPTH> {
    fn movement_cost(occ: u8) -> usize {
        [1, 10, 100, 1000][occ as usize - 1]
    }

    fn is_goal(self) -> bool {
        (0..4).all(|door| (0..DEPTH).all(|layer| self.doors[layer].get(door) == Some(door + 1)))
    }

    fn try_enter_door(self, index: u8, mover: u8) -> Option<usize> {
        let layer = (0..DEPTH)
            .take_while(|&l| self.doors[l].get(index).is_none())
            .last()?;
        (layer + 1..DEPTH)
            .all(|l| self.doors[l].get(index) == Some(mover))
            .then(|| layer)
    }

    fn iter_moves(self) -> impl Iterator<Item = (usize, Self)> {
        self.iter_hall_moves().chain(self.iter_door_moves())
    }

    fn iter_hall_moves(self) -> impl Iterator<Item = (usize, Self)> {
        (0..7).filter_map(move |h| {
            let occ = self.hall.get(h)?;
            let enter_at = if h > occ { occ + 1 } else { occ };
            let dist = self.hall.try_move(h, enter_at)?;
            let layer = self.try_enter_door(occ - 1, occ)?;
            let mut next_state = self;
            next_state.hall.set(h, 0);
            next_state.doors[layer].set(occ - 1, occ);
            Some((Self::movement_cost(occ) * (dist + layer + 2), next_state))
        })
    }

    fn iter_door_moves(self) -> impl Iterator<Item = (usize, Self)> {
        (0..4)
            .filter_map(move |door| {
                let (layer, occ) = (0..DEPTH)
                    .find_map(|layer| self.doors[layer].get(door).map(|occ| (layer, occ)))?;
                let below_in_place =
                    (layer + 1..DEPTH).all(|layer| self.doors[layer].get(door) == Some(occ));
                if occ == door + 1 && below_in_place {
                    return None;
                }
                let lefts = (0..=door + 1)
                    .rev()
                    .take_while(move |&h| self.hall.get(h).is_none())
                    .map(move |h| (h, layer + 2 * (door + 2 - h) as usize - usize::from(h == 0)));
                let rights = (door + 2..=6)
                    .take_while(move |&h| self.hall.get(h).is_none())
                    .map(move |h| (h, layer + 2 * (h - 1 - door) as usize - usize::from(h == 6)));
                Some(lefts.chain(rights).map(move |(h, dist)| {
                    let mut next_state = self;
                    next_state.doors[layer].set(door, 0);
                    next_state.hall.set(h, occ);
                    (Self::movement_cost(occ) * dist, next_state)
                }))
            })
            .flatten()
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hall(u32);

impl Hall {
    fn set(&mut self, index: u8, occ: u8) {
        self.0 &= !(0b1111 << (4 * index));
        self.0 |= (occ as u32) << (4 * index);
    }

    fn get(self, index: u8) -> Option<u8> {
        let occ = ((self.0 >> (4 * index)) & 0b1111) as u8;
        (occ != 0).then(|| occ)
    }

    fn try_move(self, from: u8, to: u8) -> Option<usize> {
        let (delta, base) = if to > from {
            (to - from, 4 * (from + 1))
        } else {
            (from - to, 4 * to)
        };
        let mask = (1 << (4 * delta)) - 1;
        let edge_cases = usize::from(from == 0)
            + usize::from(to == 0)
            + usize::from(from == 6)
            + usize::from(to == 6);
        ((self.0 >> base) & mask == 0).then(|| 2 * delta as usize - edge_cases)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DoorLayer(u16);

impl DoorLayer {
    fn from_chars(chars: [char; 4]) -> Self {
        let from_char = |c| c as u8 - b'A' + 1;
        Self(
            chars
                .iter()
                .rev()
                .map(|&occ| from_char(occ) as u16)
                .reduce(|layer, occ| layer << 4 | occ)
                .unwrap(),
        )
    }

    fn set(&mut self, index: u8, occ: u8) {
        self.0 &= !(0b1111 << (4 * index));
        self.0 |= (occ as u16) << (4 * index);
    }

    fn get(self, index: u8) -> Option<u8> {
        let occ = ((self.0 >> (4 * index)) & 0b1111) as u8;
        (occ != 0).then(|| occ)
    }
}

#[cfg(test)]
mod test_day23 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########
    "};

    #[test]
    fn test_day23_examples() {
        let input = Day23::parse(EXAMPLE);
        let (input, part1) = Day23::solve_part1(input);
        assert_eq!(part1, "12521");
        assert_eq!(Day23::solve_part2(input), "44169");
    }
}

bench_day!(23);
