use aoc2015::*;

#[derive(Clone)]
struct HappyTracker {
    changes: HashMap<String, HashMap<String, i32>>,
}

impl HappyTracker {
    fn from_str(input: &str) -> HappyTracker {
        lazy_static! {
            static ref PARSE: Regex = Regex::new("^([^ ]+) would (gain|lose) ([0-9]+) happiness units by sitting next to ([^.]+)\\.$").unwrap();
        };
        let mut ret = HappyTracker {
            changes: HashMap::new(),
        };
        for line in input.lines() {
            if let Some(cap) = PARSE.captures(line) {
                let who = cap.get(1).unwrap().as_str().to_owned();
                let by = cap.get(4).unwrap().as_str().to_owned();
                let mut amt: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
                if cap.get(2).unwrap().as_str().chars().next().unwrap() == 'l' {
                    amt = -amt;
                }
                (*ret.changes.entry(who).or_insert(HashMap::new())).insert(by, amt);
            } else {
                panic!("Unable to parse: {}", line);
            }
        }
        ret
    }

    fn all_names(&self) -> Vec<&String> {
        self.changes.keys().collect()
    }

    fn happiness_of(&self, layout: &Vec<&String>) -> i32 {
        layout
            .iter()
            .cycle()
            .take(layout.len() + 1)
            .tuple_windows()
            .map(|(&a, &b)| {
                self.changes.get(a).unwrap().get(b).unwrap()
                    + self.changes.get(b).unwrap().get(a).unwrap()
            })
            .sum()
    }

    fn add_myself(&mut self) {
        let others: Vec<String> = self.changes.keys().map(|s| s.clone()).collect();
        self.changes.insert("@".to_owned(), HashMap::new());
        for other in others.into_iter() {
            self.changes
                .get_mut(&other)
                .unwrap()
                .insert("@".to_owned(), 0);
            self.changes.get_mut("@").unwrap().insert(other, 0);
        }
    }
}

fn part1(input: &HappyTracker) -> i32 {
    let mut best = std::i32::MIN;
    let mut names = input.all_names();
    for perm in Heap::new(&mut names) {
        let permscore = input.happiness_of(&perm);
        if permscore > best {
            best = permscore;
        }
    }
    best
}

fn part2(input: &HappyTracker) -> i32 {
    let mut repl = input.clone();
    repl.add_myself();
    part1(&repl)
}

fn main() -> Result<()> {
    let input = read_input(13)?;
    let input = HappyTracker::from_str(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
