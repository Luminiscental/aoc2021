use crate::{day::Day, util::SortedVec};
use hashbrown::HashSet;
use itertools::Itertools;
use std::cmp::Ordering;

const THRESHOLD: usize = 6; // this is heuristic

type Point = [i16; 3];
type CanonicalPoint = [i16; 3];
type ScanCanonicalization = SortedVec<(CanonicalPoint, (u8, u8))>;

fn sort3_by_abs(ns: &mut [i16; 3]) -> i16 {
    let mut ms = ns.map(i16::abs);
    let mut sign = 1;
    if ms[0] > ms[1] {
        ns.swap(0, 1);
        ms.swap(0, 1);
        sign *= -1;
    }
    if ms[0] > ms[2] {
        ns.swap(0, 2);
        ms.swap(0, 2);
        sign *= -1;
    }
    if ms[1] > ms[2] {
        ns.swap(1, 2);
        ms.swap(1, 2);
        sign *= -1;
    }
    sign
}

fn canonicalize(mut p: Point) -> CanonicalPoint {
    let sign = sort3_by_abs(&mut p) * p[0].signum() * p[1].signum();
    [p[0].abs(), p[1].abs(), sign * p[2]]
}

fn is_skew(p: CanonicalPoint) -> bool {
    0 != p[0] && p[0] != p[1] && p[1] != p[2].abs()
}

fn shape_of(scan: &[Point]) -> ScanCanonicalization {
    let mut shape = SortedVec::with_capacity(scan.len() * (scan.len() - 1));
    for ((i, [ix, iy, iz]), (j, [jx, jy, jz])) in scan.iter().enumerate().tuple_combinations() {
        let connection = canonicalize([ix - jx, iy - jy, iz - jz]);
        let flipped_connection = [connection[0], connection[1], -connection[2]];
        shape.push((connection, (i as u8, j as u8)));
        shape.push((flipped_connection, (j as u8, i as u8)));
    }
    shape
}

fn find_map_matches<R, F: FnMut(CanonicalPoint, (u8, u8), (u8, u8)) -> Option<R>>(
    lhs: &ScanCanonicalization,
    rhs: &ScanCanonicalization,
    mut f: F,
) -> Option<R> {
    let (mut lhs_conns, mut rhs_conns) = (lhs.iter().peekable(), rhs.iter().peekable());
    while let (Some(&&(lhs_conn, lhs_idxs)), Some(&&(rhs_conn, rhs_idxs))) =
        (lhs_conns.peek(), rhs_conns.peek())
    {
        match lhs_conn.cmp(&rhs_conn) {
            Ordering::Less => lhs_conns.next(),
            Ordering::Greater => rhs_conns.next(),
            Ordering::Equal => {
                if let Some(result) = f(lhs_conn, lhs_idxs, rhs_idxs) {
                    return Some(result);
                }
                lhs_conns.next();
                rhs_conns.next()
            }
        };
    }
    None
}

fn shapes_match(lhs: &ScanCanonicalization, rhs: &ScanCanonicalization) -> bool {
    let mut matches = 0;
    find_map_matches(lhs, rhs, |_, _, _| {
        matches += 1;
        (matches == THRESHOLD).then(|| ())
    })
    .is_some()
}

fn find_skew(
    lhs: &ScanCanonicalization,
    rhs: &ScanCanonicalization,
) -> Option<((u8, u8), (u8, u8))> {
    find_map_matches(lhs, rhs, |conn, onto_idxs, from_idxs| {
        is_skew(conn).then(|| (onto_idxs, from_idxs))
    })
}

fn orient(tv: Point, tp: Point, fv: Point, fp: Point, from: &[Point]) -> (Point, Vec<Point>) {
    let ind = |n| match n {
        _ if n == fv[0].abs() => 0,
        _ if n == fv[1].abs() => 1,
        _ => 2,
    };
    let pm = [ind(tv[0].abs()), ind(tv[1].abs()), ind(tv[2].abs())];
    let sign = [tv[0] / fv[pm[0]], tv[1] / fv[pm[1]], tv[2] / fv[pm[2]]];
    let offset = [
        tp[0] - sign[0] * fp[pm[0]],
        tp[1] - sign[1] * fp[pm[1]],
        tp[2] - sign[2] * fp[pm[2]],
    ];
    (
        offset,
        from.iter()
            .map(|point| {
                [
                    offset[0] + sign[0] * point[pm[0]],
                    offset[1] + sign[1] * point[pm[1]],
                    offset[2] + sign[2] * point[pm[2]],
                ]
            })
            .collect(),
    )
}

pub struct Day19;

impl<'a> Day<'a> for Day19 {
    type Input = Vec<Vec<Point>>;
    type ProcessedInput = Vec<Point>;

    const DAY: usize = 19;

    fn parse(input: &'a str) -> Self::Input {
        let parse_point = |point| {
            str::split(point, ',')
                .next_tuple()
                .and_then(|(x, y, z)| Some([x.parse().ok()?, y.parse().ok()?, z.parse().ok()?]))
                .unwrap()
        };
        let parse_scan = |scan| str::lines(scan).skip(1).map(parse_point).collect();
        input.split("\n\n").map(parse_scan).collect()
    }

    fn solve_part1(scans: Self::Input) -> (Self::ProcessedInput, String) {
        let shapes = scans.iter().map(|s| shape_of(s)).collect::<Vec<_>>();
        let mut oriented = vec![Vec::with_capacity(scans[0].len()); scans.len()];
        let mut offsets = Vec::with_capacity(scans.len());
        oriented[0] = scans[0].clone();
        offsets.push([0, 0, 0]);
        while oriented.iter().any(Vec::is_empty) {
            for i in 0..scans.len() {
                if oriented[i].is_empty() {
                    continue;
                }
                for j in 0..scans.len() {
                    if !oriented[j].is_empty() {
                        continue;
                    }
                    let scan_i = &oriented[i];
                    let scan_j = &scans[j];
                    if shapes_match(&shapes[i], &shapes[j]) {
                        let ((lhs_idx_i, rhs_idx_i), (lhs_idx_j, rhs_idx_j)) =
                            find_skew(&shapes[i], &shapes[j]).unwrap();
                        let (lhs_i, rhs_i) =
                            (scan_i[lhs_idx_i as usize], scan_i[rhs_idx_i as usize]);
                        let (lhs_j, rhs_j) =
                            (scan_j[lhs_idx_j as usize], scan_j[rhs_idx_j as usize]);
                        let vec_i = [
                            lhs_i[0] - rhs_i[0],
                            lhs_i[1] - rhs_i[1],
                            lhs_i[2] - rhs_i[2],
                        ];
                        let vec_j = [
                            lhs_j[0] - rhs_j[0],
                            lhs_j[1] - rhs_j[1],
                            lhs_j[2] - rhs_j[2],
                        ];
                        let (offset, oriented_j) = orient(vec_i, lhs_i, vec_j, lhs_j, scan_j);
                        oriented[j] = oriented_j;
                        offsets.push(offset);
                    }
                }
            }
        }
        let ans = oriented.into_iter().flatten().collect::<HashSet<_>>().len();
        (offsets, ans.to_string())
    }

    fn solve_part2(offsets: Self::ProcessedInput) -> String {
        offsets
            .into_iter()
            .tuple_combinations()
            .map(|(l, r)| l[0].abs_diff(r[0]) + l[1].abs_diff(r[1]) + l[2].abs_diff(r[2]))
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
