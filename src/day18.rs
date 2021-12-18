use crate::day::Day;
use itertools::iproduct;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Literal {
    value: u8,
    lefts: u8,
    rights: u8,
}

impl Literal {
    fn inc_left(mut self) -> Self {
        self.lefts += 1;
        self
    }

    fn inc_right(mut self) -> Self {
        self.rights += 1;
        self
    }

    fn split_left(mut self) -> Self {
        self.value /= 2;
        self.inc_left()
    }

    fn split_right(mut self) -> Self {
        self.value = (self.value + 1) / 2;
        self.inc_right()
    }

    fn split(self) -> (Self, Self) {
        (self.split_left(), self.split_right())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pairs {
    literals: Vec<Literal>,
}

impl Pairs {
    fn parse(string: &str) -> Self {
        let (mut string, mut literals) = (string.as_bytes(), Vec::new());
        let (mut lefts, mut rights) = (0, 0);
        while !string.is_empty() {
            match string[0] {
                b'[' => lefts += 1,
                b']' => rights -= 1,
                b',' => {
                    lefts -= 1;
                    rights += 1;
                }
                c => literals.push(Literal {
                    value: c - b'0',
                    lefts,
                    rights,
                }),
            }
            string = &string[1..];
        }
        Self { literals }
    }

    fn add(lhs: Self, rhs: Self) -> Self {
        let literals = lhs
            .literals
            .into_iter()
            .map(Literal::inc_left)
            .chain(rhs.literals.into_iter().map(Literal::inc_right))
            .collect();
        let mut sum = Self { literals };
        while sum.explode() || sum.split() {}
        sum
    }

    fn explode(&mut self) -> bool {
        self.literals
            .iter()
            .position(|literal| literal.lefts + literal.rights >= 5)
            .map_or(false, |i| {
                let (lhs, rhs) = (self.literals[i], self.literals.remove(i + 1));
                (i != 0).then(|| self.literals[i - 1].value += lhs.value);
                (i != self.literals.len() - 1).then(|| self.literals[i + 1].value += rhs.value);
                self.literals[i].value = 0;
                self.literals[i].lefts -= 1;
                true
            })
    }

    fn split(&mut self) -> bool {
        self.literals
            .iter()
            .position(|literal| literal.value >= 10)
            .map_or(false, |i| {
                let (lhs, rhs) = self.literals[i].split();
                self.literals[i] = lhs;
                self.literals.insert(i + 1, rhs);
                true
            })
    }

    fn magnitude(&self) -> u32 {
        let pow3 = [1, 3, 9, 27, 81];
        let pow2 = [1, 2, 4, 8, 16];
        self.literals
            .iter()
            .map(|literal| {
                literal.value as u32 * pow3[literal.lefts as usize] * pow2[literal.rights as usize]
            })
            .sum()
    }
}

pub struct Day18;

impl<'a> Day<'a> for Day18 {
    type Input = Vec<Pairs>;
    type ProcessedInput = Self::Input;

    const DAY: usize = 18;

    fn parse(input: &'a str) -> Self::Input {
        input.trim().lines().map(Pairs::parse).collect()
    }

    fn solve_part1(numbers: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = numbers.clone().into_iter().reduce(Pairs::add).unwrap();
        (numbers, ans.magnitude().to_string())
    }

    fn solve_part2(numbers: Self::ProcessedInput) -> String {
        iproduct!(numbers.clone().into_iter(), numbers.into_iter())
            .map(|(lhs, rhs)| Pairs::add(lhs, rhs).magnitude())
            .max()
            .unwrap()
            .to_string()
    }
}
