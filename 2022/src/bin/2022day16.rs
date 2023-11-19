use aoc2022::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Valve(u16);

const AA: Valve = Valve(((b'A' as u16) << 8) + (b'A' as u16));

impl FromStr for Valve {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Valve(
            ((s.as_bytes()[0] as u16) << 8) + (s.as_bytes()[1] as u16),
        ))
    }
}

impl fmt::Debug for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c1 = (self.0 >> 8) as u8 as char;
        let c2 = (self.0 & 0xff) as u8 as char;
        write!(f, "{}{}", c1, c2)
    }
}

#[derive(Clone, Debug)]
struct Valves(Vec<Valve>);

impl FromStr for Valves {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let v: Vec<Valve> = s
            .split(", ")
            .map(str::trim)
            .map(Valve::from_str)
            .collect::<StdResult<Vec<_>, Self::Err>>()?;
        Ok(Valves(v))
    }
}

#[derive(ParseByRegex, Clone)]
#[regex = r"Valve (?P<valve>..) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<tunnels>.+)"]
struct InputLine {
    valve: Valve,
    rate: usize,
    tunnels: Valves,
}

struct Network {
    flows: HashMap<Valve, usize>,
    ids: HashMap<Valve, usize>,
    distances: HashMap<(Valve, Valve), usize>,
}

impl Network {
    fn new(input: &[InputLine]) -> Self {
        let exits: HashMap<Valve, Valves> =
            input.iter().map(|l| (l.valve, l.tunnels.clone())).collect();
        let flows: HashMap<Valve, usize> = input
            .iter()
            .filter(|l| l.rate > 0)
            .map(|l| (l.valve, l.rate))
            .collect();
        let ids = flows
            .keys()
            .sorted()
            .enumerate()
            .map(|(i, l)| (*l, 1 << i))
            .collect();
        let mut distances: HashMap<(Valve, Valve), usize> = input
            .iter()
            .map(|l| l.valve)
            .flat_map(|l1| input.iter().map(move |l| (l1, l.valve)))
            .map(|link @ (l1, l2)| (link, if exits[&l2].0.contains(&l1) { 1 } else { 1000 }))
            .collect();

        for v in input.iter().map(|l| l.valve).permutations(3) {
            let k = v[0];
            let i = v[1];
            let j = v[2];
            let d_i_j = distances[&(i, j)];
            let d_i_k = distances[&(i, k)];
            let d_k_j = distances[&(k, j)];
            distances.insert((i, j), d_i_j.min(d_i_k + d_k_j));
        }

        Self {
            flows,
            ids,
            distances,
        }
    }

    fn visit(
        &self,
        valve: Valve,
        minutes: usize,
        mask: usize,
        pressure: usize,
        mut values: HashMap<usize, usize>,
    ) -> HashMap<usize, usize> {
        values.insert(mask, values.get(&mask).copied().unwrap_or(0).max(pressure));
        for (valve2, flow) in self.flows.iter().map(|(v, f)| (*v, *f)) {
            let to_use = self.distances[&(valve, valve2)] + 1;
            if to_use >= minutes {
                // Wouldn't reach valve2 in time
                continue;
            }
            if (self.ids[&valve2] & mask) != 0 {
                // valve2 already in this route
                continue;
            }
            let remaining_minutes = minutes - to_use;
            values = self.visit(
                valve2,
                remaining_minutes,
                mask | self.ids[&valve2],
                pressure + flow * remaining_minutes,
                values,
            );
        }
        values
    }
}

fn part1(input: &Network) -> usize {
    let values = input.visit(AA, 30, 0, 0, HashMap::new());
    values.into_values().max().unwrap()
}

fn part2(input: &Network) -> usize {
    let values = input.visit(AA, 26, 0, 0, HashMap::new());
    let mut max = 0;
    for (&bm1, &v1) in values.iter() {
        for (&bm2, &v2) in values.iter() {
            if (bm1 & bm2) == 0 {
                max = max.max(v1 + v2);
            }
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        let network = Network::new(&input);
        assert_eq!(part1(&network), 1651);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        let network = Network::new(&input);
        assert_eq!(part2(&network), 1707);
    }
}

pub fn main() -> Result<()> {
    let input = read_input_as_vec(16).unwrap();
    let network = Network::new(&input);
    println!("Part 1: {}", part1(&network));
    println!("Part 2: {}", part2(&network));
    Ok(())
}
