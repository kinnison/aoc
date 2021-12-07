use aoc2021::*;

fn part1(input: &[i32]) -> i32 {
    // cheapest to implement algorithm is n-squared
    (input[0]..=input[input.len() - 1])
        .map(|pos| input.iter().map(|&crab| (pos - crab).abs()).sum())
        .min()
        .expect("No crabs?")
}

fn triangle(n: i32) -> i32 {
    (n * (n + 1)) / 2
}
fn part2(input: &[i32]) -> i32 {
    (input[0]..=input[input.len() - 1])
        .map(|pos| input.iter().map(|&crab| triangle((pos - crab).abs())).sum())
        .min()
        .expect("No crabs?")
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn testcase1() {
        let mut input: Vec<i32> = input_by_split_pat(TEST_INPUT, ",").unwrap();
        input.sort_unstable();
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn testcase2() {
        let mut input: Vec<i32> = input_by_split_pat(TEST_INPUT, ",").unwrap();
        input.sort_unstable();
        assert_eq!(part2(&input), 168);
    }
}

fn main() -> Result<()> {
    let input = read_input(7)?;
    let mut input = input_by_split_pat(input, ",")?;
    input.sort_unstable();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
