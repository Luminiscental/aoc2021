use super::{
    day::Day,
    util::{self, CollectArray},
};

fn pack(s: &str) -> usize {
    s.chars()
        .map(|c| c as usize - 'a' as usize)
        .map(|n| 1 << n)
        .sum()
}

fn decode(digits: &[&str]) -> [usize; 10] {
    let eight = pack("abcdefg");
    let one = pack(digits.iter().find(|s| s.len() == 2).unwrap());
    let four = pack(digits.iter().find(|s| s.len() == 4).unwrap());
    let seven = pack(digits.iter().find(|s| s.len() == 3).unwrap());
    let two_three_five: [_; 3] = digits
        .iter()
        .filter(|s| s.len() == 5)
        .map(|s| pack(s))
        .collect_array();
    let horizontals = two_three_five[0] & two_three_five[1] & two_three_five[2];
    let middle = four & horizontals;
    let top_left = four & !one & !horizontals;
    let bottom_left = eight & !four & !seven & !horizontals;
    let two = *two_three_five
        .iter()
        .find(|&n| n & bottom_left != 0)
        .unwrap();
    let top_right = two & one;
    [
        eight & !middle,
        one,
        two,
        eight & !top_left & !bottom_left,
        four,
        eight & !top_right & !bottom_left,
        eight & !top_right,
        seven,
        eight,
        eight & !bottom_left,
    ]
}

pub struct Day08;

impl<'a> Day<'a> for Day08 {
    type Input = Vec<[&'a str; 14]>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 8;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| line.split(" | ").flat_map(|s| s.split(' ')).collect_array())
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let occurences = input
            .iter()
            .flat_map(|a| a[10..].iter())
            .filter(|s| [2, 3, 4, 7].contains(&s.len()))
            .count();
        (input, occurences.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input
            .iter()
            .map(|line| {
                let digits = decode(&line[..10]);
                util::unradix(
                    line[10..]
                        .iter()
                        .map(|s| pack(s))
                        .map(|d| digits.iter().position(|&n| n == d).unwrap())
                        .rev(),
                    10,
                )
            })
            .sum::<usize>()
            .to_string()
    }
}
