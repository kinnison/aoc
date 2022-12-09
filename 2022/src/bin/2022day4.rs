use aoc2022::*;

#[derive(ParseByRegex, Clone, Copy)]
#[regex = r"(?P<lower>[0-9]+)\-(?P<upper>[0-9]+)"]
struct Elf {
    lower: usize,
    upper: usize,
}

impl Elf {
    fn within(&self, other: &Elf) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }

    fn overlap(&self, other: &Elf) -> bool {
        self.lower <= other.upper && self.upper >= other.lower
    }
}

#[derive(ParseByRegex, Clone, Copy)]
#[regex = r"(?P<first>[^,]+),(?P<second>.*)"]
struct Pairing {
    first: Elf,
    second: Elf,
}

impl Pairing {
    fn wasted(&self) -> bool {
        self.first.within(&self.second) || self.second.within(&self.first)
    }

    fn overlapped(&self) -> bool {
        self.first.overlap(&self.second)
    }
}

fn part1(input: &[Pairing]) -> usize {
    input.iter().copied().filter(Pairing::wasted).count()
}

fn part2(input: &[Pairing]) -> usize {
    input.iter().copied().filter(Pairing::overlapped).count()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn testcase1() {
        let input: Vec<Pairing> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Pairing> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 4);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<Pairing> = read_input_as_vec(4)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
