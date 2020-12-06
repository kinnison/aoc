use aoc2020::*;

struct CustomsForm {
    answers: HashMap<char, usize>,
    people: usize,
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
        let input: Vec<_> = TEST_INPUT
            .split("\n\n")
            .map(|s| CustomsForm::load(s))
            .collect();
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn testcase2() {
        let input: Vec<_> = TEST_INPUT
            .split("\n\n")
            .map(|s| CustomsForm::load(s))
            .collect();
        assert_eq!(part2(&input), 6);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(6)?;
    let input: Vec<_> = input.split("\n\n").map(|s| CustomsForm::load(s)).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
