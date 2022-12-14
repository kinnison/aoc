use aoc2022::*;

struct InputLine {
    coords: Vec<(i32, i32)>,
}

impl FromStr for InputLine {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let coords = s
            .trim()
            .split(" -> ")
            .map(|s| s.trim().split_once(',').unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect_vec();
        Ok(Self { coords })
    }
}

#[derive(Clone)]
struct Cave {
    content: HashSet<(i32, i32)>,
    floor: i32,
}

impl Cave {
    fn new(input: &[InputLine]) -> Self {
        let mut ret = Self {
            content: HashSet::new(),
            floor: -1,
        };
        for seq in input {
            for (start, end) in seq.coords.iter().copied().tuple_windows() {
                ret.add_line(start, end);
            }
        }
        ret
    }

    fn add_line(&mut self, start: (i32, i32), end: (i32, i32)) {
        if start.0 == end.0 {
            // Vertical line
            for y in min(start.1, end.1)..=max(start.1, end.1) {
                self.content.insert((start.0, y));
            }
        } else {
            // Horizontal line
            for x in min(start.0, end.0)..=max(start.0, end.0) {
                self.content.insert((x, start.1));
            }
        }
        self.floor = max(self.floor, max(start.1, end.1));
    }

    /// Drop sand into the cave.
    ///
    /// Returns true if the sand comes to rest
    fn drop_sand(&mut self) -> Option<(i32, i32)> {
        let mut sandx = 500;
        let mut sandy = 0;

        while sandy < self.floor {
            if !self.content.contains(&(sandx, sandy + 1)) {
                // drop down
                sandy += 1;
            } else if !self.content.contains(&(sandx - 1, sandy + 1)) {
                // drop down/left
                sandx -= 1;
                sandy += 1;
            } else if !self.content.contains(&(sandx + 1, sandy + 1)) {
                // drop down/right
                sandx += 1;
                sandy += 1;
            } else {
                // come to rest
                self.content.insert((sandx, sandy));
                return Some((sandx, sandy));
            }
        }

        None
    }
}

fn part1(input: &Cave) -> usize {
    let mut cave = input.clone();
    for i in 0.. {
        if cave.drop_sand().is_none() {
            // Did not come to rest, this block is the first to fall into the abyss
            return i;
        }
    }
    unreachable!()
}

fn part2(input: &Cave) -> usize {
    let mut cave = input.clone();
    cave.add_line((-1000, cave.floor + 2), (1000, cave.floor + 2));
    for i in 1.. {
        if cave.drop_sand() == Some((500, 0)) {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        let cave = Cave::new(&input);
        assert_eq!(part1(&cave), 24);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        let cave = Cave::new(&input);
        assert_eq!(part2(&cave), 93);
    }
}

pub fn main() -> Result<()> {
    let input = read_input_as_vec(14).unwrap();
    let cave = Cave::new(&input);
    println!("Part 1: {}", part1(&cave));
    println!("Part 2: {}", part2(&cave));
    Ok(())
}
