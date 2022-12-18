use aoc2022::*;

#[derive(ParseByRegex, Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[regex = r"(?P<x>\d+),(?P<y>\d+),(?P<z>\d+)"]
struct LavaCube {
    x: i32,
    y: i32,
    z: i32,
}

impl LavaCube {
    fn neighbours(self) -> impl Iterator<Item = Self> {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .into_iter()
        .map(move |(xo, yo, zo)| Self {
            x: self.x + xo,
            y: self.y + yo,
            z: self.z + zo,
        })
    }
}

fn part1(input: &[LavaCube]) -> usize {
    let cubemap: HashSet<LavaCube> = input.iter().copied().collect();
    let mut total_surface = 0;
    for cube in input {
        for neighbour in cube.neighbours() {
            if !cubemap.contains(&neighbour) {
                total_surface += 1;
            }
        }
    }
    total_surface
}

fn part2(input: &[LavaCube]) -> usize {
    let cubemap: HashSet<LavaCube> = input.iter().copied().collect();

    // containing cube computation
    let (mut minx, mut miny, mut minz) =
        input.iter().fold((i32::MAX, i32::MAX, i32::MAX), |acc, c| {
            (min(acc.0, c.x), min(acc.1, c.y), min(acc.2, c.z))
        });

    let (mut maxx, mut maxy, mut maxz) =
        input.iter().fold((i32::MIN, i32::MIN, i32::MIN), |acc, c| {
            (max(acc.0, c.x), max(acc.1, c.y), max(acc.2, c.z))
        });

    minx -= 1; // To give us guaranteed room
    miny -= 1;
    minz -= 1;
    maxx += 1;
    maxy += 1;
    maxz += 1;

    // Starting at minx,miny,minz, we attempt to flood fill for water
    let mut water = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(LavaCube {
        x: minx,
        y: miny,
        z: minz,
    });
    while let Some(consider) = queue.pop_front() {
        water.insert(consider);
        for neighbour in consider.neighbours() {
            if neighbour.x < minx
                || neighbour.x > maxx
                || neighbour.y < miny
                || neighbour.y > maxy
                || neighbour.z < minz
                || neighbour.z > maxz
            {
                continue;
            }
            if !(water.contains(&neighbour)
                || cubemap.contains(&neighbour)
                || queue.contains(&neighbour))
            {
                queue.push_back(neighbour);
            }
        }
    }

    let mut total_surface = 0;
    for cube in input {
        for neighbour in cube.neighbours() {
            if water.contains(&neighbour) {
                total_surface += 1;
            }
        }
    }
    total_surface
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 64);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 64);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<LavaCube> = read_input_as_vec(18)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
