use std::convert::Infallible;

use aoc2020::*;

struct CustomsForm {
    answers: HashMap<char, usize>,
    people: usize,
}

impl FromStr for CustomsForm {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(CustomsForm::load(s))
    }
}

impl CustomsForm {
    fn load(input: &str) -> Self {
        let mut ret = Self {
            answers: HashMap::new(),
            people: input.lines().count(),
        };
        for ch in input.chars() {
            match ch {
                'a'..='z' => *(ret.answers.entry(ch).or_default()) += 1,
                '\n' => {}
                _ => panic!("What to do with '{}'", ch),
            }
        }
        ret
    }

    fn count_keys(&self) -> usize {
        self.answers.keys().count()
    }

    fn count_everyone(&self) -> usize {
        self.answers
            .iter()
            .filter_map(|(_, p)| if *p == self.people { Some(()) } else { None })
            .count()
    }
}

fn part1(input: &[CustomsForm]) -> usize {
    input.iter().map(|f| f.count_keys()).sum()
}

fn part2(input: &[CustomsForm]) -> usize {
    input.iter().map(|f| f.count_everyone()).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn testcase1() {
        let input: Vec<_> = input_by_split_pat(TEST_INPUT, "\n\n").unwrap();
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn testcase2() {
        let input: Vec<_> = input_by_split_pat(TEST_INPUT, "\n\n").unwrap();
        assert_eq!(part2(&input), 6);
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec_split(6, "\n\n")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
