use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Facing {
    North, South, East, West
}

impl Facing {
    fn turn(&self, left : bool) -> Facing {
        match self {
            &Facing::North => if left { Facing::West } else { Facing::East },
            &Facing::South => if left { Facing::East } else { Facing::West },
            &Facing::East => if left { Facing::North } else { Facing::South },
            &Facing::West => if left { Facing::South } else { Facing::North }
        }
    }
}

struct Instruction {
    left : bool,
    dist : i32
}

impl Instruction {
    pub fn new(instr : &str) -> Instruction {
        let mut left = false;
        if instr.split_at(1).0 == "L" {
            left = true;
        }
        return Instruction { left: left, dist: instr.split_at(1).1.parse::<i32>().unwrap() }
    }

    fn follow(&self, facing : Facing, xpos : i32, ypos : i32) -> (Facing, i32, i32) {
        let turned = facing.turn(self.left);
        match turned {
            Facing::North => (turned, xpos, ypos + self.dist),
            Facing::South => (turned, xpos, ypos - self.dist),
            Facing::East => (turned, xpos + self.dist, ypos),
            Facing::West => (turned, xpos - self.dist, ypos)
        }
    }

    fn follow_slowly(&self, facing : Facing, xpos : i32, ypos : i32) -> Vec<(i32,i32)> {
        let turned = facing.turn(self.left);
        let step = match turned {
            Facing::North => (0, 1),
            Facing::South => (0, -1),
            Facing::East => (1, 0),
            Facing::West => (-1, 0)
        };
        let mut ret : Vec<(i32,i32)> = Vec::new();
        for i in 1..self.dist {
            ret.push((xpos + (i * step.0), ypos + (i * step.1)));
        }
        return ret;
    }
}

fn load_instructions () -> Vec<Instruction> {
    let infile = File::open("day1.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret : Vec<Instruction> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        let elems = line.split(", ");
        for instr in elems {
            ret.push(Instruction::new(instr));
        }
    }
    return ret;
}

fn abs (n:i32) -> i32 { if n < 0 { -n } else { n } }

fn problem1 () -> i32 {
    let instructions = load_instructions();
    let mut facing = Facing::North;
    let mut xpos = 0;
    let mut ypos = 0;
    for instr in instructions.iter() {
        let (facing_, xpos_, ypos_) = instr.follow(facing, xpos, ypos);
        facing = facing_;
        xpos = xpos_;
        ypos = ypos_;
    }
    return abs(xpos) + abs(ypos);
}

fn problem2 () -> i32 {
    let instructions = load_instructions();
    let mut facing = Facing::North;
    let mut xpos = 0;
    let mut ypos = 0;
    let mut visited = HashSet::new();
    for instr in instructions.iter() {
        let poss = instr.follow_slowly(facing, xpos, ypos);
        let (facing_, xpos_, ypos_) = instr.follow(facing, xpos, ypos);
        facing = facing_;
        xpos = xpos_;
        ypos = ypos_;
        for pos in poss.iter() {
            if visited.contains(pos) {
                return abs(pos.0) + abs(pos.1);
            }
            visited.insert((pos.0, pos.1));
        }
    }
    return 0;
}

fn main () {
    println!("Result 1: {}" , problem1());
    println!("Result 2: {}" , problem2());
}
