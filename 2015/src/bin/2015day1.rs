use aoc2015::*;
use std::io::Result;

fn part1(input: &str) -> i32 {
    let mut floor = 0;
    for ch in input.chars() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
    }
    floor
}

fn part2(input: &str) -> usize {
    let mut floor = 0;
    for (idx, ch) in input.chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        };
        if floor == -1 {
            return idx + 1;
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let input = read_input(1)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
