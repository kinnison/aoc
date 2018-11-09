use aoc2015::*;

fn part1(input: &Vec<usize>, target: usize) -> usize {
    let mut totmet = 0;
    for v in 0..(1 << input.len()) {
        let mut combo = 0;
        for ent in 0..input.len() {
            if (v & (1 << ent)) != 0 {
                combo += input[ent];
            }
        }
        if combo == target {
            totmet += 1;
        }
    }
    totmet
}

fn part2(input: &Vec<usize>, target: usize) -> usize {
    let mut ctrcount = input.len() + 1;
    let mut totmet = 0;
    for v in 0..(1 << input.len()) {
        let mut combo = 0;
        let mut ctrs = 0;
        for ent in 0..input.len() {
            if (v & (1 << ent)) != 0 {
                ctrs += 1;
                combo += input[ent];
            }
        }
        if combo == target {
            if ctrs < ctrcount {
                ctrcount = ctrs;
                totmet = 1;
            } else if ctrs == ctrcount {
                totmet += 1;
            }
        }
    }
    totmet
}

fn main() -> Result<()> {
    let test_input: Vec<usize> = vec![20, 15, 10, 5, 5];
    println!("Test 1: {}", part1(&test_input, 25));
    println!("Test 2: {}", part2(&test_input, 25));

    let input: Vec<usize> = read_input(17)?
        .lines()
        .map(|v| v.parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&input, 150));
    println!("Part 2: {}", part2(&input, 150));

    Ok(())
}
