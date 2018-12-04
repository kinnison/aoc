use aoc2018::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Action {
    WakeUp,
    FallAsleep,
    BeginShift(usize),
}

use self::Action::*;

impl Action {
    fn from_str(input: &str) -> Result<Action> {
        let mut splitter = input.split_whitespace();
        let word = splitter.next().ok_or("No word?")?;
        Ok(match word {
            "wakes" => WakeUp,
            "falls" => FallAsleep,
            _ => {
                let hash = splitter.next().ok_or("No Guard?")?;
                BeginShift(hash[1..].parse()?)
            }
        })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Entry {
    when: DateTime<Utc>,
    pub what: Action,
}

static DATEFMT: &str = "%Y-%m-%d %H:%M";

impl Entry {
    fn from_str<T: AsRef<str>>(input: T) -> Result<Entry> {
        lazy_static! {
            static ref PARSE: Regex =
                Regex::new("^\\[([^\\]]+)\\](.*)$").expect("Unable to compile regular expression");
        }
        if let Some(cap) = PARSE.captures(input.as_ref()) {
            let when = cap.get(1).ok_or("No datetime?")?.as_str();
            let when = Utc.datetime_from_str(when, DATEFMT)?;
            let what = cap.get(2).ok_or("No action?")?.as_str();
            let what = Action::from_str(what)?;
            Ok(Entry { when, what })
        } else {
            Err(format!("Unable to parse {}", input.as_ref()))?
        }
    }

    fn minute(&self) -> usize {
        self.when.minute() as usize
    }
}

static TEST_INPUT: &str = r#"
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;

fn sorted_from_str<T: AsRef<str>>(input: T) -> Result<Vec<Entry>> {
    let res: Result<Vec<Entry>> = input.as_ref().trim().lines().map(Entry::from_str).collect();
    let mut res = res?;
    res.sort();
    Ok(res)
}

struct Guard {
    sleepings: [u32; 60],
    slept_for: usize,
}

impl Guard {
    fn new() -> Guard {
        Guard {
            sleepings: [0; 60],
            slept_for: 0,
        }
    }

    fn sleeping(&mut self, from: usize, to: usize) {
        for i in from..to {
            self.sleepings[i] += 1;
        }
        self.slept_for += to - from;
    }

    fn count_asleep(&self) -> usize {
        self.slept_for
    }

    fn best(&self) -> (usize, u32) {
        let mut best = 0;
        let mut best_idx = 0;
        for i in 0..self.sleepings.len() {
            if self.sleepings[i] > best {
                best = self.sleepings[i];
                best_idx = i;
            }
        }
        (best_idx, best)
    }
}

fn parts(input: &[Entry]) -> Result<(usize, usize)> {
    let mut guards = HashMap::new();
    let mut cur_guard: Option<usize> = None;
    let mut sleepiest: usize = 0;
    let mut sleepiest_asleep: usize = 0;
    let mut sleepiest_minute: usize = 0;
    let mut deepest: usize = 0;
    let mut deepest_count: u32 = 0;
    let mut deepest_minute: usize = 0;
    let mut asleep_at = 0;
    for entry in input {
        match entry.what {
            BeginShift(gnum) => {
                cur_guard = Some(gnum);
                asleep_at = 0;
            }
            FallAsleep => {
                asleep_at = entry.minute();
            }
            WakeUp => {
                let gnum = cur_guard.ok_or("No guard number?")?;
                // Guard gnum is asleep from asleep_at to entry.minute();
                let gptr = guards.entry(gnum).or_insert_with(Guard::new);
                gptr.sleeping(asleep_at, entry.minute());
                let (best_minute, best_count) = gptr.best();
                if gptr.count_asleep() > sleepiest_asleep {
                    sleepiest_asleep = gptr.count_asleep();
                    sleepiest = gnum;
                    sleepiest_minute = best_minute;
                }
                if best_count > deepest_count {
                    deepest = gnum;
                    deepest_count = best_count;
                    deepest_minute = best_minute;
                }
            }
        }
    }
    // Part 1 is the sleepiest guard number multiplied by its sleepiest minute
    let part1 = sleepiest * sleepiest_minute;
    // Part 2 requires us to find the most asleep guard by the minute
    let part2 = deepest * deepest_minute;

    Ok((part1, part2))
}

fn main() -> Result<()> {
    let test_input = sorted_from_str(TEST_INPUT)?;
    let input = sorted_from_str(read_input(4)?)?;
    println!("Loaded {} instructions from test data", test_input.len());
    println!("Loaded {} instructions from real data", input.len());

    println!("Test: {:?}", parts(&test_input)?);
    println!("Real: {:?}", parts(&input)?);
    Ok(())
}
