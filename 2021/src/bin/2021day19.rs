use aoc2021::*;

#[derive(ParseByRegex, PartialEq, Eq, PartialOrd, Ord)]
#[regex = r"^(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)"]
struct RelativeBeacon {
    x: i32,
    y: i32,
    z: i32,
}

struct Scanner {
    beacons: Vec<RelativeBeacon>,
}

struct Input {
    scanners: Vec<Scanner>,
}

impl FromStr for Input {
    type Err = GenError;

    fn from_str(input: &str) -> Result<Self> {
        let mut scanners = Vec::new();
        let mut lines = input.trim().lines().map(str::trim);
        'outer: loop {
            assert_eq!(lines.next().map(|s| &s[0..3]), Some("---"));
            let mut beacons = Vec::new();
            'inner: loop {
                if let Some(line) = lines.next() {
                    if line.is_empty() {
                        break 'inner;
                    }
                    beacons.push(RelativeBeacon::parse_by_regex(line)?);
                } else {
                    scanners.push(Scanner { beacons });
                    break 'outer;
                }
            }
            scanners.push(Scanner { beacons });
        }
        Ok(Self { scanners })
    }
}

impl RelativeBeacon {
    fn rotate(&self, rotation: &[i32], perm: &[usize]) -> Self {
        let pos = [
            self.x * rotation[0],
            self.y * rotation[1],
            self.z * rotation[2],
        ];

        RelativeBeacon {
            x: pos[perm[0]],
            y: pos[perm[1]],
            z: pos[perm[2]],
        }
    }

    fn offset_to(&self, other: (i32, i32, i32)) -> (i32, i32, i32) {
        (other.0 - self.x, other.1 - self.y, other.2 - self.z)
    }

    fn moveby(&self, ofs: (i32, i32, i32)) -> (i32, i32, i32) {
        (self.x + ofs.0, self.y + ofs.1, self.z + ofs.2)
    }
}

#[allow(clippy::type_complexity)]
fn clouds(input: &Input) -> (HashMap<usize, (i32, i32, i32)>, HashSet<(i32, i32, i32)>) {
    // We need to find the beacon cloud.
    // to do that, we assume the beacons from scanner zero are good, and we keep
    // rotating each of the other beacon sets until we get an overlap.
    let mut scanpos = HashMap::new();
    scanpos.insert(0, (0, 0, 0));
    let mut remaining_scanners = HashSet::new();
    (1..input.scanners.len()).for_each(|n| {
        remaining_scanners.insert(n);
    });
    let mut fixed_beacons = HashSet::new();
    for beacon in &input.scanners[0].beacons {
        fixed_beacons.insert((beacon.x, beacon.y, beacon.z));
    }
    let rots_multi_prod = [[-1, 1], [-1, 1], [-1, 1]]
        .iter()
        .copied()
        .multi_cartesian_product();
    let perms_multi_prod = (0..3).permutations(3);
    let multi_prod = rots_multi_prod.cartesian_product(perms_multi_prod);
    while !remaining_scanners.is_empty() {
        let mut matched = None;
        'scanners: for scanner in remaining_scanners.iter().copied() {
            println!("Trying scanner {}", scanner);
            // A cheap way for every valid 3d rotation is to try each combination of
            // negating or not each axis, and picking xyz from those rotated values
            // This is cheap only in the sense of computing the combinations, it likely
            // ends up duplicating a lot of outputs as a result.
            for (rotation, perm) in multi_prod.clone() {
                //println!("Try rotation: {:?} axis permutation: {:?}", rotation, perm);
                let rotated: Vec<_> = input.scanners[scanner]
                    .beacons
                    .iter()
                    .map(|b| b.rotate(&rotation, &perm))
                    .collect();
                // we now want to map this set of rotated beacons over
                // To do this, we pick pairs of beacons out of the set we've found already
                // and the list we've rotated, and we shift everything so that those
                // two overlap, then we look for matches
                for good_pos in fixed_beacons.iter().copied() {
                    for maybe_pos in rotated.iter() {
                        let offset = maybe_pos.offset_to(good_pos);
                        let moved = rotated.iter().map(|r| r.moveby(offset)).collect();
                        if fixed_beacons.intersection(&moved).count() >= 12 {
                            // We've found the scanner at this location
                            fixed_beacons.extend(moved.into_iter());
                            println!(
                                "Found scanner {} at position {:?} in rotation {:?} with permutation {:?}",
                                scanner, offset, rotation, perm
                            );
                            matched = Some(scanner);
                            scanpos.insert(scanner, offset);
                            break 'scanners;
                        }
                    }
                }
            }
        }
        if let Some(matched) = matched {
            remaining_scanners.remove(&matched);
            println!(
                "There are {} scanners left to locate",
                remaining_scanners.len()
            );
        } else {
            panic!("No scanner found this time?");
        }
    }
    (scanpos, fixed_beacons)
}

fn manhattan(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"--- scanner 0 ---
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
30,-46,-14"#;

    #[test]
    fn testcase1() {
        let input = Input::from_str(TEST_INPUT).unwrap();
        let (scanners, beacons) = clouds(&input);
        assert_eq!(beacons.len(), 79);
        println!("Scanners: {:?}", scanners);
        assert_eq!(
            scanners
                .values()
                .combinations(2)
                .map(|v| manhattan(v[0], v[1]))
                .max()
                .unwrap(),
            3621
        );
    }
}

fn main() -> Result<()> {
    let input = read_input(19)?;
    let input = Input::from_str(&input)?;
    let (scanners, beacons) = clouds(&input);
    println!("Part 1: {}", beacons.len());
    println!(
        "Part 2: {}",
        scanners
            .values()
            .combinations(2)
            .map(|v| manhattan(v[0], v[1]))
            .max()
            .ok_or("No scanner pairs?")?
    );
    Ok(())
}
