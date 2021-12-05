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
    fn minx(&self) -> usize {
        min(self.x1, self.x2)
    }
    fn miny(&self) -> usize {
        min(self.y1, self.y2)
    }

    fn maxx(&self) -> usize {
        max(self.x1, self.x2)
    }
    fn maxy(&self) -> usize {
        max(self.y1, self.y2)
    }

    fn is_orthogonal(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn point_in_orthogonal_line(&self, x: usize, y: usize) -> bool {
        (self.x1 == x && (self.miny() <= y && self.maxy() >= y))
            || (self.y1 == y && (self.minx() <= x && self.maxx() >= x))
    }
}

struct OceanFloor {
    vents: Vec<Vent>,
    minx: usize,
    maxx: usize,
    miny: usize,
    maxy: usize,
}

impl From<Vec<Vent>> for OceanFloor {
    fn from(vents: Vec<Vent>) -> Self {
        let (minx, maxx, miny, maxy) =
            vents
                .iter()
                .copied()
                .fold((100000, 0, 100000, 0), |mut acc, v| {
                    acc.0 = min(acc.0, v.minx());
                    acc.1 = max(acc.1, v.maxx());
                    acc.2 = min(acc.2, v.miny());
                    acc.3 = max(acc.3, v.maxy());
                    acc
                });
        OceanFloor {
            vents,
            minx,
            maxx,
            miny,
            maxy,
        }
    }
}

fn part1(input: &OceanFloor) -> usize {
    let orthogs: Vec<Vent> = input
        .vents
        .iter()
        .copied()
        .filter(Vent::is_orthogonal)
        .collect();
    let mut crossovers = 0;
    for x in input.minx..=input.maxx {
        for y in input.miny..=input.maxy {
            if orthogs
                .iter()
                .filter(|v| v.point_in_orthogonal_line(x, y))
                .count()
                > 1
            {
                crossovers += 1;
            }
        }
    }
    crossovers
}

fn part2(input: &OceanFloor) -> usize {
    let mut grid = Vec::new();
    for _ in 0..=input.maxy {
        let mut row = Vec::new();
        row.resize(input.maxx + 1, 0usize);
        grid.push(row);
    }

    for vent in &input.vents {
        let mut x = vent.x1;
        let mut y = vent.y1;
        let stepx = vent.x1.cmp(&vent.x2);
        let stepy = vent.y1.cmp(&vent.y2);
        loop {
            grid[y][x] += 1;
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

    grid.iter().for_each(|row| {
        row.iter().copied().for_each(|v| {
            if v == 0 {
                print!(".");
            } else {
                print!("{}", v)
            }
        });
        println!();
    });

    grid.iter()
        .map(|r| r.iter().filter(|n| **n > 1).count())
        .sum()
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
