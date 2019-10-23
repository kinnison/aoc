use aoc2018::*;

fn part2(input: &[i32]) -> Result<i32> {
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(0);
    let mut cur = 0;
    for n in input.iter().cycle() {
        cur += n;
        if seen.contains(&cur) {
            return Ok(cur);
        }
        seen.insert(cur);
    }
    Err("Unable to find a cycle ever?".into())
}

fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn main() -> Result<()> {
    let input = read_input(1)?;
    let input: Result<Vec<i32>> = input.lines().map(|s| Ok(s.parse()?)).collect();
    let input = input?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
