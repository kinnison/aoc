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
            _ => unimplemented!(),
        }
    }

    fn is_match(&self, s: &str) -> bool {
        // We are implementing a backtracking solver...
        println!("!!! Trying to match {}", s);
        self.try_match(s, 0, 0) == Some("")
    }

    // Try to match the rule to the start of s, if successful
    // return the rest of s, otherwise None
    fn try_match<'a>(&self, s: &'a str, rule: usize, pfx: usize) -> Option<&'a str> {
        print!("{:width$}", "", width = pfx);
        println!("Trying to match {}: {:?}", rule, self.rules[&rule]);
        match &self.rules[&rule] {
            Rule::Terminal(c) => {
                if s.starts_with(*c) {
                    print!("{:width$}", "", width = pfx);
                    println!("Matched");
                    Some(&s[1..])
                } else {
                    print!("{:width$}", "", width = pfx);
                    println!("Nope");
                    None
                }
            }
            Rule::Recurse(alt) => {
                let mut alt_n = 0;
                loop {
                    if alt_n == alt.len() {
                        print!("{:width$}", "", width = pfx);
                        println!("Ran out of alts");
                        break None;
                    }
                    let alt = &alt[alt_n];
                    let mut rs = s;
                    let mut ok = true;
                    print!("{:width$}", "", width = pfx);
                    println!("Considering alt: {:?}", alt);
                    for rule in alt.iter().copied() {
                        if let Some(rest) = self.try_match(rs, rule, pfx + 1) {
                            rs = rest;
                        } else {
                            print!("{:width$}", "", width = pfx);
                            println!("Failed at rule {}", rule);
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        print!("{:width$}", "", width = pfx);
                        println!("Matched alt {}!", alt_n);
                        break Some(rs);
                    }
                    alt_n += 1;
                }
            }
            Rule::Repeat(n) => {
                let mut rs = s;
                let mut count = 0;
                while let Some(rest) = self.try_match(s, *n, pfx + 1) {
                    rs = rest;
                    count += 1;
                }
                if rs == s {
                    print!("{:width$}", "", width = pfx);
                    println!("Found none");
                    None
                } else {
                    print!("{:width$}", "", width = pfx);
                    println!("Found {}", count);
                    Some(rs)
                }
            }
            Rule::Matched(first, second) => {
                let mut rs = s;
                let mut count = 0;
                while let Some(rest) = self.try_match(s, *first, pfx + 1) {
                    rs = rest;
                    count += 1;
                }
                print!("{:width$}", "", width = pfx);
                println!("Found {} of opening", count);
                if count == 0 {
                    None
                } else {
                    let mut count2 = 0;
                    while let Some(rest) = self.try_match(s, *second, pfx + 1) {
                        rs = rest;
                        count2 += 1;
                    }
                    print!("{:width$}", "", width = pfx);
                    println!("Found {} of closing", count2);
                    if count == count2 {
                        Some(rs)
                    } else {
                        None
                    }
                }
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
    // operation, so we basically end up replacing those with custom matching
    // rules
    input.rules.rules.insert(8, Rule::Repeat(42));
    input.rules.rules.insert(11, Rule::Matched(42, 31));
    input
        .goals
        .iter()
        .filter(|s| {
            let b = input.rules.is_match(s);
            unreachable!();
            b
        })
        .count()
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
