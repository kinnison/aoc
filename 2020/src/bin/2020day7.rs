use std::{cell::RefCell, convert::Infallible};

use aoc2020::*;

struct BagList(Vec<(usize, String)>);
#[derive(ParseByRegex)]
enum BagContent {
    #[regex = "no other bags"]
    NoOtherBags,
    #[regex = "(.+)"]
    SomeBags(BagList),
}
#[derive(ParseByRegex)]
#[regex = r"(?P<colour>.+?) bags contain (?P<content>.+)\."]
struct RawBagRule {
    colour: String,
    content: BagContent,
}

impl FromStr for BagList {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let mut ret = Vec::new();
        for bag in s.split(',') {
            let mut words = bag.split_ascii_whitespace();
            let num = words.next().unwrap().parse().unwrap();
            let colour = words.take(2).intersperse(" ").collect();
            ret.push((num, colour));
        }
        Ok(BagList(ret))
    }
}

struct BagRules {
    rules: HashMap<String, HashMap<String, usize>>,
    cache: RefCell<HashMap<String, HashMap<String, usize>>>,
}

impl From<Vec<RawBagRule>> for BagRules {
    fn from(value: Vec<RawBagRule>) -> Self {
        let mut ret = BagRules {
            rules: HashMap::new(),
            cache: RefCell::new(HashMap::new()),
        };
        for rule in value {
            let inner = match rule.content {
                BagContent::NoOtherBags => HashMap::new(),
                BagContent::SomeBags(v) => v.0.into_iter().map(|(n, s)| (s, n)).collect(),
            };
            ret.rules.insert(rule.colour, inner);
        }
        for col in ret.rules.keys() {
            ret.build_contains(col);
        }
        ret
    }
}

impl BagRules {
    fn build_contains(&self, bag: &str) {
        if self.cache.borrow().get(bag).is_some() {
            //println!("Already cached {}", bag);
            return;
        }
        let rule = self.rules.get(bag).unwrap();
        let mut cached: HashMap<String, usize> = HashMap::new();
        //println!("Populating {}", bag);
        //println!("Content: {:?}", rule);
        for rule in rule {
            // ensure cache populated
            self.build_contains(rule.0);
            // capture multiplications
            for sub in self.cache.borrow().get(rule.0).unwrap() {
                //println!("Adding {}*{} of {} to {}", sub.1, rule.1, sub.0, bag);
                *(cached.entry(sub.0.clone()).or_default()) += *sub.1 * *rule.1;
            }
            // capture bag itself
            //println!("Adding {} of {}", rule.1, rule.0);
            *(cached.entry(rule.0.clone()).or_default()) += rule.1;
        }
        //println!("Bag {} contains {:?}", bag, cached);
        // Insert the cached entry
        self.cache.borrow_mut().insert(bag.to_string(), cached);
    }

    fn contains(&self, bag: &str, target: &str) -> usize {
        if let Some(cache) = self.cache.borrow().get(bag) {
            cache.get(target).copied().unwrap_or(0)
        } else {
            0
        }
    }
}

fn part1(input: &BagRules) -> usize {
    let target = "shiny gold";
    input
        .rules
        .keys()
        .filter(|c| input.contains(c, target) > 0)
        .count()
}

fn part2(input: &BagRules) -> usize {
    let target = "shiny gold";
    input
        .cache
        .borrow()
        .get(target)
        .unwrap()
        .values()
        .copied()
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        let input = input.into();
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(
            r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.",
        )
        .unwrap();
        let input = input.into();
        assert_eq!(part2(&input), 126);
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec(7)?;
    let input = input.into();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
