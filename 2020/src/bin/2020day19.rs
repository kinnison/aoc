use aoc2020::*;

enum Rule {
    Terminal(char),
    Recurse(Vec<Vec<usize>>),
}

struct Ruleset {
    rules: HashMap<usize, Rule>,
}

impl FromStr for Ruleset {
    type Err = GenError;

    fn from_str(value: &str) -> Result<Self> {
        let mut rules = HashMap::new();
        for l in value.trim().lines() {
            let l = l.trim();
            let colon = l.find(':').ok_or("no colon")?;
            let rulenum = l[..colon].trim();
            let rulenum = rulenum.parse()?;
            let rule = if let Some(quote) = l.find('"') {
                Rule::Terminal(l.chars().nth(quote + 1).ok_or("no chars?")?)
            } else {
                let rest = l[colon + 1..].trim();
                let alts = rest.split('|');
                let alts = alts.map(|s| {
                    s.trim()
                        .split_ascii_whitespace()
                        .map(|n| n.parse::<usize>())
                        .collect::<StdResult<_, _>>()
                });
                let alts: StdResult<Vec<_>, _> = alts.collect();
                let alts = alts?;
                Rule::Recurse(alts)
            };
            rules.insert(rulenum, rule);
        }
        Ok(Self { rules })
    }
}

impl Ruleset {
    fn regex_string(&self) -> String {
        let mut ret = String::new();
        ret.push('^');
        self._regex_string(&mut ret, 0);
        ret.push('$');
        ret
    }

    fn _regex_string(&self, acc: &mut String, rule: usize) {
        match self.rules.get(&rule).unwrap() {
            Rule::Terminal(c) => acc.push(*c),
            Rule::Recurse(alts) => {
                let alts = alts.iter().map(|subs| {
                    let mut alt = String::new();
                    subs.iter()
                        .copied()
                        .for_each(|n| self._regex_string(&mut alt, n));
                    alt
                });
                acc.push_str("(?:");
                alts.for_each(|s| {
                    acc.push_str(&s);
                    acc.push('|');
                });
                acc.pop(); // remove last |
                acc.push(')');
            }
        }
    }
}

struct Puzzle {
    rules: Ruleset,
    goals: Vec<String>,
}

impl FromStr for Puzzle {
    type Err = GenError;

    fn from_str(value: &str) -> Result<Self> {
        let pos = value.find("\n\n").ok_or("no split")?;
        let (rules, goals) = value.split_at(pos);
        let rules = rules.trim();
        let rules = rules.parse()?;
        let goals = goals.trim().lines().map(str::to_string).collect();
        Ok(Self { rules, goals })
    }
}

fn part1(input: &Puzzle) -> usize {
    // Build a regexp representing the puzzle and then test all the goals
    let rex = input.rules.regex_string();
    let rex = Regex::new(&rex).unwrap();
    input.goals.iter().filter(|s| rex.is_match(s)).count()
}

fn part2(input: &Puzzle) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn testcase1() {
        let input = TEST_INPUT.parse().unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn testcase2() {}
}

fn main() -> Result<()> {
    let input: String = read_input(19)?;
    let input = input.parse()?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
