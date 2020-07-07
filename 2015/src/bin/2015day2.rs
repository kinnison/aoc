use aoc2015::*;
use std::io::Result;

struct Gift {
    length: usize,
    width: usize,
    height: usize,
}

impl Gift {
    fn surface_area(&self) -> usize {
        (self.length * self.width * 2)
            + (self.width * self.height * 2)
            + (self.length * self.height * 2)
    }

    fn smallest_side(&self) -> usize {
        let mut sized = [self.length, self.width, self.height];
        sized.sort();
        sized[0] * sized[1]
    }

    fn ribbon(&self) -> usize {
        let mut sized = [self.length, self.width, self.height];
        sized.sort();
        ((sized[0] + sized[1]) * 2) + (sized[0] * sized[1] * sized[2])
    }
}

fn parse_input(input: &str) -> Vec<Gift> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let parts: Vec<usize> = line.split('x').map(|s| s.parse().unwrap()).collect();
        ret.push(Gift {
            length: parts[0],
            width: parts[1],
            height: parts[2],
        })
    }
    ret
}

fn part1(input: &Vec<Gift>) -> usize {
    input
        .iter()
        .map(|g| g.surface_area() + g.smallest_side())
        .sum()
}

fn part2(input: &Vec<Gift>) -> usize {
    input.iter().map(|g| g.ribbon()).sum()
}

fn main() -> Result<()> {
    let input = read_input(2)?;
    let input = parse_input(&input);
    println!("Loaded {} gifts", input.len());
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
