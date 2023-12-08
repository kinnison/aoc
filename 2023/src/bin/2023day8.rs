use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(8)?;
    let input = parse_map(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug, ParseByRegex)]
#[regex = r"^(?P<from>...) = .(?P<left>...), (?P<right>...).$"]
struct Rule {
    from: Room,
    left: Room,
    right: Room,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Room(u64);

impl std::fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = [
            (self.0 >> 16) as u8 as char,
            ((self.0 >> 8) & 0xFF) as u8 as char,
            (self.0 & 0xFF) as u8 as char,
        ];
        f.debug_tuple("Room").field(&ch).finish()
    }
}

impl FromStr for Room {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let ch1 = (s.as_bytes()[0]) as u64;
        let ch2 = (s.as_bytes()[1]) as u64;
        let ch3 = (s.as_bytes()[2]) as u64;
        Ok(Room((ch1 << 16) + (ch2 << 8) + ch3))
    }
}

impl Room {
    #[allow(non_snake_case)]
    fn AAA() -> Self {
        Room::from_str("AAA").unwrap()
    }

    #[allow(non_snake_case)]
    fn ZZZ() -> Self {
        Room::from_str("ZZZ").unwrap()
    }

    fn ends_a(&self) -> bool {
        (self.0 & 0xFF) == (b'A' as u64)
    }

    fn ends_z(&self) -> bool {
        (self.0 & 0xFF) == (b'Z' as u64)
    }
}

#[derive(Debug)]
struct Map {
    path: String,
    rules: HashMap<Room, (Room, Room)>,
}

fn parse_map(input: &str) -> Map {
    let (rules, path): (Vec<Rule>, _) = input_as_vec_and_first(input).unwrap();

    let rules = rules
        .into_iter()
        .map(|rule| (rule.from, (rule.left, rule.right)))
        .collect();

    Map { path, rules }
}

fn part1(input: &Map) -> usize {
    // Route length
    let mut pos = Room::AAA();
    let endpos = Room::ZZZ();
    let mut len = 0;
    for lr in input.path.chars().cycle() {
        pos = match lr {
            'L' => input.rules[&pos].0,
            'R' => input.rules[&pos].1,
            _ => unreachable!(),
        };
        len += 1;
        if pos == endpos {
            break;
        }
    }

    len
}

fn part2(input: &Map) -> u64 {
    let ghosts = input
        .rules
        .keys()
        .copied()
        .filter(|k| k.ends_a())
        .sorted()
        .collect_vec();

    // for each ghost, collect a path to an ends_z and then one more, and see where it shows up
    let plen: Vec<u64> = ghosts
        .into_iter()
        .map(|sghost| {
            let mut ghost = sghost;
            let mut chain = input.path.chars().cycle().peekable();
            let mut len = 0;
            while !ghost.ends_z() {
                ghost = match chain.next().unwrap() {
                    'L' => input.rules[&ghost].0,
                    'R' => input.rules[&ghost].1,
                    _ => unreachable!(),
                };
                len += 1;
            }
            len
        })
        .collect_vec();

    plen.into_iter().fold(1, |acc, x| acc.lcm(x))
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    static TEST_INPUT2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    static TEST_INPUT3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn testcase1_1() {
        let input = parse_map(TEST_INPUT1);
        eprintln!("{input:?}");
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn testcase1_2() {
        let input = parse_map(TEST_INPUT2);
        eprintln!("{input:?}");
        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn testcase2() {
        let input = parse_map(TEST_INPUT3);
        assert_eq!(part2(&input), 6);
    }
}
