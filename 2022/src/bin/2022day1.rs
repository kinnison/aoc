use aoc2022::*;

fn part1(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|elf| elf.iter().copied().sum())
        .max()
        .unwrap()
}

fn part2(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .map(|elf| elf.iter().copied().sum())
        .sorted_by(|a: &u64, b| Ord::cmp(b, a))
        .take(3)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn testcase1() {
        let input: Vec<Vec<u64>> = input_as_groups(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Vec<u64>> = input_as_groups(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 45000);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<Vec<u64>> = read_input_as_groups(1)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
