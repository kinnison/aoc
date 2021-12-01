use aoc2021::*;

fn part1(input: &[u64]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b)| a < b)
        .filter(|x| *x)
        .count()
}

fn part2(input: &[u64]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| a < b)
        .filter(|x| *x)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
199
200
208
210
200
207
240
269
260
263"#;

    #[test]
    fn testcase1() {
        let input: Vec<u64> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn testcase2() {
        let input: Vec<u64> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 5);
    }
}

fn main() -> Result<()> {
    let input: Vec<u64> = read_input_as_vec(1)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
