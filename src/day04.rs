use super::{
    day::Day,
    util::{CollectArray, DrainFilterMappable},
};
use itertools::iproduct;

const SIZE: usize = 5;
const WINS: [usize; 10] = [
    0b1111100000000000000000000,
    0b11111000000000000000,
    0b111110000000000,
    0b1111100000,
    0b11111,
    0b1000010000100001000010000,
    0b100001000010000100001000,
    0b10000100001000010000100,
    0b1000010000100001000010,
    0b100001000010000100001,
];

pub struct Board {
    rows: [[usize; SIZE]; SIZE],
    marks: usize,
}

impl Board {
    fn parse(rows: &[&str]) -> Board {
        Board {
            rows: rows
                .iter()
                .map(|row| {
                    row.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect_array()
                })
                .collect_array(),
            marks: 0,
        }
    }

    fn call(&mut self, number: usize) -> Option<usize> {
        self.marks |= iproduct!(0..SIZE, 0..SIZE)
            .filter(|&(r, c)| self.rows[r][c] == number)
            .map(|(r, c)| 1 << (r * SIZE + c))
            .sum::<usize>();
        WINS.iter().any(|&win| (self.marks & win) == win).then(|| {
            iproduct!(0..SIZE, 0..SIZE)
                .filter(|(r, c)| self.marks & (1 << (r * SIZE + c)) == 0)
                .map(|(r, c)| self.rows[r][c])
                .sum::<usize>()
                * number
        })
    }
}

pub struct Day04;

impl<'a> Day<'a> for Day04 {
    type Input = (impl 'a + Iterator<Item = usize>, Vec<Board>);
    type ProcessedInput = impl Iterator<Item = usize>;

    const DAY: usize = 4;

    fn parse(input: &'a str) -> Self::Input {
        let lines = input.lines().collect::<Vec<_>>();
        (
            lines[0].split(',').map(|n| n.parse().unwrap()),
            lines[1..].split(|&s| s == "").map(Board::parse).collect(),
        )
    }

    fn solve_part1((numbers, mut boards): Self::Input) -> (Self::ProcessedInput, String) {
        let mut scores = numbers.filter_map(move |n| boards.drain_filter_map(|b| b.call(n)).last());
        let score = scores.next().unwrap();
        (scores, score.to_string())
    }

    fn solve_part2(scores: Self::ProcessedInput) -> String {
        scores.last().unwrap().to_string()
    }
}
