use crate::day::Day;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    fn parse<I: Iterator<Item = u8>>(bits: &mut I) -> (usize, Self) {
        let take_number = |n, bits: &mut I| {
            (0..n)
                .rev()
                .map(|i| (bits.next().unwrap() as u64) << i)
                .sum::<u64>()
        };
        let version = take_number(3, bits) as u8;
        let type_id = take_number(3, bits) as u8;
        match type_id {
            4 => {
                let (mut size, mut literal) = (6, 0);
                while let Some(flag) = bits.next() {
                    literal = 16 * literal + take_number(4, bits);
                    size += 5;
                    if flag == 0 {
                        break;
                    }
                }
                (size, Packet::Literal(version, literal))
            }
            _ => {
                let (mut size, packet_max, bit_max) = match bits.next().unwrap() {
                    0 => (22, usize::MAX, 22 + take_number(15, bits) as usize),
                    _ => (18, take_number(11, bits) as usize, usize::MAX),
                };
                let mut packets = Vec::new();
                while size < bit_max && packets.len() < packet_max {
                    let (subpacket_size, subpacket) = Packet::parse(bits);
                    size += subpacket_size;
                    packets.push(subpacket);
                }
                (size, Packet::Operator(version, type_id, packets))
            }
        }
    }

    fn sum_versions(&self) -> u32 {
        match self {
            Self::Literal(v, _) => *v as u32,
            Self::Operator(v, _, ps) => *v as u32 + ps.iter().map(Self::sum_versions).sum::<u32>(),
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Self::Literal(_, n) => *n,
            Self::Operator(_, op, packets) => match (op, packets.iter().map(Self::evaluate)) {
                (0, values) => values.sum(),
                (1, values) => values.product(),
                (2, values) => values.min().unwrap(),
                (3, values) => values.max().unwrap(),
                (5, mut values) => values.next_tuple().map(|(a, b)| a > b).unwrap().into(),
                (6, mut values) => values.next_tuple().map(|(a, b)| a < b).unwrap().into(),
                (_, mut values) => values.next_tuple().map(|(a, b)| a == b).unwrap().into(),
            },
        }
    }
}

pub struct Day16;

impl<'a> Day<'a> for Day16 {
    type Input = Packet;
    type ProcessedInput = Self::Input;

    const DAY: usize = 16;

    fn parse(input: &'a str) -> Self::Input {
        Packet::parse(
            &mut input
                .chars()
                .filter_map(|c| c.to_digit(16))
                .flat_map(|n| (0..4).rev().map(move |i| ((n >> i) & 1) as u8)),
        )
        .1
    }

    fn solve_part1(packet: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = packet.sum_versions();
        (packet, ans.to_string())
    }

    fn solve_part2(packet: Self::ProcessedInput) -> String {
        packet.evaluate().to_string()
    }
}

#[cfg(test)]
mod test_day16 {
    use super::*;

    fn assert_part1(example: &str, output: &str) {
        let input = Day16::parse(example);
        let (_, part1) = Day16::solve_part1(input);
        assert_eq!(part1, output);
    }

    fn assert_part2(example: &str, output: &str) {
        let input = Day16::parse(example);
        let (input, _) = Day16::solve_part1(input);
        let part2 = Day16::solve_part2(input);
        assert_eq!(part2, output);
    }

    #[test]
    fn test_day16_examples() {
        assert_part1("8A004A801A8002F478", "16");
        assert_part1("620080001611562C8802118E34", "12");
        assert_part1("C0015000016115A2E0802F182340", "23");
        assert_part1("A0016C880162017C3686B18A3D4780", "31");

        assert_part2("C200B40A82", "3");
        assert_part2("04005AC33890", "54");
        assert_part2("CE00C43D881120", "9");
        assert_part2("D8005AC2A8F0", "1");
        assert_part2("F600BC2D8F", "0");
        assert_part2("9C005AC2F8F0", "0");
        assert_part2("9C0141080250320F1802104A08", "1");
    }
}

bench_day!(16);
