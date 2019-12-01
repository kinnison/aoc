use aoc2019::*;

fn fuel_cost_1(n: usize) -> usize {
    (n / 3) - 2
}

fn fuel_cost_2(n: usize) -> usize {
    let mut mod_cost = fuel_cost_1(n);
    let mut fuel = mod_cost;
    while fuel > 8 {
        let cost = fuel_cost_1(fuel);
        mod_cost += cost;
        fuel = cost;
    }
    mod_cost
}

fn part2(input: &[usize]) -> usize {
    input.iter().copied().map(fuel_cost_2).sum()
}

fn part1(input: &[usize]) -> usize {
    input.iter().copied().map(fuel_cost_1).sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_cases_1() {
        use super::fuel_cost_1;
        assert_eq!(fuel_cost_1(12), 2);
        assert_eq!(fuel_cost_1(14), 2);
        assert_eq!(fuel_cost_1(1969), 654);
        assert_eq!(fuel_cost_1(100_756), 33_583);
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
