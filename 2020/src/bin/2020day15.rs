use aoc2020::*;

fn nth(input: &[usize], nth: usize) -> usize {
    let mut last_spoken = HashMap::new();
    // Prime the pump
    for (when, what) in input.iter().copied().enumerate().rev().skip(1).rev() {
        last_spoken.insert(what, when + 1);
    }
    // Now it's time to say the last number in our input
    let mut time_code = input.len();
    let mut number = input[time_code - 1];
    // So we're always considering the previously spoken number to decide what
    // to say next.  that number is then inserted the next time we speak
    while time_code < nth {
        // It is time_code + 1 in time, what do we say?
        let to_say = match last_spoken.get(&number).copied() {
            Some(when) => {
                // This number has been said before, indeed it was said
                time_code - when
                // turns ago
            }
            None => {
                // It wasn't said before, so we're going to say 0
                0
            }
        };
        // Remember our previously spoken number now
        last_spoken.insert(number, time_code);
        time_code += 1;
        number = to_say;
    }
    number
}

fn part1(input: &[usize]) -> usize {
    nth(input, 2020)
}

fn part2(input: &[usize]) -> usize {
    nth(input, 30_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &[(&str, usize, usize)] = &[
        ("0,3,6", 436, 175594),
        ("1,3,2", 1, 2578),
        ("2,1,3", 10, 3544142),
        ("1,2,3", 27, 261214),
        ("2,3,1", 78, 6895259),
        ("3,2,1", 438, 18),
        ("3,1,2", 1836, 362),
    ];

    #[test]
    fn testcase1() {
        for (input, target, _) in INPUTS.iter().copied() {
            let nums = input_by_split_pat(input, ",").unwrap();
            assert_eq!(part1(&nums), target);
        }
    }

    #[test]
    fn testcase2() {
        for (input, _, target) in INPUTS.iter().copied() {
            let nums = input_by_split_pat(input, ",").unwrap();
            assert_eq!(part2(&nums), target);
        }
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec_split(15, ",")?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
