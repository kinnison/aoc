use aoc2022::*;

struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        assert!((s.len() % 2) == 0);
        let each = s.len() >> 1;
        let first = s.chars().take(each).collect();
        let second = s.chars().skip(each).collect();
        Ok(Self { first, second })
    }
}

impl Rucksack {
    fn prio(c: char) -> u64 {
        match c as u8 {
            v @ b'a'..=b'z' => (v - b'a' + 1) as u64,
            v @ b'A'..=b'Z' => (v - b'A' + 27) as u64,
            _ => unreachable!(),
        }
    }

    fn overlap(&self) -> impl Iterator<Item = char> + '_ {
        self.first.intersection(&self.second).copied()
    }

    fn all_items(&self) -> HashSet<char> {
        self.first.union(&self.second).copied().collect()
    }
}

fn part1(input: &[Rucksack]) -> u64 {
    input
        .iter()
        .map(|rs| rs.overlap().map(Rucksack::prio).sum::<u64>())
        .sum()
}

fn part2(input: &[Rucksack]) -> u64 {
    input
        .iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(Rucksack::all_items)
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .unwrap()
                .into_iter()
                .map(Rucksack::prio)
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn testcase1() {
        let input: Vec<Rucksack> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Rucksack> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 70);
    }
}

fn main() -> Result<()> {
    let input: Vec<Rucksack> = read_input_as_vec(3)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
