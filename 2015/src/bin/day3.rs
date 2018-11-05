use aoc2015::*;
use std::collections::HashMap;
use std::io::Result;

fn part1(input: &str) -> usize {
    let mut houses: HashMap<(i32, i32), usize> = HashMap::new();

    houses.insert((0, 0), 1); // Santa delivers at his start point before travel
    let mut locx = 0;
    let mut locy = 0;

    for go in input.chars() {
        match go {
            '^' => locy += 1,
            'v' => locy -= 1,
            '>' => locx += 1,
            '<' => locx -= 1,
            _ => unimplemented!(),
        };
        *(houses.entry((locx, locy)).or_default()) += 1;
    }

    houses.iter().count()
}

fn part2(input: &str) -> usize {
    let mut houses: HashMap<(i32, i32), usize> = HashMap::new();

    houses.insert((0, 0), 1); // Santa delivers at his start point before travel
    let mut santa_locx = 0;
    let mut santa_locy = 0;
    let mut robo_locx = 0;
    let mut robo_locy = 0;
    let mut robo = false;

    for go in input.chars() {
        if robo {
            match go {
                '^' => robo_locy += 1,
                'v' => robo_locy -= 1,
                '>' => robo_locx += 1,
                '<' => robo_locx -= 1,
                _ => unimplemented!(),
            };
            *(houses.entry((robo_locx, robo_locy)).or_default()) += 1;
        } else {
            match go {
                '^' => santa_locy += 1,
                'v' => santa_locy -= 1,
                '>' => santa_locx += 1,
                '<' => santa_locx -= 1,
                _ => unimplemented!(),
            };
            *(houses.entry((santa_locx, santa_locy)).or_default()) += 1;
        }
        robo = !robo;
    }

    houses.iter().count()
}

fn main() -> Result<()> {
    let input = read_input(3)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
