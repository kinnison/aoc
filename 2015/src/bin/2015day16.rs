use aoc2015::*;

struct Sue {
    sue: usize,
    facts: HashMap<String, usize>,
}

impl Sue {
    fn from_str(input: &str) -> Sue {
        lazy_static! {
            static ref PARSE: Regex = Regex::new(
                "^Sue ([0-9]+): ([^:]+): ([0-9]+), ([^:]+): ([0-9]+), ([^:]+): ([0-9]+)$"
            )
            .unwrap();
        }
        if let Some(cap) = PARSE.captures(input) {
            let sue = cap.get(1).unwrap().as_str().parse().unwrap();
            let mut ret = Sue {
                sue,
                facts: HashMap::new(),
            };
            for fact in 0..3 {
                let fac = cap.get(2 + (fact * 2)).unwrap().as_str().to_owned();
                let val = cap.get(3 + (fact * 2)).unwrap().as_str().parse().unwrap();
                ret.facts.insert(fac, val);
            }
            ret
        } else {
            panic!("Unable to parse: {}", input);
        }
    }

    fn matches(&self, factset: &HashMap<String, usize>) -> bool {
        let mut okay = true;
        for (fact, amount) in factset.iter() {
            if let Some(value) = self.facts.get(fact) {
                if value != amount {
                    okay = false
                }
            }
        }
        okay
    }

    fn matches2(&self, factset: &HashMap<String, usize>) -> bool {
        let mut okay = true;
        for (fact, amount) in factset.iter() {
            if let Some(value) = self.facts.get(fact) {
                if fact == "cats" || fact == "trees" {
                    if value <= amount {
                        okay = false
                    }
                } else if fact == "pomeranians" || fact == "goldfish" {
                    if value >= amount {
                        okay = false
                    }
                } else if value != amount {
                    okay = false
                }
            }
        }
        okay
    }
}

fn reading() -> HashMap<String, usize> {
    let mut known = HashMap::new();
    known.insert("children".to_owned(), 3);
    known.insert("cats".to_owned(), 7);
    known.insert("samoyeds".to_owned(), 2);
    known.insert("pomeranians".to_owned(), 3);
    known.insert("akitas".to_owned(), 0);
    known.insert("vizslas".to_owned(), 0);
    known.insert("goldfish".to_owned(), 5);
    known.insert("trees".to_owned(), 3);
    known.insert("cars".to_owned(), 2);
    known.insert("perfumes".to_owned(), 1);
    known
}

fn part1(input: &[Sue]) -> usize {
    let known = reading();
    for sue in input.iter() {
        if sue.matches(&known) {
            return sue.sue;
        }
    }
    unreachable!()
}

fn part2(input: &[Sue]) -> usize {
    let known = reading();
    for sue in input.iter() {
        if sue.matches2(&known) {
            return sue.sue;
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let input: Vec<Sue> = read_input(16)?.lines().map(Sue::from_str).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
