use aoc2020::*;

// Hex directions are defined as follows:
// We have an x/y coordinate.
// x is the west/east direction (east +ve)
// y is the north/south direction (south +ve)
// Each time you move north, x shifts westward 0.5, so nw doesn't adjust x
// Each time you move south, x shifts eastward 0.5, so se doesn't adjust x

#[derive(Debug, Copy, Clone)]
enum HexDirection {
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl HexDirection {
    fn offsets(self) -> (i32, i32) {
        match self {
            Self::East => (1, 0),
            Self::West => (-1, 0),
            Self::NorthEast => (1, -1),
            Self::NorthWest => (0, -1),
            Self::SouthEast => (0, 1),
            Self::SouthWest => (-1, 1),
        }
    }

    fn list_from(dirs: &str) -> Vec<Self> {
        let mut ret = Vec::new();
        let mut dirs = dirs.chars();
        while let Some(c) = dirs.next() {
            ret.push(match c {
                'e' => Self::East,
                'w' => Self::West,
                'n' | 's' => match (c, dirs.next().unwrap()) {
                    ('n', 'e') => Self::NorthEast,
                    ('n', 'w') => Self::NorthWest,
                    ('s', 'e') => Self::SouthEast,
                    ('s', 'w') => Self::SouthWest,
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            });
        }
        ret
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct HexCoord {
    x: i32,
    y: i32,
}

impl HexCoord {
    fn walk_to(mut self, path: &[HexDirection]) -> Self {
        for (xofs, yofs) in path.iter().copied().map(|d| d.offsets()) {
            self.x += xofs;
            self.y += yofs;
        }
        self
    }

    fn neighbours(self) -> HexNeighbours {
        HexNeighbours {
            loc: self,
            dir: Some(HexDirection::East),
        }
    }
}

struct HexNeighbours {
    loc: HexCoord,
    dir: Option<HexDirection>,
}

impl Iterator for HexNeighbours {
    type Item = HexCoord;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            None => None,
            Some(d) => {
                let (xofs, yofs) = d.offsets();
                let ret = HexCoord {
                    x: self.loc.x + xofs,
                    y: self.loc.y + yofs,
                };
                use HexDirection::*;
                self.dir = match d {
                    East => Some(West),
                    West => Some(NorthEast),
                    NorthEast => Some(NorthWest),
                    NorthWest => Some(SouthEast),
                    SouthEast => Some(SouthWest),
                    SouthWest => None,
                };
                Some(ret)
            }
        }
    }
}

fn _part1(input: &[Vec<HexDirection>]) -> HashSet<HexCoord> {
    let mut flipped = HashSet::new();
    for path in input {
        let tile = HexCoord::default().walk_to(path);
        if flipped.contains(&tile) {
            flipped.remove(&tile);
        } else {
            flipped.insert(tile);
        }
    }
    flipped
}

fn part1(input: &[Vec<HexDirection>]) -> usize {
    let flipped = _part1(input);
    flipped.len()
}

fn part2(input: &[Vec<HexDirection>]) -> usize {
    let mut tileset = _part1(input);
    for _ in 0..100 {
        let mut neighbours: HashMap<HexCoord, usize> = HashMap::new();
        // For every flipped tile, note its neighbours
        for tile in tileset.iter().copied() {
            for neigh in tile.neighbours() {
                *(neighbours.entry(neigh).or_default()) += 1;
            }
        }
        tileset = neighbours
            .into_iter()
            .flat_map(|(loc, ncount)| match ncount {
                2 => Some(loc), // 2 == always black
                1 => {
                    if tileset.contains(&loc) {
                        Some(loc) // 1 and was black before == still black
                    } else {
                        None // 1 and was white before == still white
                    }
                }
                _ => None, // 0 or > 2 => white now, no matter what
            })
            .collect();
    }

    tileset.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

    #[test]
    fn origin() {
        let dirs = HexDirection::list_from("nwwswee");
        let origin = HexCoord::default();
        let neworigin = origin.walk_to(&dirs);
        assert_eq!(origin, neworigin);
    }

    #[test]
    fn testcase1() {
        let input: Vec<_> = TEST_INPUT
            .trim()
            .lines()
            .map(|l| HexDirection::list_from(l.trim()))
            .collect();
        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn testcase2() {
        let input: Vec<_> = TEST_INPUT
            .trim()
            .lines()
            .map(|l| HexDirection::list_from(l.trim()))
            .collect();
        assert_eq!(part2(&input), 2208);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(24)?;
    let input: Vec<_> = input
        .trim()
        .lines()
        .map(|l| HexDirection::list_from(l.trim()))
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
