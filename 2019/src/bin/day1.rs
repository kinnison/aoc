use aoc2019::*;

fn fuel_cost_1(mass: usize) -> Option<usize> {
    if mass < 9 {
        None
    } else {
        Some((mass / 3) - 2)
    }
}

fn fuel_cost_2(mass: usize) -> Option<usize> {
    Some(std::iter::successors(Some(fuel_cost_1(mass)?), |n| fuel_cost_1(*n)).sum())
}

fn part2(input: &[usize]) -> usize {
    input.iter().copied().filter_map(fuel_cost_2).sum()
}

fn part1(input: &[usize]) -> usize {
    input.iter().copied().filter_map(fuel_cost_1).sum()
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
        assert_eq!(fuel_cost_2(12).unwrap(), 2);
        assert_eq!(fuel_cost_2(1969).unwrap(), 966);
        assert_eq!(fuel_cost_2(100_756).unwrap(), 50_346);
    }
}

fn main() -> Result<()> {
    let input: Result<Vec<usize>> = read_input_as_vec(1);
    let input = input?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
