use crate::day::Day;
use itertools::Itertools;

#[derive(Clone)]
pub struct Pairs(Vec<(u8, u8, u8)>);

impl Pairs {
    fn parse(string: &str) -> Self {
        let (mut string, mut literals) = (string.as_bytes(), Vec::new());
        let mut lr = 0x00;
        while !string.is_empty() {
            match string[0] {
                b'[' => lr += 0x10,
                b']' => lr -= 0x01,
                b',' => lr -= 0x10 - 0x01,
                c => literals.push((c - b'0', lr >> 4, lr & 0x0f)),
            }
            string = &string[1..];
        }
        Self(literals)
    }

    fn add(lhs: Self, rhs: Self) -> Self {
        let ls = lhs.0.into_iter().map(|(v, l, r)| (v, l + 1, r));
        let rs = rhs.0.into_iter().map(|(v, l, r)| (v, l, r + 1));
        let mut sum = Self(ls.chain(rs).collect());
        while sum.explode().is_some() || sum.split().is_some() {}
        sum
    }

    fn explode(&mut self) -> Option<()> {
        let i = self.0.iter().position(|(_, l, r)| l + r >= 5)?;
        let (lhs, rhs) = (self.0[i], self.0.remove(i + 1));
        (i != 0).then(|| self.0[i - 1].0 += lhs.0);
        (i != self.0.len() - 1).then(|| self.0[i + 1].0 += rhs.0);
        self.0[i] = (0, lhs.1 - 1, lhs.2);
        Some(())
    }

    fn split(&mut self) -> Option<()> {
        let i = self.0.iter().position(|lit| lit.0 >= 10)?;
        let sp = self.0[i];
        self.0[i] = (sp.0 / 2, sp.1 + 1, sp.2);
        self.0.insert(i + 1, ((sp.0 + 1) / 2, sp.1, sp.2 + 1));
        Some(())
    }

    fn magnitude(&self) -> u32 {
        let pow3 = |i| [1, 3, 9, 27, 81][i as usize];
        let pow2 = |i| [1, 2, 4, 8, 16][i as usize];
        let magn = |&(lit, ls, rs)| lit as u32 * pow3(ls) * pow2(rs);
        self.0.iter().map(magn).sum()
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
        let m = |l, r| Pairs::add(l, r).magnitude();
        let ord = numbers.into_iter().tuple_combinations();
        let ms = ord.map(|(lhs, rhs)| m(lhs.clone(), rhs.clone()).max(m(rhs, lhs)));
        ms.max().unwrap().to_string()
    }
}

#[cfg(test)]
mod test_day18 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "};

    #[test]
    fn test_day18_examples() {
        let input = Day18::parse(EXAMPLE);
        let (input, part1) = Day18::solve_part1(input);
        assert_eq!(part1, "4140");
        let part2 = Day18::solve_part2(input);
        assert_eq!(part2, "3993");
    }
}

bench_day!(18);
