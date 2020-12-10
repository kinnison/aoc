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
    // If we have a run of 1 then we can remove nothing, so there's 1 combination
    // If we have a run of 2 then we can remove at most 1 and so there are 2
    // possibilties.
    // a b
    //   b
    // A run of 3 we could remove either of the first 2, or both
    // so there are 4 possibilities.
    // a b c
    // two ways to remove 1
    //   b c
    // a   c
    // one way to remove 2
    //     c
    // A run of 4 is more complex since we can remove any number of runs
    // such that there are no gaps of more than 2 (leading to a 3 in the output)
    // remove nothing
    // a b c d
    // three ways to remove 1
    //   b c d
    // a   c d
    // a b   d
    // two ways to remove 2
    //     c d
    // a     d
    // This is 6 possibilities
    // Nominally this gives us a sequence of 1,2,4,6
    // which naively looks like multiples of 2, so let's try run of 5
    // there's one where we do nothing
    // a b c d e
    // four possible removals of 1 entry
    //   b c d e
    // a   c d e
    // a b   d e
    // a b c   e
    // some six ways to remove 2 entries
    //     c d e
    //   b   d e
    //   b c   e
    // a     d e
    // a   c   e
    // a b     e
    // And two ways to remove 3 entries
    //     c   e
    //   b     e
    // And no way to remove 3 or more
    // For a total of 12
    // Now we have 1,2,4,6,12 which no longer looks like multiples of 2
    // for a sequence of n, we always have 1 for unchanged, n-1 for one elision.
    // Removal of 2 entries sequence is 0,0,1,2,3
    // Let's longhand out sequences of 6 1s
    // one unchanged
    // a b c d e f
    // five ways to remove 1, let's not write them out
    // ten ways to remove 2
    //     c d e f
    //   b   d e f
    //   b c   e f
    //   b c d   f
    // a     d e f
    // a   c   e f
    // a   c d   f
    // a b     e f
    // a b   d   f
    // a b c     f
    // three ways to remove 3
    //     c   e f
    //     c d   f
    // a     d   f
    // one way to remove 4
    //     c     f
    // For a total of 20
    // 1,2,4,6,12,20

    // naively we multiple 2^(n-1) for all runs
    runs.into_iter().map(|n| 2u64.pow(n - 1)).product()
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
