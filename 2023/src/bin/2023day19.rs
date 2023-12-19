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
    let mut heads = Vec::new();
    heads.push(PathHead::default());
    let mut finished = Vec::new();

    while !heads.is_empty() {
        for mut head in std::mem::take(&mut heads) {
            if head.loc == "A" {
                finished.push(head);
                continue;
            } else if head.loc == "R" {
                // Rejected
                continue;
            }
            let flow = &input.workflows[head.loc];

            for rule in &*flow.rules {
                match rule {
                    Rule::Conditional {
                        attr,
                        operator,
                        value,
                        target,
                    } => {
                        let (mut at_min_pass, mut at_most_pass) = head.constraints[attr];
                        let (mut at_min_skip, mut at_most_skip) = head.constraints[attr];
                        match operator {
                            Op::LessThan => {
                                // attr < value to pass means that
                                // to pass, we need at_most_pass to be dropped
                                at_most_pass = at_most_pass.min((*value) - 1);
                                // But to skip, we need to be more
                                at_min_skip = at_min_skip.max(*value);
                            }
                            Op::GreaterThan => {
                                // attr > value to pass means that
                                // to pass, we need at least value
                                at_min_pass = at_min_pass.max((*value) + 1);
                                // to skip, it must be less
                                at_most_skip = at_most_skip.min(*value);
                            }
                        }
                        if at_min_pass <= at_most_pass {
                            // At least one option still exists for this attribute
                            // so pass on to the next requirement
                            let mut pass = head.clone();
                            pass.constraints.insert(*attr, (at_min_pass, at_most_pass));
                            pass.loc = target.as_str();
                            heads.push(pass);
                        }
                        if at_min_skip <= at_most_skip {
                            // At least one option still exists for this attribute
                            // so move on to the next part of the workflow
                            head.constraints.insert(*attr, (at_min_skip, at_most_skip));
                        } else {
                            // No option remains for this workflow, so move on
                            break;
                        }
                    }
                    Rule::Unconditional { target } => {
                        head.loc = target.as_str();
                        heads.push(head);
                        break;
                    }
                }
            }
        }
    }

    let mut tot = 0;

    for head in finished {
        let mut possibilities = 1;
        for (_, (min, max)) in head.constraints {
            possibilities *= (max - min) + 1;
        }
        tot += possibilities;
    }

    tot
}

#[derive(Clone, Debug)]
struct PathHead<'a> {
    loc: &'a str,
    constraints: HashMap<Attribute, (i64, i64)>,
}

impl Default for PathHead<'_> {
    fn default() -> Self {
        let mut constraints = HashMap::new();
        constraints.insert(Attribute::ExtremelyCoolLooking, (1, 4000));
        constraints.insert(Attribute::Musical, (1, 4000));
        constraints.insert(Attribute::Aerodynamic, (1, 4000));
        constraints.insert(Attribute::Shiny, (1, 4000));
        Self {
            loc: "in",
            constraints,
        }
    }
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

#[derive(Debug, ParseByRegex, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
        assert_eq!(part2(&input), 167409079868000);
    }
}
