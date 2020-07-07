use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl Instruction {
    pub fn new(c: char) -> Instruction {
        if c == 'U' {
            return Instruction::Up;
        } else if c == 'D' {
            return Instruction::Down;
        } else if c == 'L' {
            return Instruction::Left;
        } else {
            return Instruction::Right;
        }
    }
    fn go(&self, pos: i32) -> i32 {
        match self {
            &Instruction::Up => {
                if pos < 4 {
                    pos
                } else {
                    pos - 3
                }
            }
            &Instruction::Down => {
                if pos > 6 {
                    pos
                } else {
                    pos + 3
                }
            }
            &Instruction::Left => {
                if (pos % 3) == 1 {
                    pos
                } else {
                    pos - 1
                }
            }
            &Instruction::Right => {
                if (pos % 3) == 0 {
                    pos
                } else {
                    pos + 1
                }
            }
        }
    }

    fn go2(&self, pos: i32) -> i32 {
        if pos == 1 {
            match self {
                &Instruction::Down => 3,
                _ => 1,
            }
        } else if pos < 5 {
            match self {
                &Instruction::Up => {
                    if pos == 3 {
                        1
                    } else {
                        pos
                    }
                }
                &Instruction::Down => pos + 4,
                &Instruction::Left => {
                    if pos == 2 {
                        pos
                    } else {
                        pos - 1
                    }
                }
                &Instruction::Right => {
                    if pos == 4 {
                        pos
                    } else {
                        pos + 1
                    }
                }
            }
        } else if pos < 10 {
            match self {
                &Instruction::Up => {
                    if (pos == 5) || (pos == 9) {
                        pos
                    } else {
                        pos - 4
                    }
                }
                &Instruction::Down => {
                    if (pos == 5) || (pos == 9) {
                        pos
                    } else {
                        pos + 4
                    }
                }
                &Instruction::Left => {
                    if pos == 5 {
                        pos
                    } else {
                        pos - 1
                    }
                }
                &Instruction::Right => {
                    if pos == 9 {
                        pos
                    } else {
                        pos + 1
                    }
                }
            }
        } else if pos < 13 {
            match self {
                &Instruction::Up => pos - 4,
                &Instruction::Down => {
                    if pos == 11 {
                        13
                    } else {
                        pos
                    }
                }
                &Instruction::Left => {
                    if pos == 10 {
                        10
                    } else {
                        pos - 1
                    }
                }
                &Instruction::Right => {
                    if pos == 12 {
                        12
                    } else {
                        pos + 1
                    }
                }
            }
        } else {
            match self {
                &Instruction::Up => 11,
                _ => 13,
            }
        }
    }
}

fn load_instructions() -> Vec<Vec<Instruction>> {
    let infile = File::open("day2.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<Vec<Instruction>> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        let mut lvec: Vec<Instruction> = Vec::new();
        for instr in line.chars() {
            lvec.push(Instruction::new(instr));
        }
        ret.push(lvec);
    }
    return ret;
}

fn problem1() {
    let instructions = load_instructions();
    print!("Result of problem 1: ");
    for lvec in instructions.iter() {
        let mut pos = 5;
        for instr in lvec.iter() {
            pos = instr.go(pos);
        }
        print!("{}", pos);
    }
    println!("");
}

fn problem2() {
    let instructions = load_instructions();
    print!("Result of problem 2: ");
    let buttons = "-123456789ABCD";
    for lvec in instructions.iter() {
        let mut pos = 5;
        for instr in lvec.iter() {
            pos = instr.go2(pos);
        }
        let ch = buttons.chars().nth(pos as usize).unwrap();
        print!("{}", ch);
    }
    println!("");
}

fn main() {
    problem1();
    problem2();
}
