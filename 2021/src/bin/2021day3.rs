use aoc2021::*;

fn part1(input: &[String]) -> u64 {
    let mut gamma = 0;
    let mut epsilon = 0;

    for bit in 0..input[0].len() {
        let ones = input.iter().filter(|s| s.as_bytes()[bit] == b'1').count();
        let zeroes = input.len() - ones;
        gamma <<= 1;
        epsilon <<= 1;
        if ones > zeroes {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
    }

    gamma * epsilon
}

fn filter_for(input: &[String], pickones: bool) -> u64 {
    let mut input = input.to_vec();
    let mut bit = 0;
    while input.len() > 1 {
        let ones = input.iter().filter(|s| s.as_bytes()[bit] == b'1').count();
        let zeroes = input.len() - ones;
        let goal = if pickones {
            if ones >= zeroes {
                b'1'
            } else {
                b'0'
            }
        } else if ones >= zeroes {
            b'0'
        } else {
            b'1'
        };
        input.retain(|v| v.as_bytes()[bit] == goal);
        bit += 1;
    }
    let mut out = 0;
    for b in input[0].bytes() {
        out <<= 1;
        if b == b'1' {
            out |= 1;
        }
    }
    out
}

fn part2(input: &[String]) -> u64 {
    filter_for(input, true) * filter_for(input, false)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn testcase1() {
        let input: Vec<String> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn testcase2() {
        let input: Vec<String> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 230);
    }
}

fn main() -> Result<()> {
    let input: Vec<String> = read_input_as_vec(3)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
