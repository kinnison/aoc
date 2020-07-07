use aoc2015::*;

struct Reindeer {
    _name: String,
    speed: usize,
    runtime: usize,
    resttime: usize,
}

impl Reindeer {
    fn from_str(input: &str) -> Reindeer {
        lazy_static! {
            static ref PARSE: Regex = Regex::new("^([^ ]+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds\\.$").unwrap();
        }
        if let Some(cap) = PARSE.captures(input) {
            let name = cap.get(1).unwrap().as_str().to_owned();
            let speed = cap.get(2).unwrap().as_str().parse().unwrap();
            let runtime = cap.get(3).unwrap().as_str().parse().unwrap();
            let resttime = cap.get(4).unwrap().as_str().parse().unwrap();
            Reindeer {
                _name: name,
                speed,
                runtime,
                resttime,
            }
        } else {
            panic!("Unable to parse reindeer: {}", input);
        }
    }

    fn position_at_time(&self, time: usize) -> usize {
        let time_in_window: usize = time % (self.runtime + self.resttime);
        let windows: usize = time / (self.runtime + self.resttime);
        if time_in_window <= self.runtime {
            (windows * self.runtime * self.speed) + (time_in_window * self.speed)
        } else {
            (windows + 1) * self.runtime * self.speed
        }
    }
}

fn part1(input: &Vec<Reindeer>, time: usize) -> usize {
    let mut maxdist = std::usize::MIN;
    for deer in input {
        let pos = deer.position_at_time(time);
        if pos > maxdist {
            maxdist = pos
        }
    }
    maxdist
}

fn part2(input: &Vec<Reindeer>, racelen: usize) -> usize {
    let mut scores: Vec<usize> = Vec::new();
    scores.resize(input.len(), 0);
    for second in 1..=racelen {
        let best = part1(input, second);
        for (n, deer) in input.iter().enumerate() {
            if deer.position_at_time(second) == best {
                scores[n] += 1;
            }
        }
    }

    *scores.iter().max().unwrap()
}

fn main() -> Result<()> {
    //let test_input: Vec<Reindeer> = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.\n".lines().map(Reindeer::from_str).collect();
    //println!("Test part 1: {}", part1(&test_input, 2503));
    //println!("Test part 2: {}", part2(&test_input, 1000));

    let input: Vec<Reindeer> = read_input(14)?.lines().map(Reindeer::from_str).collect();
    println!("Part 1: {}", part1(&input, 2503));
    println!("Part 2: {}", part2(&input, 2503));
    Ok(())
}
