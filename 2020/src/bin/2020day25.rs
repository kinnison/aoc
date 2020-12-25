use aoc2020::*;

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

fn determine_loop_size(public: u64) -> u64 {
    // secret loop size is 7 transformed N times until equal to public
    let mut ret = 0;
    let mut value = 1;
    while value != public {
        value *= 7; // Initial subject number
        value %= 20201227;
        ret += 1;
    }
    ret
}

fn part1(input: &[u64]) -> u64 {
    let loops = determine_loop_size(input[0]);
    transform(input[1], loops)
}

fn part2(input: &[u64]) -> u64 {
    part1(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn det_loop_sz() {
        assert_eq!(determine_loop_size(5764801), 8);
        assert_eq!(determine_loop_size(17807724), 11);
    }

    #[test]
    fn transform_chk() {
        assert_eq!(transform(17807724, 8), 14897079);
        assert_eq!(transform(5764801, 11), 14897079);
    }

    const TEST_INPUT: &str = r#"5764801
17807724"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 14897079);
    }

    #[test]
    fn testcase2() {}
}

fn main() -> Result<()> {
    let input = read_input_as_vec(25)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
