use aoc2019::*;

fn fuel_cost_1(n: usize) -> Option<usize> {
    if n < 9 {
        None
    } else {
        Some((n / 3) - 2)
    }
}

fn fuel_cost_2(n: usize) -> usize {
    std::iter::successors(fuel_cost_1(n), |&n| fuel_cost_1(n)).sum()
}

fn part2(input: &[usize]) -> usize {
    input.iter().copied().map(fuel_cost_2).sum()
}

fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .copied()
        .map(fuel_cost_1)
        .map(Option::unwrap)
        .sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_cases_1() {
        use super::fuel_cost_1;
        assert_eq!(fuel_cost_1(8), None);
        assert_eq!(fuel_cost_1(12).unwrap(), 2);
        assert_eq!(fuel_cost_1(14).unwrap(), 2);
        assert_eq!(fuel_cost_1(1969).unwrap(), 654);
        assert_eq!(fuel_cost_1(100_756).unwrap(), 33_583);
    }

    #[test]
    fn test_cases_2() {
        use super::fuel_cost_2;
        assert_eq!(fuel_cost_2(12), 2);
        assert_eq!(fuel_cost_2(1969), 966);
        assert_eq!(fuel_cost_2(100_756), 50_346);
    }
}

fn main() -> Result<()> {
    let input = read_input(1)?;
    let input: Result<Vec<usize>> = input.lines().map(|s| Ok(s.parse()?)).collect();
    let input = input?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
