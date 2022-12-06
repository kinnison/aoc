use aoc2022::*;

fn part1(input: &str) -> usize {
    input
        .chars()
        .tuple_windows()
        .enumerate()
        .find(|&(_, (a, b, c, d))| (a != b && a != c && a != d) && (b != c && b != d) && (c != d))
        .map(|(i, _)| i + 4)
        .unwrap()
}

fn part2(input: &str) -> usize {
    (0..input.len() - 14)
        .map(|i| (i, input.chars().skip(i).take(14)))
        .map(|(i, ch)| {
            let s: HashSet<char> = ch.collect();
            (i, s.len() == 14)
        })
        .find(|&(_, b)| b)
        .map(|(i, _)| i + 14)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &[(&str, usize, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn testcase1() {
        for &(input, p1res, _) in TEST_INPUT {
            assert_eq!(part1(input), p1res);
        }
    }

    #[test]
    fn testcase2() {
        for &(input, _, p2res) in TEST_INPUT {
            assert_eq!(part2(input), p2res);
        }
    }
}

fn main() -> Result<()> {
    let input: String = read_input(6)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
