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

fn value_of_entry(entry: &SpacedString) -> usize {
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
                dstrs[1] = &entry[i];
                entry[i].chars().for_each(|c| {
                    digits[1].insert(c);
                });
            }
            4 => {
                dstrs[4] = &entry[i];
                entry[i].chars().for_each(|c| {
                    digits[4].insert(c);
                });
            }
            3 => {
                dstrs[7] = &entry[i];
                entry[i].chars().for_each(|c| {
                    digits[7].insert(c);
                });
            }
            7 => {
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
    // segment b/d will be 4 minus 1
    let seg_bd: String = digits[4].difference(&digits[1]).cloned().collect();
    assert_eq!(seg_bd.len(), 2);
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
                })
            }
        });
    // We now know a, b, d and we think we know cf

    todo!()
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
