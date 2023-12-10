use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(9)?;
    let input = parse_input(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Input {
    seqs: Vec<Vec<i64>>,
}

fn parse_input(input: &str) -> Input {
    let mut seqs = Vec::new();
    for l in input.trim().lines() {
        seqs.push(
            l.trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec(),
        );
    }
    Input { seqs }
}

fn predict_seq(n: &[i64]) -> i64 {
    let mut intermeds = Vec::new();
    let mut diffs = n.to_vec();
    loop {
        diffs = std::mem::take(&mut diffs)
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect_vec();
        intermeds.push(diffs.last().copied().unwrap());
        if diffs.iter().all(|&n| n == 0) {
            break;
        }
    }

    intermeds.into_iter().fold(n[n.len() - 1], |acc, n| acc + n)
}

fn predict_seq_backwards(n: &[i64]) -> i64 {
    let mut intermeds = Vec::new();
    let mut diffs = n.to_vec();
    loop {
        diffs = std::mem::take(&mut diffs)
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect_vec();
        intermeds.push(diffs.first().copied().unwrap());
        if diffs.iter().all(|&n| n == 0) {
            break;
        }
    }
    while intermeds.len() > 1 {
        let a = intermeds.pop().unwrap();
        let b = intermeds.pop().unwrap();
        intermeds.push(b - a);
    }
    n[0] - intermeds[0]
}

fn part1(input: &Input) -> i64 {
    input.seqs.iter().map(|seq| predict_seq(seq)).sum()
}

fn part2(input: &Input) -> i64 {
    input
        .seqs
        .iter()
        .map(|seq| predict_seq_backwards(seq))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn testcase1_1() {
        let input = parse_input(TEST_INPUT);
        eprintln!("{input:?}");
        assert_eq!(part1(&input), 114);
    }

    #[test]
    fn testcase2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 2);
    }
}
