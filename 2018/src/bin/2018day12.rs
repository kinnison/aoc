use aoc2018::*;

#[derive(ParseByRegex)]
#[regex = "^(?P<conds>.....) => (?P<result>.)$"]
struct RawRule {
    conds: String,
    result: char,
}

struct Rule {
    conds: [bool; 5],
    result: bool,
}

impl Rule {
    fn from_raw(raw: &RawRule) -> Rule {
        let conds: Vec<bool> = raw.conds.bytes().map(|b| b == b'#').collect();
        Rule {
            conds: [conds[0], conds[1], conds[2], conds[3], conds[4]],
            result: raw.result == '#',
        }
    }

    fn matches(&self, window: [bool; 5]) -> bool {
        self.conds == window
    }
}

struct Flowerpots {
    pots: HashSet<i32>,
    minused: i32,
    maxused: i32,
}

impl Flowerpots {
    fn from_input(input: &str) -> Flowerpots {
        // Input is of the form 'initial state: ......'
        let mut pots = HashSet::new();
        let mut minused = std::i32::MAX;
        let mut maxused = std::i32::MIN;
        for (i, present) in input
            .bytes()
            .filter(|&b| b == b'.' || b == b'#')
            .map(|b| b == b'#')
            .enumerate()
        {
            if present {
                pots.insert(i as i32);
            }
            minused = min(minused, i as i32);
            maxused = max(maxused, i as i32);
        }
        Flowerpots {
            pots,
            minused,
            maxused,
        }
    }

    fn tick(&mut self, rules: &[Rule]) {
        let mut newpots = HashSet::new();
        let mut window: [bool; 5] = [false; 5];
        let mut minused = std::i32::MAX;
        let mut maxused = std::i32::MIN;
        for i in self.minused - 2..=self.maxused + 2 {
            window[0] = window[1];
            window[1] = window[2];
            window[2] = window[3];
            window[3] = window[4];
            window[4] = self.pots.contains(&(i + 2));
            for rule in rules {
                if rule.matches(window) {
                    if rule.result {
                        newpots.insert(i);
                        minused = min(minused, i);
                        maxused = max(maxused, i);
                    }
                    break;
                }
            }
        }
        self.pots = newpots;
        self.minused = minused;
        self.maxused = maxused;
    }

    fn plantsum(&self) -> i32 {
        self.pots.iter().sum()
    }
}

static TEST_INPUT: &str = r"
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";

fn part1(start: &str, rules: &[Rule]) -> i32 {
    let mut pots = Flowerpots::from_input(start);
    for _i in 0..20 {
        pots.tick(rules);
    }
    pots.plantsum()
}

fn part2(start: &str, rules: &[Rule]) -> i64 {
    let mut pots = Flowerpots::from_input(start);
    let mut lastscore = pots.plantsum();
    let mut lastdelta = 0;
    let mut lastlastdelta = 1;
    // observation has shown us that we reach a steady state eventually where
    // lastdelta is the same as delta is the same as lastlastdelta
    for _i in 1.. {
        pots.tick(rules);
        let score = pots.plantsum();
        let delta = score - lastscore;
        if delta == lastdelta && lastdelta == lastlastdelta {
            println!("Steady state after {} ticks.  Delta is {}", _i, delta);
            // Now that we've reached steady state, we know that we've done
            // _i generations so we need to make the rest up to 50billion
            // by adding delta times the remaining generations
            let ticksleft: i64 = 50_000_000_000 - _i;
            let finalscore = i64::from(score) + (ticksleft * i64::from(delta));
            println!("As a result, there are {} ticks left", ticksleft);
            return finalscore;
        }
        lastlastdelta = lastdelta;
        lastdelta = delta;
        lastscore = score;
    }

    0
}

fn main() -> Result<()> {
    let (test_rawrules, test_startline): (Vec<RawRule>, String) =
        input_as_vec_and_first(TEST_INPUT)?;
    let test_rules: Vec<Rule> = test_rawrules.iter().map(Rule::from_raw).collect();
    println!("Test 1: {}", part1(&test_startline, &test_rules));
    let (input_rawrules, input_startline): (Vec<RawRule>, String) =
        read_input_as_vec_and_first(12)?;
    let input_rules: Vec<Rule> = input_rawrules.iter().map(Rule::from_raw).collect();
    println!("Part 1: {}", part1(&input_startline, &input_rules));
    println!("Part 2: {}", part2(&input_startline, &input_rules));
    Ok(())
}
