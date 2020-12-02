use aoc2020::*;

#[derive(ParseByRegex)]
#[regex = r"^(?P<min>[0-9]+)-(?P<max>[0-9]+) (?P<letter>.): (?P<password>.+)$"]
struct TobogganEntry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl TobogganEntry {
    fn is_valid1(&self) -> bool {
        // Passwords are valid if the have at least min and at most max chars
        // which match letter
        let matching = self.password.chars().filter(|&c| c == self.letter).count();
        self.min <= matching && matching <= self.max
    }

    fn is_valid2(&self) -> bool {
        // Passwords are valid if positions min and max contain exactly one of
        // letter.  Wrinkle: '1' means 0th character
        let minch = self.password.chars().nth(self.min - 1).unwrap();
        let maxch = self.password.chars().nth(self.max - 1).unwrap();
        (minch == self.letter || maxch == self.letter) && minch != maxch
    }
}

fn part1(input: &[TobogganEntry]) -> usize {
    input.iter().filter(|p| p.is_valid1()).count()
}

fn part2(input: &[TobogganEntry]) -> usize {
    input.iter().filter(|p| p.is_valid2()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 1);
    }
}

fn main() -> Result<()> {
    let input: Vec<TobogganEntry> = read_input_as_vec(2)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
