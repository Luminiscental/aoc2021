use crate::{day::Day, util};
use itertools::Itertools;

fn apply_operation(type_id: u8, mut values: impl Iterator<Item = u64>) -> u64 {
    match type_id {
        0 => values.sum(),
        1 => values.product(),
        2 => values.min().unwrap(),
        3 => values.max().unwrap(),
        5 => values.next_tuple().map(|(a, b)| a > b).unwrap().into(),
        6 => values.next_tuple().map(|(a, b)| a < b).unwrap().into(),
        7 => values.next_tuple().map(|(a, b)| a == b).unwrap().into(),
        _ => panic!("Unexpected type id: {}", type_id),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Literal(u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

impl Packet {
    fn parse<I: Iterator<Item = u8>>(bits: &mut I) -> (usize, Self) {
        let take_number =
            |n, bits: &mut I| util::unradix((0..n).map(|_| bits.next().unwrap() as u64), 2);
        let version = take_number(3, bits) as u8;
        let type_id = take_number(3, bits) as u8;
        match type_id {
            4 => {
                let (mut size, mut literal) = (6, 0);
                while let Some(flag) = bits.next() {
                    literal = 16 * literal + take_number(4, bits);
                    size += 4;
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
            Self::Operator(_, op, ps) => apply_operation(*op, ps.iter().map(Self::evaluate)),
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
                .map(|c| c.to_digit(16).unwrap())
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

    const EXAMPLE1_1: &str = "8A004A801A8002F478";
    const EXAMPLE1_2: &str = "620080001611562C8802118E34";
    const EXAMPLE1_3: &str = "C0015000016115A2E0802F182340";
    const EXAMPLE1_4: &str = "A0016C880162017C3686B18A3D4780";
    const EXAMPLE2_1: &str = "C200B40A82";
    const EXAMPLE2_2: &str = "04005AC33890";
    const EXAMPLE2_3: &str = "CE00C43D881120";
    const EXAMPLE2_4: &str = "D8005AC2A8F0";
    const EXAMPLE2_5: &str = "F600BC2D8F";
    const EXAMPLE2_6: &str = "9C005AC2F8F0";
    const EXAMPLE2_7: &str = "9C0141080250320F1802104A08";

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
        assert_part1(EXAMPLE1_1, "16");
        assert_part1(EXAMPLE1_2, "12");
        assert_part1(EXAMPLE1_3, "23");
        assert_part1(EXAMPLE1_4, "31");
        assert_part2(EXAMPLE2_1, "3");
        assert_part2(EXAMPLE2_2, "54");
        assert_part2(EXAMPLE2_3, "9");
        assert_part2(EXAMPLE2_4, "1");
        assert_part2(EXAMPLE2_5, "0");
        assert_part2(EXAMPLE2_6, "0");
        assert_part2(EXAMPLE2_7, "1");
    }
}

bench_day!(16);
