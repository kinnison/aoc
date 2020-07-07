use aoc2015::*;

struct Input {
    repls: Vec<(String, String)>,
    molecule: Option<String>,
}

impl Input {
    fn from_str(input: &str) -> Input {
        lazy_static! {
            static ref PARSE: Regex = Regex::new("^([^ ]+) => (.+)$").unwrap();
        }
        let mut ret = Input {
            repls: Vec::new(),
            molecule: None,
        };
        for line in input.lines() {
            if line != "" {
                if let Some(cap) = PARSE.captures(line) {
                    let from = cap.get(1).unwrap().as_str().to_owned();
                    let res = cap.get(2).unwrap().as_str().to_owned();
                    assert!(res.len() >= from.len());
                    ret.repls.push((from, res));
                } else {
                    ret.molecule = Some(line.to_owned())
                }
            }
        }
        assert!(ret.molecule.is_some() && ret.repls.len() > 0);
        ret
    }

    fn get_distinct(&self, mole: &str) -> HashSet<String> {
        let mut found = HashSet::new();
        for (ref from, ref repl) in self.repls.iter() {
            for (idx, _) in mole.match_indices(from) {
                let mut new = String::new();
                new.push_str(&mole[0..idx]);
                new.push_str(repl);
                new.push_str(&mole[(idx + from.len())..]);
                found.insert(new);
            }
        }
        found
    }

    fn get_to_e(&self, mole: &str) -> usize {
        let mut ret = mole.to_owned();
        let mut steps = 0;
        while ret != "e" {
            for (ref from, ref repl) in self.repls.iter() {
                if let Some(idx) = ret.find(repl) {
                    let mut new = String::new();
                    new.push_str(&ret[0..idx]);
                    new.push_str(from);
                    new.push_str(&ret[(idx + repl.len())..]);
                    ret = new;
                    steps += 1;
                }
            }
        }
        steps
    }
}

fn part1(input: &Input) -> usize {
    if let Some(ref mole) = input.molecule {
        input.get_distinct(mole).len()
    } else {
        unreachable!()
    }
}

/* Theoretically possible, but frankly takes too long */
/*
fn part2_(
    input: &Input,
    target: &str,
    replacements: usize,
    sofar: &HashSet<String>,
    tried: &mut HashSet<String>,
) -> usize {
    let mut step = HashSet::new();
    println!(
        "Considering {} steps with {} inputs having already tried {} molecules",
        replacements,
        sofar.len(),
        tried.len()
    );
    for mole in sofar.iter() {
        for found in input.get_distinct(mole) {
            if found.len() > target.len() || tried.contains(&found) {
                println!("SKIP");
            } else {
                step.insert(found);
            }
        }
        tried.insert(mole.clone());
    }
    assert!(step.len() > 0);
    if step.contains(target) {
        replacements
    } else {
        part2_(input, target, replacements + 1, &step, tried)
    }
}

fn part2(input: &Input) -> usize {
    if let Some(ref mole) = input.molecule {
        let mut e = HashSet::new();
        e.insert("e".to_owned());
        part2_(input, mole, 1, &e, &mut HashSet::new())
    } else {
        unreachable!()
    }
}
*/

fn part2(input: &Input) -> usize {
    if let Some(ref mole) = input.molecule {
        input.get_to_e(mole)
    } else {
        unreachable!()
    }
}

fn main() -> Result<()> {
    let test_input1 = Input::from_str("H => HO\nH => OH\nO => HH\ne => H\ne => O\n\nHOH");
    let test_input2 = Input::from_str("H => HO\nH => OH\nO => HH\ne => H\ne => O\n\nHOHOHO");
    println!("Test1 1: {}", part1(&test_input1));
    println!("Test2 1: {}", part1(&test_input2));
    let input = Input::from_str(&read_input(19)?);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
