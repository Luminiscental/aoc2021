use crate::day::Day;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

const THRESHOLD: usize = 12;

type Point = [i16; 3];
type Conn = [u16; 3];
type Scan = Vec<Point>;
type ScanShape = Vec<HashMap<Conn, u8>>;

fn record_connections(scans: &[Scan]) -> Vec<ScanShape> {
    let mut all_connections = Vec::with_capacity(scans.len());
    for scan in scans.iter() {
        let npairs = scan.len() * (scan.len() - 1) / 2;
        let mut scan_connections = vec![HashMap::with_capacity(npairs); scan.len()];
        for ((i, [ix, iy, iz]), (j, &[jx, jy, jz])) in scan.iter().enumerate().tuple_combinations()
        {
            let mut connection = [ix.abs_diff(jx), iy.abs_diff(jy), iz.abs_diff(jz)];
            connection.sort_unstable();
            scan_connections[i].insert(connection, j as u8);
            scan_connections[j].insert(connection, i as u8);
        }
        all_connections.push(scan_connections);
    }
    all_connections
}

fn try_align(to_shape: &ScanShape, from_shape: &ScanShape) -> Option<(usize, usize, Conn)> {
    for (f_bcn, f_conns) in from_shape.iter().enumerate() {
        for (t_bcn, t_conns) in to_shape.iter().enumerate().skip(THRESHOLD - 1) {
            let match_conns = f_conns.keys().copied().filter(|f| t_conns.contains_key(f));
            if match_conns.clone().count() >= THRESHOLD - 1 {
                let skew = |p: &Conn| 0 != p[0] && p[0] != p[1] && p[1] != p[2];
                return Some((t_bcn, f_bcn, match_conns.clone().find(skew).unwrap()));
            }
        }
    }
    None
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
        let conns = record_connections(&scans);
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
                    if let Some((bcn_i, bcn_j, match_conn)) = try_align(&conns[i], &conns[j]) {
                        let conn_bcn_i = scan_i[conns[i][bcn_i][&match_conn] as usize];
                        let conn_bcn_j = scan_j[conns[j][bcn_j][&match_conn] as usize];
                        let (bcn_i, bcn_j) = (scan_i[bcn_i], scan_j[bcn_j]);
                        let conn_i = [
                            conn_bcn_i[0] - bcn_i[0],
                            conn_bcn_i[1] - bcn_i[1],
                            conn_bcn_i[2] - bcn_i[2],
                        ];
                        let conn_j = [
                            conn_bcn_j[0] - bcn_j[0],
                            conn_bcn_j[1] - bcn_j[1],
                            conn_bcn_j[2] - bcn_j[2],
                        ];
                        let (offset, oriented_j) = orient(conn_i, bcn_i, conn_j, bcn_j, scan_j);
                        oriented[j] = oriented_j;
                        offsets.push(offset);
                    }
                }
            }
        }
        let bcns = oriented.into_iter().flatten().collect::<HashSet<_>>();
        (offsets, bcns.len().to_string())
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
