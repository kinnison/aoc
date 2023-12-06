use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(5)?;
    let input = parse_almanac(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Vec<MapEntry>>,
}

#[derive(Debug)]
struct MapEntry {
    dest: u64,
    src: u64,
    len: u64,
}

impl From<&str> for MapEntry {
    fn from(value: &str) -> Self {
        let mut parts = value.split_ascii_whitespace();
        let dest = parts.next().unwrap().parse().unwrap();
        let src = parts.next().unwrap().parse().unwrap();
        let len = parts.next().unwrap().parse().unwrap();
        Self { dest, src, len }
    }
}

fn parse_almanac(input: &str) -> Almanac {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    println!("Seeds: {seeds:?}");

    let seeds = seeds
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|e| e.parse::<u64>().unwrap())
        .collect();

    let maps = maps
        .split("\n\n")
        .map(|map| map.lines().skip(1).map(MapEntry::from).collect())
        .collect();

    Almanac { seeds, maps }
}

impl MapEntry {
    fn maps(&self, v: u64) -> Option<u64> {
        if v >= self.src && v < (self.src + self.len) {
            Some(self.dest + v - self.src)
        } else {
            None
        }
    }

    fn block_map(maps: &[MapEntry], v: u64) -> u64 {
        maps.iter().filter_map(|m| m.maps(v)).next().unwrap_or(v)
    }
}

impl Almanac {
    fn map_seed(&self, mut seed: u64) -> u64 {
        for map in &self.maps {
            seed = MapEntry::block_map(map, seed);
        }
        seed
    }
}

fn part1(input: &Almanac) -> u64 {
    input
        .seeds
        .iter()
        .copied()
        .map(|seed| input.map_seed(seed))
        .min()
        .unwrap()
}

fn part2(input: &Almanac) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn testcase1() {
        let input = parse_almanac(TEST_INPUT);
        eprintln!("{input:?}");
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn testcase2() {
        let input = parse_almanac(TEST_INPUT);
        assert_eq!(part2(&input), 30);
    }
}
