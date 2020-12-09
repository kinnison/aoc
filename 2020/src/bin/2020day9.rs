use aoc2020::*;

fn part1(input: &[u64], preamble: usize) -> u64 {
    for idx in preamble..input.len() {
        let samples = &input[idx - preamble..idx];
        let mut found = false;
        'outer: for a in 0..preamble - 1 {
            for b in a + 1..preamble {
                if samples[a] + samples[b] == input[idx] {
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            return input[idx];
        }
    }
    unreachable!()
}

fn part2(input: &[u64], preamble: usize) -> u64 {
    let target = part1(input, preamble);
    for a in 0..input.len() - 2 {
        'inner: for b in a + 2..input.len() {
            // Sum, starting at a, ending at b or earlier if we hit or overflow
            let mut total = input[a];
            let mut smallest = total;
            let mut largest = total;
            #[allow(clippy::clippy::needless_range_loop)]
            for pos in a + 1..=b {
                total += input[pos];
                smallest = min(smallest, input[pos]);
                largest = max(largest, input[pos]);
                #[allow(clippy::comparison_chain)]
                if total == target {
                    // We've hit our target
                    return smallest + largest;
                } else if total > target {
                    // We've exceeded our target so stop this trial and start
                    // at a new starting position
                    break 'inner;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;

    #[test]
    fn testcase1() {
        let nums: Vec<u64> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&nums, 5), 127);
    }

    #[test]
    fn testcase2() {
        let nums: Vec<u64> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&nums, 5), 62);
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec(9)?;
    println!("Part 1: {}", part1(&input, 25));
    println!("Part 2: {}", part2(&input, 25));
    Ok(())
}
