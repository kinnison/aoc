use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(4)?;
    let input = parse_winners(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn parse_winners(input: &str) -> Vec<usize> {
    let mut ret = Vec::new();
    for card in input.lines() {
        let (_, rest) = card.split_once(": ").unwrap();
        let (winners, have) = rest.trim().split_once(" | ").unwrap();
        let winners: HashSet<u8> = winners
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect();
        let have: HashSet<u8> = have
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect();
        ret.push(winners.intersection(&have).count());
    }
    ret
}

fn part1(input: &[usize]) -> u64 {
    input.iter().copied().map(|n| (1 << n) >> 1).sum()
}

fn part2(input: &[usize]) -> u64 {
    let mut card_count = vec![1; input.len()];

    for (i, winners) in input.iter().copied().enumerate() {
        let mul = card_count[i];
        ((i + 1)..=(i + winners)).for_each(|idx| {
            card_count[idx] += mul;
        });
    }

    card_count.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn testcase1() {
        let input = parse_winners(TEST_INPUT);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn testcase2() {
        let input = parse_winners(TEST_INPUT);
        assert_eq!(part2(&input), 30);
    }
}
