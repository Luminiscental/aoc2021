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

#[derive(Debug)]
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
            lines[2..]
                .split(|s| s.is_empty())
                .map(Board::parse)
                .collect(),
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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "};

    #[test]
    fn test_day04_examples() {
        let input = Day04::parse(EXAMPLE);
        let (input, part1) = Day04::solve_part1(input);
        let part2 = Day04::solve_part2(input);
        assert_eq!(part1, "4512");
        assert_eq!(part2, "1924");
    }
}

bench_day!(04);
