use aoc2018::*;

fn part2(input: &Vec<i32>) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();
    seen.insert(0);
    let mut cur = 0;
    for n in input.iter().cycle() {
        cur += n;
        if seen.contains(&cur) {
            return cur;
        }
        seen.insert(cur);
    }
    unreachable!()
}

fn part1(input: &Vec<i32>) -> i32 {
    input.iter().sum()
}

fn main() -> Result<()> {
    let input = read_input(1)?;
    let input: Vec<i32> = input.lines().map(|s| s.parse().unwrap()).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
