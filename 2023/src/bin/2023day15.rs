use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(15)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> u64 {
    input.trim().split(',').map(badhash).sum()
}

fn part2(input: &str) -> u64 {
    let mut present: HashMap<&str, (u64, u64)> = HashMap::new();
    for (idx, instr) in input.trim().split(',').enumerate() {
        if let Some(label) = instr.strip_suffix('-') {
            present.remove(&label);
        } else {
            let (label, flen) = instr.split_once('=').unwrap();
            let flen = (flen.as_bytes()[0] - b'0') as u64;

            match present.entry(label) {
                Entry::Occupied(mut o) => {
                    o.get_mut().1 = flen;
                }
                Entry::Vacant(v) => {
                    v.insert((idx as u64, flen));
                }
            }
        }
    }

    // At this point, only the lenses in the hashmap are actually present
    // in any box, so let's acquire and sort them according to when they went in.
    let mut ins_seq = present
        .into_iter()
        .map(|(label, (idx, flen))| (idx, label, flen))
        .collect_vec();
    ins_seq.sort_by_cached_key(|v| v.0);

    // Next, we go through the insertion sequence, computing the score
    let mut score = 0;
    let mut boxes = vec![0u64; 256];

    for (_, label, flen) in ins_seq {
        let boxn = badhash(label) as usize;
        boxes[boxn] += 1;
        let slot = boxes[boxn];
        let boxn = (boxn as u64) + 1;
        score += boxn * slot * flen
    }

    score
}

// While the badhash is defined as 0-255 by returning a u64 it's easier to process later
fn badhash(s: &str) -> u64 {
    s.bytes().fold(0, |acc, b| ((acc + (b as u64)) * 17) & 0xff)
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn hashhash() {
        assert_eq!(badhash("HASH"), 52);
    }

    #[test]
    fn testcase1() {
        assert_eq!(part1(TEST_INPUT), 1320);
    }

    #[test]
    fn testcase2() {
        assert_eq!(part2(TEST_INPUT), 145);
    }
}
