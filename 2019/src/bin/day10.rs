use aoc2019::*;

#[derive(Debug)]
struct AsteroidField {
    roids: Vec<(i32, i32)>,
    cols: i32,
    lines: i32,
}

fn manhat(r1: (i32, i32), r2: (i32, i32)) -> i32 {
    (r1.0 - r2.0).abs() + (r1.1 - r2.1).abs()
}

impl AsteroidField {
    pub fn parse_string(s: &str) -> Result<Self> {
        let lines = s.lines().count() as i32;
        let cols = s.lines().next().unwrap().len() as i32;
        let mut roids = Vec::new();
        for (linenum, line) in s.lines().enumerate() {
            for (colnum, roid) in line.chars().enumerate() {
                match roid {
                    '.' => {}
                    '#' => {
                        roids.push((colnum as i32, linenum as i32));
                    }
                    _ => return Err(format!("Unknown roid: '{}'", roid).into()),
                }
            }
        }
        Ok(Self { roids, cols, lines })
    }

    pub fn seeing_counts(&self) -> HashMap<(i32, i32), usize> {
        let mut ret = HashMap::new();
        for roid in self.roids.iter().copied() {
            let mut angles = HashSet::new();
            for other in self.roids.iter().copied().filter(|other| *other != roid) {
                let xdiff = other.0 - roid.0;
                let ydiff = -(other.1 - roid.1);
                let mut angle = (ydiff as f64).atan2(xdiff as f64);
                // angle is curently -pi to +pi with 0 being up
                // we want 0 to 2pi, with 0 being up, 2pi being up-again
                // so we want to map -pi to 0, to pi to 2pi
                if angle < 0.0 {
                    angle += 2.0 * std::f64::consts::PI;
                }
                let angle000 = (angle * 1_000_000.0) as usize;
                angles.insert(angle000);
            }
            ret.insert(roid, angles.len());
        }
        ret
    }

    pub fn vaporisation_order(&self, roid: (i32, i32)) -> Vec<(i32, i32)> {
        let mut roidmap = HashMap::new();
        for other in self.roids.iter().copied().filter(|other| *other != roid) {
            let xdiff = other.0 - roid.0;
            let ydiff = other.1 - roid.1;
            let mut angle = (xdiff as f64).atan2(-ydiff as f64);
            // angle is curently -pi to +pi with 0 being up
            // we want 0 to 2pi, with 0 being up, 2pi being up-again
            // so we want to map -pi to 0, to pi to 2pi
            if angle < 0.0 {
                angle += 2.0 * std::f64::consts::PI;
            }
            let angle0 = (angle * 1_000_000.0) as usize;
            roidmap.entry(angle0).or_insert_with(Vec::new).push(other);
        }
        let mut angles: Vec<_> = roidmap.into_iter().collect();
        angles.sort();
        for (_, v) in angles.iter_mut() {
            v.sort_by_cached_key(|other| -manhat(roid, *other));
        }
        let mut ret = Vec::new();
        let mut oldlen = 1;
        while ret.len() != oldlen {
            oldlen = ret.len();
            for (_angle, roidlist) in angles.iter_mut() {
                if let Some(roid) = roidlist.pop() {
                    ret.push(roid);
                }
            }
        }
        ret
    }
}

fn part1(input: &AsteroidField) -> ((i32, i32), usize) {
    let mut counts: Vec<_> = input.seeing_counts().into_iter().collect();
    counts.sort_by_cached_key(|(_, n)| *n);
    counts[counts.len() - 1]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        static TESTS: &[(&str, i32, i32, usize)] = &[
            (
                r".#..#
                  .....
                  #####
                  ....#
                  ...##",
                3,
                4,
                8,
            ),
            (
                r"......#.#.
                  #..#.#....
                  ..#######.
                  .#.#.###..
                  .#..#.....
                  ..#....#.#
                  #..#....#.
                  .##.#..###
                  ##...#..#.
                  .#....####",
                5,
                8,
                33,
            ),
            (
                r"#.#...#.#.
                  .###....#.
                  .#....#...
                  ##.#.#.#.#
                  ....#.#.#.
                  .##..###.#
                  ..#...##..
                  ..##....##
                  ......#...
                  .####.###.",
                1,
                2,
                35,
            ),
            (
                r".#..#..###
                  ####.###.#
                  ....###.#.
                  ..###.##.#
                  ##.##.#.#.
                  ....###..#
                  ..#.#..#.#
                  #..#.#.###
                  .##...##.#
                  .....#.#..",
                6,
                3,
                41,
            ),
            (
                r".#..##.###...#######
                  ##.############..##.
                  .#.######.########.#
                  .###.#######.####.#.
                  #####.##.#.##.###.##
                  ..#####..#.#########
                  ####################
                  #.####....###.#.#.##
                  ##.#################
                  #####.##.###..####..
                  ..######..##.#######
                  ####.##.####...##..#
                  .#####..#.######.###
                  ##...#.##########...
                  #.##########.#######
                  .####.#.###.###.#.##
                  ....##.##.###..#####
                  .#.#.###########.###
                  #.#.#.#####.####.###
                  ###.##.####.##.#..##",
                11,
                13,
                210,
            ),
        ];
        for (input, x, y, count) in TESTS {
            let input = input.replace(' ', "");
            let input = AsteroidField::parse_string(&input).expect("Unable to parse");
            eprintln!("{:?}", input);
            let ((x_, y_), count_) = part1(&input);
            assert_eq!(*x, x_);
            assert_eq!(*y, y_);
            assert_eq!(*count, count_);
        }
    }

    #[test]
    fn test_2() {
        static INPUT: &str = r".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";
        let input = INPUT.replace(' ', "");
        let input = AsteroidField::parse_string(&input).expect("Unable to parse");
        let seq = input.vaporisation_order((11, 13));
        const VERIFS: &[(usize, (i32, i32))] = &[
            (1, (11, 12)),
            (2, (12, 1)),
            (3, (12, 2)),
            (10, (12, 8)),
            (20, (16, 0)),
            (50, (16, 9)),
            (100, (10, 16)),
            (199, (9, 6)),
            (200, (8, 2)),
            (201, (10, 9)),
            (299, (11, 1)),
        ];
        for (pos, roid) in VERIFS.iter().copied() {
            assert_eq!(seq[pos - 1], roid);
        }
    }
}

fn part2(input: &AsteroidField, roid: (i32, i32)) -> (i32, i32) {
    let order = input.vaporisation_order(roid);
    order[199]
}

fn main() -> Result<()> {
    let input = AsteroidField::parse_string(&read_input(10)?)?;
    let p1 = part1(&input);
    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", part2(&input, p1.0));
    Ok(())
}
