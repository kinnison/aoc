use aoc2020::*;

#[derive(ParseByRegex, Debug)]
enum Bus {
    #[regex = r"(\d+)"]
    Line(u64),
    #[regex = "x"]
    Missing,
}

fn part1(start: u64, busses: &[Bus]) -> u64 {
    let chosen = busses
        .iter()
        // First, find when the next bus will happen
        .filter_map(|b| match b {
            Bus::Missing => None,
            Bus::Line(n) => {
                // The next bus for this line is whatever multiple of n is
                // equal or greater than start
                let mut mult = start / n;
                if (mult * n) < start {
                    mult += 1;
                }
                Some((n, mult * n))
            }
        })
        // Next we need to pick the smallest delta between start and when
        // the bus will depart
        .fold1(|a, b| if a.1 < b.1 { a } else { b })
        // Now we can be sure we got at least one bus
        .unwrap();
    // Bus ID * wait in minutes
    chosen.0 * (chosen.1 - start)
}

fn part2(busses: &[Bus]) -> u64 {
    // Turn busses into (offset, line) tuples
    let busses = busses.iter().enumerate().filter_map(|(i, b)| match b {
        Bus::Missing => None,
        Bus::Line(n) => Some((i as u64, *n)),
    });
    // We know that in theory we can depart every minute
    // so we can treat ourselves as a (0,1) tuple and then
    // proceed generically.
    let mut step = 1;
    let mut time = 0;
    for (offset, line) in busses {
        // Count from time in step until we reach a multiple of the bus
        // at the offset
        for furthertime in (1..).map(move |n| n * step) {
            if (time + furthertime + offset) % line == 0 {
                // We've hit this bus's correct spot in the sequence
                // Advance our timestamp to this point
                time += furthertime;
                // And we continue stepping with the lowest common multiple
                // of the old step and the new line
                step = step.lcm(line);
                break;
            }
        }
    }
    time
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "939\n7,13,x,x,59,x,31,19\n";

    #[test]
    fn testcase1() {
        let (start, busses) = input_as_first_and_vec_by_pat(TEST_INPUT, ",").unwrap();
        let start = start.parse().unwrap();
        assert_eq!(part1(start, &busses), 295);
    }

    #[test]
    fn testcase2() {
        for tcase in &[
            (TEST_INPUT, 1068781),
            ("x\n17,x,13,19", 3417),
            ("x\n67,7,59,61", 754018),
            ("x\n67,x,7,59,61", 779210),
            ("x\n67,7,x,59,61", 1261476),
            ("x\n1789,37,47,1889", 1202161486),
        ] {
            let (_, busses) = input_as_first_and_vec_by_pat(tcase.0, ",").unwrap();
            assert_eq!(part2(&busses), tcase.1);
        }
    }
}

fn main() -> Result<()> {
    let (start, busses) = read_input_as_first_and_vec_by_pat(13, ",")?;
    let start = start.parse()?;
    println!("Part 1: {}", part1(start, &busses));
    println!("Part 2: {}", part2(&busses));
    Ok(())
}
