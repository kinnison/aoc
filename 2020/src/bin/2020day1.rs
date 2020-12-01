use aoc2020::*;

fn part1(input: &[u64]) -> u64 {
    for a in 0..input.len() - 1 {
        for b in a + 1..input.len() {
            if input[a] + input[b] == 2020 {
                return input[a] * input[b];
            }
        }
    }
    unreachable!();
}

fn part2(input: &[u64]) -> u64 {
    for a in 0..input.len() - 2 {
        for b in a + 1..input.len() - 1 {
            for c in b + 1..input.len() {
                if input[a] + input[b] + input[c] == 2020 {
                    return input[a] * input[b] * input[c];
                }
            }
        }
    }
    unreachable!();
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
