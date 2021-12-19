use crate::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Point = (i32, i32, i32);

struct Orientation {
    roll: u8,
    face: u8,
}

impl Orientation {
    fn iter_all() -> impl Iterator<Item = Self> {
        (0..4).flat_map(|roll| (0..6).map(move |face| Self { roll, face }))
    }

    fn apply(&self, point: Point) -> Point {
        let cos = [1, 0, -1, 0][self.roll as usize];
        let sin = [0, 1, 0, -1][self.roll as usize];
        let rolled = (
            point.0,
            cos * point.1 + sin * point.2,
            -sin * point.1 + cos * point.2,
        );
        match self.face {
            0 => rolled,
            1 => (-rolled.1, rolled.0, rolled.2),
            2 => (-rolled.2, rolled.1, rolled.0),
            3 => (-rolled.0, -rolled.1, rolled.2),
            4 => (rolled.1, -rolled.0, rolled.2),
            5 => (rolled.2, rolled.1, -rolled.0),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Translation(i32, i32, i32);

impl Translation {
    fn onto(target: Point, origin: Point) -> Self {
        Self(
            target.0 - origin.0,
            target.1 - origin.1,
            target.2 - origin.2,
        )
    }

    fn manhattan_between(lhs: Self, rhs: Self) -> u32 {
        lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1) + lhs.2.abs_diff(rhs.2)
    }

    fn apply(&self, point: Point) -> Point {
        (point.0 + self.0, point.1 + self.1, point.2 + self.2)
    }
}

fn align(onto: &[Point], from: &[Point]) -> Option<(Vec<Point>, Translation)> {
    for orientation in Orientation::iter_all() {
        let mut bad = HashMap::<Point, Vec<Point>>::new();
        for &point in from[11..].iter() {
            for &ref_point in onto[11..].iter() {
                if bad.get(&point).map_or(false, |v| v.contains(&ref_point)) {
                    continue;
                }
                let translation = Translation::onto(ref_point, orientation.apply(point));
                let transform = |p| translation.apply(orientation.apply(p));
                let alignment = from
                    .iter()
                    .map(|&p| (p, transform(p)))
                    .filter(|(_p, tp)| onto.contains(tp))
                    .collect::<Vec<_>>();
                if alignment.len() >= 12 {
                    return Some((from.iter().map(|&p| transform(p)).collect(), translation));
                }
                for (p, tp) in alignment.into_iter() {
                    bad.entry(p).or_insert_with(Vec::new).push(tp);
                }
            }
        }
    }
    None
}

pub struct Day19;

impl<'a> Day<'a> for Day19 {
    type Input = Vec<Vec<Point>>;
    type ProcessedInput = Vec<Translation>;

    const DAY: usize = 19;

    fn parse(input: &'a str) -> Self::Input {
        let parse_point = |point| {
            str::split(point, ',')
                .next_tuple()
                .and_then(|(x, y, z)| Some((x.parse().ok()?, y.parse().ok()?, z.parse().ok()?)))
                .unwrap()
        };
        let parse_scan = |scan| str::lines(scan).skip(1).map(parse_point).collect();
        input.split("\n\n").map(parse_scan).collect()
    }

    fn solve_part1(scanners: Self::Input) -> (Self::ProcessedInput, String) {
        let mut found = HashMap::with_capacity(scanners.len());
        let mut translations = vec![Translation(0, 0, 0)];
        found.insert(0, scanners[0].clone());
        while found.len() < scanners.len() {
            for i in (0..scanners.len()).filter(|i| !found.contains_key(i)) {
                if let Some((beacons, translation)) =
                    found.values().find_map(|bs| align(bs, &scanners[i]))
                {
                    found.insert(i, beacons);
                    translations.push(translation);
                    break;
                }
            }
        }
        let beacons = found.into_values().flatten().collect::<HashSet<_>>();
        (translations, beacons.len().to_string())
    }

    fn solve_part2(positions: Self::ProcessedInput) -> String {
        positions
            .into_iter()
            .tuple_combinations()
            .map(|(a, b)| Translation::manhattan_between(a, b))
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test_day19 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        --- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14
    "};

    #[test]
    fn test_day19_examples() {
        let input = Day19::parse(EXAMPLE);
        let (input, part1) = Day19::solve_part1(input);
        assert_eq!(part1, "79");
        let part2 = Day19::solve_part2(input);
        assert_eq!(part2, "3621");
    }
}

bench_day!(19);
