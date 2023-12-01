use aoc2023::*;

pub fn main() -> Result<()> {
    let input: Vec<String> = read_input_as_vec(1)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &[String]) -> u64 {
    // For each entry we want the first and last digits and then we treat that as a number and sum them
    input
        .iter()
        .map(|s| {
            let first = s.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = s.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let tens = (first as u8) - b'0';
            let digits = (last as u8) - b'0';
            (digits + (tens * 10)) as u64
        })
        .sum()
}

fn part2(input: &[String]) -> u64 {
    // This time we need to consider numeric digits 1-9 but also the substrings:
    // one two three four five six seven eight nine
    static NEEDLES: &[&str] = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    assert_eq!(NEEDLES.len(), 18);

    input
        .iter()
        .map(|s| {
            let needlepos = NEEDLES
                .iter()
                .map(|n| s.find(n))
                .enumerate()
                .filter_map(|(needle, pos)| pos.map(|pos| (pos, needle)))
                .sorted()
                .collect_vec();
            let first_needle = 1 + (needlepos[0].1 % 9);
            let needlepos = NEEDLES
                .iter()
                .map(|n| s.rfind(n))
                .enumerate()
                .filter_map(|(needle, pos)| pos.map(|pos| (pos, needle)))
                .sorted()
                .collect_vec();
            let last_needle = 1 + (needlepos.last().unwrap().1 % 9);

            (last_needle + (first_needle * 10)) as u64
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    static TEST_INPUT2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn testcase1() {
        let input: Vec<String> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn testcase2() {
        let input: Vec<String> = input_as_vec(TEST_INPUT2).unwrap();
        assert_eq!(part2(&input), 281);
    }
}
