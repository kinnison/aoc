use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(19)?;
    let input = PartSorter::from_str(&input)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &PartSorter) -> i64 {
    input
        .parts
        .iter()
        .filter(|part| input.accept_part(part))
        .map(|p| p.attrs.values().copied().sum::<i64>())
        .sum()
}

fn part2(input: &PartSorter) -> i64 {
    todo!()
}

#[derive(Debug)]
struct PartSorter {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl PartSorter {
    fn accept_part(&self, part: &Part) -> bool {
        let mut pos = "in";
        loop {
            let flow = &self.workflows[pos];
            pos = flow.filter(part);
            match pos {
                "A" => break true,
                "R" => break false,
                _ => {}
            }
        }
    }
}

impl FromStr for PartSorter {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // Step one, split into the workflows and parts
        let (workflows, parts) = s.split_once("\n\n").unwrap();
        let workflows: Vec<Workflow> = input_as_vec(workflows).unwrap();
        let parts: Vec<Part> = input_as_vec(parts).unwrap();

        Ok(Self {
            workflows: workflows.into_iter().map(|w| (w.name.clone(), w)).collect(),
            parts,
        })
    }
}

#[derive(Debug, ParseByRegex)]
#[regex = r"(?P<name>[^\{]+)\{(?P<rules>[^\}]+)\}"]
struct Workflow {
    name: String,
    rules: Rules,
}

impl Workflow {
    fn filter(&self, part: &Part) -> &str {
        for rule in &*self.rules {
            if rule.applies(part) {
                return rule.target();
            }
        }
        unreachable!()
    }
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
}

impl FromStr for Rules {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rules = s
            .split(',')
            .map(|r| Rule::parse_by_regex(r).unwrap())
            .collect_vec();

        Ok(Self { rules })
    }
}

impl Deref for Rules {
    type Target = [Rule];

    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}

#[derive(Debug, ParseByRegex)]
enum Rule {
    #[regex = r"(?P<attr>.)(?P<operator>.)(?P<value>\d+):(?P<target>.+)"]
    Conditional {
        attr: Attribute,
        operator: Op,
        value: i64,
        target: String,
    },
    #[regex = r"(?P<target>.+)"]
    Unconditional { target: String },
}

impl Rule {
    fn applies(&self, part: &Part) -> bool {
        match self {
            Rule::Conditional {
                attr,
                operator,
                value,
                ..
            } => {
                let pval = part.attrs[attr];
                match operator {
                    Op::GreaterThan => pval > *value,
                    Op::LessThan => pval < *value,
                }
            }
            Rule::Unconditional { .. } => true,
        }
    }

    fn target(&self) -> &str {
        match self {
            Rule::Conditional { target, .. } => target,
            Rule::Unconditional { target } => target,
        }
    }
}

#[derive(Debug, ParseByRegex, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Attribute {
    #[regex = r"x"]
    ExtremelyCoolLooking,
    #[regex = r"m"]
    Musical,
    #[regex = r"a"]
    Aerodynamic,
    #[regex = r"s"]
    Shiny,
}

#[derive(Debug, ParseByRegex)]
enum Op {
    #[regex = r"<"]
    LessThan,
    #[regex = r">"]
    GreaterThan,
}

#[derive(Debug)]
struct Part {
    attrs: HashMap<Attribute, i64>,
}

impl FromStr for Part {
    type Err = Infallible;

    fn from_str(mut s: &str) -> std::result::Result<Self, Self::Err> {
        s = s.strip_prefix('{').unwrap();
        s = s.strip_suffix('}').unwrap();

        let attrs = s
            .split(',')
            .map(|e| AttrAssign::parse_by_regex(e).unwrap())
            .map(|aa| (aa.attr, aa.value))
            .collect();

        Ok(Self { attrs })
    }
}

#[derive(Debug, ParseByRegex)]
#[regex = r"(?P<attr>.)=(?P<value>\d+)"]
struct AttrAssign {
    attr: Attribute,
    value: i64,
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

    #[test]
    fn testcase1() {
        let input = PartSorter::from_str(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 19114);
    }

    #[test]
    fn testcase2() {
        let input = PartSorter::from_str(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 0);
    }
}
