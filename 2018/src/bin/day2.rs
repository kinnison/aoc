use aoc2018::*;

fn check_n(input: &str, n: usize) -> bool {
    let mut counts = [0; 26];
    input
        .chars()
        .for_each(|c| counts[(c as usize) - ('a' as usize)] += 1);
    counts.iter().any(|&cn| cn == n)
}

fn count_with_n(input: &[&str], n: usize) -> usize {
    input.iter().filter(|&s| check_n(s, n)).count()
}

fn part1(input: &[&str]) -> usize {
    count_with_n(input, 2) * count_with_n(input, 3)
}

fn single_common(left: &str, right: &str) -> Option<usize> {
    let pairs = left.chars().zip(right.chars()).enumerate();
    let diffs: Vec<usize> = pairs
        .filter(|(_n, (l, r))| l != r)
        .map(|(n, _)| n)
        .collect();
    if diffs.len() != 1 {
        None
    } else {
        Some(diffs[0])
    }
}

fn part2(input: &[&str]) -> Result<String> {
    for l in 0..input.len() - 1 {
        for r in l + 1..input.len() {
            if let Some(ofs) = single_common(input[l], input[r]) {
                return Ok(input[l]
                    .chars()
                    .take(ofs)
                    .chain(input[l].chars().skip(ofs + 1))
                    .collect());
            }
        }
    }
    Err("Unable to find two which have a matching single character")?
}

fn main() -> Result<()> {
    let test_input1 = vec![
        "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
    ];
    let test_input2 = vec![
        "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
    ];
    let input = read_input(2)?;
    let input: Vec<&str> = input.lines().collect();
    println!("Test 1: {}", part1(&test_input1));
    println!("Part 1: {}", part1(&input));
    println!("Test 2: {}", part2(&test_input2)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
