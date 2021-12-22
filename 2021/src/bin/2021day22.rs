use aoc2021::*;

#[derive(ParseByRegex, Clone, Copy, Debug, Eq, PartialEq)]
#[regex = r"^x=(?P<minx>-?\d+)\.\.(?P<maxx>-?\d+),y=(?P<miny>-?\d+)\.\.(?P<maxy>-?\d+),z=(?P<minz>-?\d+)\.\.(?P<maxz>-?\d+)$"]
struct Cuboid {
    minx: i64,
    maxx: i64,
    miny: i64,
    maxy: i64,
    minz: i64,
    maxz: i64,
}

static INIT_AREA: Cuboid = Cuboid {
    minx: -50,
    maxx: 50,
    miny: -50,
    maxy: 50,
    minz: -50,
    maxz: 50,
};

#[derive(ParseByRegex, Debug)]
enum Instruction {
    #[regex = r"^on (.+)$"]
    On(Cuboid),
    #[regex = r"^off (.+)$"]
    Off(Cuboid),
}

impl Cuboid {
    fn intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        // Intersection of two cuboids is either nothing, or another cuboid.
        let minx = max(self.minx, other.minx);
        let maxx = min(self.maxx, other.maxx);
        let miny = max(self.miny, other.miny);
        let maxy = min(self.maxy, other.maxy);
        let minz = max(self.minz, other.minz);
        let maxz = min(self.maxz, other.maxz);
        if (maxx < minx) || (maxy < miny) || (maxz < minz) {
            None
        } else {
            Some(Cuboid {
                minx,
                maxx,
                miny,
                maxy,
                minz,
                maxz,
            })
        }
    }

    fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        if let Some(int) = self.intersect(other) {
            // the two cuboids intersect, remove the intersection
            let mut ret = Vec::new();
            // if the intersection's maxy is less than our maxy then there's a "top" to slice off
            if int.maxy < self.maxy {
                ret.push(Cuboid {
                    minx: self.minx,
                    maxx: self.maxx,
                    miny: int.maxy + 1,
                    maxy: self.maxy,
                    minz: self.minz,
                    maxz: self.maxz,
                });
            }
            // if the intersection's miny is greater than our miny then there's a "bottom" to slice off
            if int.miny > self.miny {
                ret.push(Cuboid {
                    minx: self.minx,
                    maxx: self.maxx,
                    miny: self.miny,
                    maxy: int.miny - 1,
                    minz: self.minz,
                    maxz: self.maxz,
                });
            }
            // Now we want only to consider the slice from the intersections min/max y coordinates
            // so let's think about x.  similar rules apply for "left" and "right"
            if int.maxx < self.maxx {
                ret.push(Cuboid {
                    minx: int.maxx + 1,
                    maxx: self.maxx,
                    miny: int.miny,
                    maxy: int.maxy,
                    minz: self.minz,
                    maxz: self.maxz,
                });
            }
            if int.minx > self.minx {
                ret.push(Cuboid {
                    minx: self.minx,
                    maxx: int.minx - 1,
                    miny: int.miny,
                    maxy: int.maxy,
                    minz: self.minz,
                    maxz: self.maxz,
                });
            }
            // Finally having lopped left/right off, we're left with front and back
            if int.maxz < self.maxz {
                ret.push(Cuboid {
                    minx: int.minx,
                    maxx: int.maxx,
                    miny: int.miny,
                    maxy: int.maxy,
                    minz: int.maxz + 1,
                    maxz: self.maxz,
                });
            }
            if int.minz > self.minz {
                ret.push(Cuboid {
                    minx: int.minx,
                    maxx: int.maxx,
                    miny: int.miny,
                    maxy: int.maxy,
                    minz: self.minz,
                    maxz: int.minz - 1,
                });
            }

            ret
        } else {
            // No intersection, nothing to subtract, only self left
            vec![*self]
        }
    }

    fn size(&self) -> i64 {
        (self.maxx - self.minx + 1) * (self.maxy - self.miny + 1) * (self.maxz - self.minz + 1)
    }
}

impl Instruction {
    fn is_on(&self) -> bool {
        matches!(self, Instruction::On(_))
    }

    fn cuboid(&self) -> &Cuboid {
        match self {
            Instruction::On(v) => v,
            Instruction::Off(v) => v,
        }
    }
}

fn all_on_cubes(input: &[Instruction]) -> Vec<Cuboid> {
    let mut all_on_cubes: Vec<Cuboid> = Vec::new();
    for instr in input {
        let mut new_on_cubes = Vec::new();
        for old_cube in &all_on_cubes {
            // subtract this new cube from all the cubes
            new_on_cubes.extend(old_cube.subtract(instr.cuboid()).into_iter());
        }
        if instr.is_on() {
            new_on_cubes.push(*instr.cuboid());
        }
        all_on_cubes = new_on_cubes;
    }
    all_on_cubes
}

fn part1(all_on_cubes: &[Cuboid]) -> i64 {
    // finally, sum all the cube sizes
    all_on_cubes
        .iter()
        .filter_map(|c| c.intersect(&INIT_AREA))
        .map(|c| c.size())
        .sum()
}

fn part2(all_on_cubes: &[Cuboid]) -> i64 {
    // finally, sum all the cube sizes
    all_on_cubes.iter().map(|c| c.size()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"#;

    #[test]
    fn testcase1() {
        let input: Vec<Instruction> = input_as_vec(TEST_INPUT).unwrap();
        let input = all_on_cubes(&input);
        assert_eq!(part1(&input), 474140);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Instruction> = input_as_vec(TEST_INPUT).unwrap();
        let input = all_on_cubes(&input);
        assert_eq!(part2(&input), 2758514936282235);
    }

    #[test]
    fn intersect() {
        let within = Cuboid::parse_by_regex("x=10..12,y=10..12,z=10..12").unwrap();
        let isect = INIT_AREA.intersect(&within);
        assert_eq!(isect, Some(within));
        let without = Cuboid::parse_by_regex("x=100..120,y=100..120,z=100..120").unwrap();
        let isect = INIT_AREA.intersect(&without);
        assert_eq!(isect, None);
    }

    #[test]
    fn difference() {
        let within = Cuboid::parse_by_regex("x=10..12,y=10..12,z=10..12").unwrap();
        let diffs = INIT_AREA.subtract(&within);
        assert_eq!(diffs.len(), 6); // There are six cuboids
        assert_eq!(
            diffs[0],
            Cuboid::parse_by_regex("x=-50..50,y=13..50,z=-50..50").unwrap()
        );
        assert_eq!(
            diffs[1],
            Cuboid::parse_by_regex("x=-50..50,y=-50..9,z=-50..50").unwrap()
        );
        assert_eq!(
            diffs[2],
            Cuboid::parse_by_regex("x=13..50,y=10..12,z=-50..50").unwrap()
        );
        assert_eq!(
            diffs[3],
            Cuboid::parse_by_regex("x=-50..9,y=10..12,z=-50..50").unwrap()
        );
        assert_eq!(
            diffs[4],
            Cuboid::parse_by_regex("x=10..12,y=10..12,z=13..50").unwrap()
        );
        assert_eq!(
            diffs[5],
            Cuboid::parse_by_regex("x=10..12,y=10..12,z=-50..9").unwrap()
        );
    }
}

fn main() -> Result<()> {
    let input: Vec<Instruction> = read_input_as_vec(22)?;
    let input = all_on_cubes(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
