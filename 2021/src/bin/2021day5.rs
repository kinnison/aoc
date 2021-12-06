use aoc2021::*;

#[derive(Debug, ParseByRegex, Clone, Copy)]
#[regex = r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)"]
struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Vent {
    fn maxx(&self) -> usize {
        max(self.x1, self.x2)
    }
    fn maxy(&self) -> usize {
        max(self.y1, self.y2)
    }

    fn is_orthogonal(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
}

struct OceanFloor {
    vents: Vec<Vent>,
    maxx: usize,
    maxy: usize,
}

impl From<Vec<Vent>> for OceanFloor {
    fn from(vents: Vec<Vent>) -> Self {
        let (maxx, maxy) = vents.iter().copied().fold((100000, 0), |mut acc, v| {
            acc.0 = max(acc.0, v.maxx());
            acc.1 = max(acc.1, v.maxy());
            acc
        });
        OceanFloor { vents, maxx, maxy }
    }
}

impl OceanFloor {
    fn make_grid(&self, all_vents: bool) -> usize {
        let mut seen = HashSet::new();
        let mut dups = HashSet::new();

        for vent in self.vents.iter().filter(|v| all_vents || v.is_orthogonal()) {
            let mut x = vent.x1;
            let mut y = vent.y1;
            let stepx = vent.x1.cmp(&vent.x2);
            let stepy = vent.y1.cmp(&vent.y2);
            loop {
                let point = (x, y);
                if seen.contains(&point) {
                    dups.insert(point);
                } else {
                    seen.insert(point);
                }
                if x == vent.x2 && y == vent.y2 {
                    break;
                }
                match stepx {
                    Ordering::Less => x += 1,
                    Ordering::Equal => {}
                    Ordering::Greater => x -= 1,
                }
                match stepy {
                    Ordering::Less => y += 1,
                    Ordering::Equal => {}
                    Ordering::Greater => y -= 1,
                }
            }
        }
        dups.len()
    }
}

fn part1(input: &OceanFloor) -> usize {
    input.make_grid(false)
}

fn part2(input: &OceanFloor) -> usize {
    input.make_grid(true)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn testcase1() {
        let input: Vec<Vent> = input_as_vec(TEST_INPUT).unwrap();
        let input = input.into();
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Vent> = input_as_vec(TEST_INPUT).unwrap();
        let input = input.into();
        assert_eq!(part2(&input), 12);
    }
}

fn main() -> Result<()> {
    let input: Vec<Vent> = read_input_as_vec(5)?;
    let input = input.into();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
