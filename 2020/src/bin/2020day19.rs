use std::cell::RefCell;

use aoc2020::*;

#[derive(Debug)]
enum Rule {
    Terminal(char),
    Recurse(Vec<Vec<usize>>),
    Repeat(usize),
    Matched(usize, usize),
}

struct Ruleset {
    rules: HashMap<usize, Rule>,
    min_lens: RefCell<HashMap<usize, usize>>,
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
        let ret = Self {
            rules,
            min_lens: RefCell::new(HashMap::new()),
        };

        ret.compute_min_len(0);

        Ok(ret)
    }
}

impl Ruleset {
    fn regex_string(&self, maxlen: usize) -> String {
        let mut ret = String::new();
        ret.push('^');
        self._regex_string(&mut ret, 0, maxlen);
        ret.push('$');
        ret
    }

    fn compute_min_len(&self, rule: usize) -> usize {
        if self.min_lens.borrow().contains_key(&rule) {
            return *self.min_lens.borrow().get(&rule).unwrap();
        }
        let len = match self.rules.get(&rule).unwrap() {
            Rule::Terminal(_) => 1,
            Rule::Recurse(alts) => alts
                .iter()
                .map(|alt| alt.iter().map(|r| self.compute_min_len(*r)).sum())
                .max()
                .unwrap(),
            Rule::Repeat(sub) => self.compute_min_len(*sub),
            Rule::Matched(first, second) => {
                self.compute_min_len(*first) + self.compute_min_len(*second)
            }
        };
        self.min_lens.borrow_mut().insert(rule, len);
        len
    }

    fn min_len_of(&self, rule: usize) -> usize {
        *self.min_lens.borrow().get(&rule).unwrap()
    }

    fn _regex_string(&self, acc: &mut String, rule: usize, maxlen: usize) {
        match self.rules.get(&rule).unwrap() {
            Rule::Terminal(c) => acc.push(*c),
            Rule::Recurse(alts) => {
                let alts = alts.iter().map(|subs| {
                    let mut alt = String::new();
                    subs.iter()
                        .copied()
                        .for_each(|n| self._regex_string(&mut alt, n, maxlen));
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
            Rule::Repeat(n) => {
                acc.push_str("(?:");
                self._regex_string(acc, *n, maxlen);
                acc.push_str(")+");
            }
            Rule::Matched(first, second) => {
                let mut first_str = String::new();
                self._regex_string(&mut first_str, *first, maxlen);
                let mut second_str = String::new();
                self._regex_string(&mut second_str, *second, maxlen);
                acc.push_str("(?:");
                let totlen = self.min_len_of(*first) + self.min_len_of(*second);
                let maxrep = (maxlen + totlen - 1) / totlen;
                for n in 1..=maxrep {
                    for _ in 1..=n {
                        acc.push_str(&first_str)
                    }
                    for _ in 1..=n {
                        acc.push_str(&second_str)
                    }
                    acc.push('|');
                }
                acc.pop(); //last |
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
    let maxlen = input.goals.iter().map(|s| s.len()).max().unwrap();
    let rex = input.rules.regex_string(maxlen);
    let rex = RegexBuilder::new(&rex)
        .size_limit(104857600)
        .build()
        .unwrap();
    input.goals.iter().filter(|s| rex.is_match(s)).count()
}

fn part2(mut input: Puzzle) -> usize {
    // replace:
    // 8: 42
    // with:
    // 8: 42 | 42 8
    // replace:
    // 11: 42 31
    // with:
    // 11: 42 31 | 42 11 31
    // The 8 rule becomes 8+ which is pretty simple to encode in a regex
    // but the 11 rule becomes a counting pattern which is more of a parsing
    // operation, so we basically end up replacing those with custom expressions
    // which cannot exceed the length of a given input
    input.rules.rules.insert(8, Rule::Repeat(42));
    input.rules.rules.insert(11, Rule::Matched(42, 31));
    part1(&input)
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
    fn testcase2() {
        let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
            .parse()
            .unwrap();
        assert_eq!(part1(&input), 3);
        assert_eq!(part2(input), 12);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(19)?;
    let input = input.parse()?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(input));
    Ok(())
}
