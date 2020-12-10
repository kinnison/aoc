use aoc2020::*;

fn part1(input: &[u32]) -> u64 {
    let mut ones = 0;
    let mut threes = 0;
    for (a, b) in Some(0) // Flight seat
        .iter()
        .chain(input.iter()) // My adapters
        .chain(Some(input[input.len() - 1] + 3).iter()) // laptop input 3 higher
        .tuple_windows()
    {
        match b - a {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        };
    }
    ones * threes
}

fn part2(input: &[u32]) -> u64 {
    // Count subsets of input which can make it from zero to max+3
    // Nominally there are too many to enumerate them easily
    let full_input: Vec<_> = Some(0) // Flight seat
        .iter()
        .copied()
        .chain(input.iter().copied()) // My adapters
        .chain(Some(input[input.len() - 1] + 3).iter().copied()) // laptop input 3 higher
        .collect();
    // We know there's no differences of 2 jolts in our input thanks
    // to the assertion made in main() and our test input by inspection.
    // As such, we want to count the lengths of the runs of 1 jolt differences
    // so that we can work up from there.  We can never remove any 3diff, but
    // all but the last of a run of 1diffs could be removed

    // Step 1 for that is to collect all our differences
    let diffs = full_input.iter().tuple_windows().map(|(a, b)| b - a);
    // Step 2, find lengths of runs of difference 1 of 4 or more
    let runs: Vec<_> = diffs
        .map(|v| (v, 1))
        .coalesce(|prev, next| {
            if prev.0 == next.0 {
                Ok((prev.0, prev.1 + next.1))
            } else {
                Err((prev, next))
            }
        })
        // but we only care about the runs of 1s of 2 or more
        .filter_map(|(v, l)| if v == 1 && l > 1 { Some(l) } else { None })
        .collect();
    println!("{:?}", runs);

    // for runs of 1, there's only 1 way to do it (keep)
    // for runs of 2, there's 2 ways (keep or remove first)
    // for runs of 3, there's 4 ways (keep or remove each of first 2)
    // for runs of 4, there's 7 ways (keep or remove first, or keep first one, keep or remove subsequent 2)
    // by inspection, we don't have runs > 4 in our input, so try multiplying by that
    runs.into_iter()
        .map(|n| match n {
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            _ => unimplemented!(),
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT1: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    const TEST_INPUT2: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    #[test]
    fn testcase1() {
        let mut input1 = input_as_vec(TEST_INPUT1).unwrap();
        input1.sort_unstable();
        let mut input2 = input_as_vec(TEST_INPUT2).unwrap();
        input2.sort_unstable();
        assert_eq!(part1(&input1), 7 * 5);
        assert_eq!(part1(&input2), 22 * 10);
    }

    #[test]
    fn testcase2() {
        let mut input1 = input_as_vec(TEST_INPUT1).unwrap();
        input1.sort_unstable();
        let mut input2 = input_as_vec(TEST_INPUT2).unwrap();
        input2.sort_unstable();
        assert_eq!(part2(&input1), 8);
        assert_eq!(part2(&input2), 19208);
    }
}

fn main() -> Result<()> {
    let mut input = read_input_as_vec(10)?;
    input.sort_unstable();
    // We make the assumption there are no diff-2 spaces in our sequence
    for (a, b) in input.iter().tuple_windows() {
        let diff = b - a;
        assert!(diff == 1 || diff == 3);
    }
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
