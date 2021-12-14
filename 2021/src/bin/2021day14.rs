use aoc2021::*;

#[derive(ParseByRegex)]
#[regex = r"^(?P<a>.)(?P<b>.) -> (?P<i>.)$"]
struct Rule {
    a: char,
    b: char,
    i: char,
}

struct Input {
    template: String,
    rules: HashMap<(char, char), char>,
}

impl FromStr for Input {
    type Err = GenError;

    fn from_str(input: &str) -> Result<Self> {
        let mut lines = input.trim().lines().map(str::trim);
        let template = lines.next().unwrap().to_string();
        assert_eq!(lines.next().unwrap(), "");
        let mut rules = HashMap::new();
        for rule in lines {
            let rule = Rule::parse_by_regex(rule)?;
            rules.insert((rule.a, rule.b), rule.i);
        }
        Ok(Self { template, rules })
    }
}

fn insertion_step(input: &str, rules: &HashMap<(char, char), char>) -> String {
    std::iter::once(input.chars().next().unwrap())
        .chain(
            input
                .chars()
                .tuple_windows()
                .flat_map(|(a, b)| [rules[&(a, b)], b]),
        )
        .collect()
}

fn part1(input: &Input) -> usize {
    let mut polymer = input.template.clone();
    println!("Template: {}", polymer);
    for i in 0..10 {
        polymer = insertion_step(&std::mem::take(&mut polymer), &input.rules);
        println!("After step {}: {} tokens", i + 1, polymer.len());
    }
    let mut counts = HashMap::new();
    polymer
        .chars()
        .for_each(|c| *counts.entry(c).or_default() += 1);
    let mut counts: Vec<(usize, char)> = counts.into_iter().map(|(c, n)| (n, c)).collect();
    counts.sort_unstable();
    counts[counts.len() - 1].0 - counts[0].0
}

fn part2(input: &Input) -> u64 {
    // Since the actual ordering of the polymer does not matter, let's just consider the
    // pairs
    let mut counts: HashMap<(char, char), u64> = HashMap::new();
    input
        .template
        .chars()
        .tuple_windows()
        .for_each(|(a, b)| *counts.entry((a, b)).or_default() += 1);
    // repeat 40 times
    for _ in 0..40 {
        let oldcounts = std::mem::take(&mut counts);
        for ((a, b), n) in oldcounts {
            let i = input.rules[&(a, b)];
            *counts.entry((a, i)).or_default() += n;
            *counts.entry((i, b)).or_default() += n;
        }
    }
    // Now count the chars
    let mut ccounts: HashMap<char, u64> = HashMap::new();
    counts.into_iter().for_each(|((a, _), n)| {
        *ccounts.entry(a).or_default() += n;
    });
    // Because we only counted the first char of each pair, also we need the last character of the original template
    *ccounts
        .entry(input.template.chars().last().unwrap())
        .or_default() += 1;
    let mut ccounts: Vec<(u64, char)> = ccounts.into_iter().map(|(c, n)| (n, c)).collect();
    ccounts.sort_unstable();
    ccounts[ccounts.len() - 1].0 - ccounts[0].0
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn testcase1() {
        let input = Input::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 1588);
    }

    #[test]
    fn testcase2() {
        let input = Input::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 2188189693529);
    }
}

fn main() -> Result<()> {
    let input = read_input(14)?;
    let input = Input::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
