use aoc2021::*;

fn part1(input: &[SpacedString]) -> usize {
    // Count 1, 4, 7, 8 in outputs in entries
    // they use 2, 4, 3, and 7 segments respectively
    // outputs start at offset 11
    input
        .iter()
        .map(|e| {
            e.iter()
                .skip(11) // 10 digits, plus separator
                .filter(|v| matches!(v.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum()
}

#[allow(clippy::iter_nth_zero)]
fn value_of_entry(entry: &SpacedString) -> usize {
    println!("Input: {:?}", entry);
    // To calculate the value of the entry we need to decode which input is which.
    let mut digits = Vec::new();
    let mut dstrs = [""; 10];
    for _ in 0..10 {
        digits.push(HashSet::new())
    }
    // Extract 1, 4, 7, 8 (lens 2, 4, 3, 7)
    for i in 0..10 {
        match entry[i].len() {
            2 => {
                println!("1 == {}", entry[i]);
                dstrs[1] = &entry[i];
                entry[i].chars().for_each(|c| {
                    digits[1].insert(c);
                });
            }
            4 => {
                println!("4 == {}", entry[i]);
                dstrs[4] = &entry[i];
                entry[i].chars().for_each(|c| {
                    digits[4].insert(c);
                });
            }
            3 => {
                println!("7 == {}", entry[i]);
                dstrs[7] = &entry[i];
                entry[i].chars().for_each(|c| {
                    digits[7].insert(c);
                });
            }
            7 => {
                println!("8 == {}", entry[i]);
                dstrs[8] = &entry[i];
                entry[i].chars().for_each(|c| {
                    digits[8].insert(c);
                });
            }
            _ => {}
        }
    }

    // Now attempt to determine mappings of other values
    // 7 is 1 plus segment a
    let seg_a = *digits[7].difference(&digits[1]).next().unwrap();
    println!("a => {}", seg_a);
    // segment b/d will be 4 minus 1
    let seg_bd: String = digits[4].difference(&digits[1]).cloned().collect();
    assert_eq!(seg_bd.len(), 2);
    println!("b/d in {}", seg_bd);
    // whichever 6 char sequence lacks one of b/d is zero and the lacked segment is d
    let mut seg_b = ' ';
    let mut seg_d = ' ';
    entry
        .iter()
        .take(10)
        .filter(|v| v.len() == 6)
        .for_each(|v| {
            let may_b = v.contains(seg_bd.chars().nth(0).unwrap());
            let may_d = v.contains(seg_bd.chars().nth(1).unwrap());
            if let Some(idx) = if may_b && !may_d {
                Some(1)
            } else if may_d && !may_b {
                Some(0)
            } else {
                None
            } {
                // idx is which of seg_bd was not in the 0, and thus is segment d
                seg_d = seg_bd.chars().nth(idx).unwrap();
                seg_b = seg_bd.chars().nth(1 - idx).unwrap();
                dstrs[0] = v;
                v.chars().for_each(|c| {
                    digits[0].insert(c);
                });
                println!("0 == {}", v);
            }
        });
    println!("b => {}", seg_b);
    println!("d => {}", seg_d);
    // We now know a, b, d and we think we know cf
    // If we look for 6 char sequences which lack one of cf and are not zero then
    // we will have found 6 (and commensurately 9 which lacks e)
    let mut seg_c = ' ';
    let mut seg_f = ' ';
    let mut seg_e = ' ';
    let zero = dstrs[0].to_string();
    entry
        .iter()
        .take(10)
        .filter(|v| v.len() == 6 && *v != &zero)
        .for_each(|v| {
            let may_c = v.contains(dstrs[1].chars().nth(0).unwrap());
            let may_f = v.contains(dstrs[1].chars().nth(1).unwrap());
            if let Some(idx) = if may_c && !may_f {
                Some(0)
            } else if may_f && !may_c {
                Some(1)
            } else {
                None
            } {
                // idx is the found character (f) we found 6
                seg_f = dstrs[1].chars().nth(idx).unwrap();
                seg_c = dstrs[1].chars().nth(1 - idx).unwrap();
                dstrs[6] = v;
                v.chars().for_each(|c| {
                    digits[6].insert(c);
                });
                println!("6 == {}", v);
            } else {
                // We found 9, which lacks e...
                assert!(may_c);
                assert!(may_f);
                dstrs[9] = v;
                v.chars().for_each(|c| {
                    digits[9].insert(c);
                });
                for e in "abcdefg".chars() {
                    if !v.contains(e) {
                        seg_e = e;
                    }
                }
                println!("9 == {}", v);
            }
        });
    println!("c => {}", seg_c);
    println!("e => {}", seg_e);
    println!("f => {}", seg_f);

    // At this point we know 0, 1, 4, 6, 7, 8, 9
    // And we know a, b, c, d, e, f - meaning we could deduce g and thence
    // know 2, 3, and 5, all of which lack 2 of those characters
    let mut eight = digits[8].clone();
    eight.remove(&seg_a);
    eight.remove(&seg_b);
    eight.remove(&seg_c);
    eight.remove(&seg_d);
    eight.remove(&seg_e);
    eight.remove(&seg_f);
    let seg_g = eight.iter().copied().next().unwrap();
    println!("g => {}", seg_g);
    // Now we know g we can work out which of the remaining items is 2, 3, and 5
    entry
        .iter()
        .take(10)
        .filter(|v| v.len() == 5)
        .for_each(|v| {
            // 2, 3, 5
            let has_b = v.contains(seg_b);
            let has_e = v.contains(seg_e);
            let has_f = v.contains(seg_f);
            let idx = match (has_b, has_e, has_f) {
                (false, true, false) => 2,
                (false, false, true) => 3,
                (true, false, true) => 5,
                _ => panic!("Didn't expect this digit: {}", v),
            };
            dstrs[idx] = v;
            v.chars().for_each(|c| {
                digits[idx].insert(c);
            });
            println!("{} == {}", idx, v);
        });
    // Finally we want to iterate the outputs, finding the inputs and mapping them.
    entry
        .iter()
        .skip(11)
        .map(|v| {
            // Find which dstr contains v
            println!("Looking for {}", v);
            dstrs
                .iter()
                .enumerate()
                .filter(|(_, &d)| d.len() == v.len() && d.chars().all(|c| v.contains(c)))
                .map(|(n, _)| n)
                .next()
                .unwrap()
        })
        .fold(0, |acc, n| (acc * 10) + n)
}
fn part2(input: &[SpacedString]) -> usize {
    // Attempt to decode each entry, and sum the values
    input.iter().map(value_of_entry).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn testcase1() {
        let input: Vec<SpacedString> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn testcase2() {
        let input: Vec<SpacedString> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 61229);
    }
}

fn main() -> Result<()> {
    let input: Vec<SpacedString> = read_input_as_vec(8)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
