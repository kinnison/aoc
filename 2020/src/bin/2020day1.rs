use aoc2020::*;

fn part1(input: &[u64]) -> u64 {
    input
        .iter()
        .copied()
        .tuple_combinations::<(_, _)>()
        .find_map(|i| {
            if i.0 + i.1 == 2020 {
                Some(i.0 * i.1)
            } else {
                None
            }
        })
        .unwrap()
}

fn part2(input: &[u64]) -> u64 {
    input
        .iter()
        .copied()
        .combinations(3)
        .find_map(|n| {
            if n.iter().copied().sum::<u64>() == 2020 {
                Some(n.iter().product())
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testcase1() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&input), 514579);
    }

    #[test]
    fn testcase2() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(&input), 241861950);
    }
}

fn main() -> Result<()> {
    let input: Vec<u64> = read_input_as_vec(1)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
