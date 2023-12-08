use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(6)?;
    let input = parse_racedata(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct RaceData {
    times: Vec<u64>,
    records: Vec<u64>,
    bigtime: u64,
    bigrecord: u64,
}

fn parse_racedata(input: &str) -> RaceData {
    // Two lines, the first is times, the second is records.
    let (times, records) = input.split_once('\n').unwrap();
    let (_, times) = times.trim().split_once(": ").unwrap();
    let (_, records) = records.trim().split_once(": ").unwrap();
    let bigtime = times.chars().filter(|c| c.is_numeric()).collect::<String>();
    let bigtime = bigtime.parse().unwrap();
    let bigrecord = records
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>();
    let bigrecord = bigrecord.parse().unwrap();
    let times = times
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    let records = records
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();
    RaceData {
        times,
        records,
        bigtime,
        bigrecord,
    }
}

fn part1(input: &RaceData) -> u64 {
    let mut ret = 1;
    for (time, record) in input
        .times
        .iter()
        .copied()
        .zip(input.records.iter().copied())
    {
        let mut beats = 0;
        for hold in 1..time {
            let dist = (time - hold) * hold;
            if dist > record {
                beats += 1;
            }
        }
        ret *= beats;
    }
    ret
}

fn part2(input: &RaceData) -> u64 {
    let mut beats = 0;
    for hold in 1..input.bigtime {
        let dist = (input.bigtime - hold) * hold;
        if dist > input.bigrecord {
            beats += 1;
        }
    }
    beats
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn testcase1() {
        let input = parse_racedata(TEST_INPUT);
        eprintln!("{input:?}");
        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn testcase2() {
        let input = parse_racedata(TEST_INPUT);
        assert_eq!(part2(&input), 71503);
    }
}
