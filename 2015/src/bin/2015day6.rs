use aoc2015::*;
use lazy_static::lazy_static;
use regex::Regex;

enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
    op: Op,
}

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        lazy_static! {
            static ref TURNON: Regex =
                Regex::new("turn on ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
            static ref TURNOFF: Regex =
                Regex::new("turn off ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
            static ref TOGGLE: Regex =
                Regex::new("toggle ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
        };
        if let Some(cap) = TURNON.captures(input) {
            let x1 = cap.get(1).unwrap().as_str().parse().unwrap();
            let y1 = cap.get(2).unwrap().as_str().parse().unwrap();
            let x2 = cap.get(3).unwrap().as_str().parse().unwrap();
            let y2 = cap.get(4).unwrap().as_str().parse().unwrap();
            Instruction {
                x1,
                x2,
                y1,
                y2,
                op: Op::TurnOn,
            }
        } else if let Some(cap) = TURNOFF.captures(input) {
            let x1 = cap.get(1).unwrap().as_str().parse().unwrap();
            let y1 = cap.get(2).unwrap().as_str().parse().unwrap();
            let x2 = cap.get(3).unwrap().as_str().parse().unwrap();
            let y2 = cap.get(4).unwrap().as_str().parse().unwrap();
            Instruction {
                x1,
                x2,
                y1,
                y2,
                op: Op::TurnOff,
            }
        } else if let Some(cap) = TOGGLE.captures(input) {
            let x1 = cap.get(1).unwrap().as_str().parse().unwrap();
            let y1 = cap.get(2).unwrap().as_str().parse().unwrap();
            let x2 = cap.get(3).unwrap().as_str().parse().unwrap();
            let y2 = cap.get(4).unwrap().as_str().parse().unwrap();
            Instruction {
                x1,
                x2,
                y1,
                y2,
                op: Op::Toggle,
            }
        } else {
            unreachable!()
        }
    }

    fn apply1(&self, lights: &mut [[bool; 1000]; 1000]) {
        for x in self.x1..=self.x2 {
            for y in self.y1..=self.y2 {
                match self.op {
                    Op::TurnOn => lights[y][x] = true,
                    Op::TurnOff => lights[y][x] = false,
                    Op::Toggle => lights[y][x] = !lights[y][x],
                }
            }
        }
    }

    fn apply2(&self, lights: &mut [[usize; 1000]; 1000]) {
        for x in self.x1..=self.x2 {
            for y in self.y1..=self.y2 {
                match self.op {
                    Op::TurnOn => lights[y][x] = lights[y][x] + 1,
                    Op::TurnOff => lights[y][x] = lights[y][x].saturating_sub(1),
                    Op::Toggle => lights[y][x] = lights[y][x] + 2,
                }
            }
        }
    }
}

fn part1(input: &Vec<Instruction>) -> usize {
    let mut lights = [[false; 1000]; 1000];
    for instr in input {
        instr.apply1(&mut lights);
    }
    let mut tot = 0;
    for row in lights.iter() {
        for col in row.iter() {
            if *col {
                tot += 1;
            }
        }
    }
    tot
}

fn part2(input: &Vec<Instruction>) -> usize {
    let mut lights = [[0usize; 1000]; 1000];
    for instr in input {
        instr.apply2(&mut lights);
    }
    let mut tot = 0;
    for row in lights.iter() {
        for col in row.iter() {
            tot += *col
        }
    }
    tot
}

fn main() -> Result<()> {
    let input: Vec<Instruction> = read_input(6)?
        .lines()
        .map(|s| Instruction::from_str(s))
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
