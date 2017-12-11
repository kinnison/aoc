
use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
enum Move {
    N, S, NE, NW, SE, SW
}

impl Move {
    fn from_str(s: &str) -> Move {
        match s {
            "n"  => Move::N,
            "s"  => Move::S,
            "ne" => Move::NE,
            "nw" => Move::NW,
            "se" => Move::SE,
            "sw" => Move::SW,
            _    => panic!("Unknown move: {}", s)
        }
    }
}

fn load_instructions () -> Vec<Move> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        for move_ in line.split(",") {
            ret.push(Move::from_str(move_));
        }
    }
    ret
}

// Our Hex Position is based on X being horizontal
// Y being nw/sw
// Z being ne/sw
// The origin of the grid being 0,0,0 obviously
// Interestingly we know that x+y+z always is zero
#[derive(Debug, Clone)]
struct HexPos {
    x: i32,
    y: i32,
    z: i32,
}

impl HexPos {
    fn new(x: i32, y: i32, z: i32) -> HexPos {
        HexPos { x: x, y: y, z: z }
    }

    fn domove(&mut self, move_: &Move) {
        match *move_ {
            Move::N  => { self.y += 1; self.z -= 1 },
            Move::S  => { self.y -= 1; self.z += 1 },
            Move::NE => { self.x += 1; self.z -= 1 },
            Move::SW => { self.x -= 1; self.z += 1 },
            Move::SE => { self.x += 1; self.y -= 1 },
            Move::NW => { self.x -= 1; self.y += 1 }
        }
    }

    fn doseq(&mut self, seq: &Vec<Move>) {
        //println!("Starting move sequence at {:?}", self);
        for move_ in seq {
            self.domove(move_);
            //println!("Move {:?} => {:?}", move_, self);
        }
    }

    fn doseq_farthest(&mut self, seq: &Vec<Move>) -> usize {
        //println!("Starting move sequence at {:?}", self);
        let mut dist = self.dist_to_origin();
        for move_ in seq {
            self.domove(move_);
            //println!("Move {:?} => {:?}", move_, self);
            let newdist = self.dist_to_origin();
            if newdist > dist {
                dist = newdist;
            }
        }
        dist
    }

    fn dist_to_origin(&self) -> usize {
        // Easiest way to get a distance to origin is to compute first
        // the distance to one of the diagonals, which can be done by
        // looking at the absolute smallest coordinate...
        //println!("Normalise starts: {:?}", self);
        let normmove;
        if (self.x.abs() < self.y.abs()) && (self.x.abs() < self.z.abs()) {
            // x is smallest
            //println!("X is smallest");
            if self.x < 0 {
                normmove = Move::NE;
            } else {
                normmove = Move::SW;
            }
        } else if (self.y.abs() < self.x.abs()) && (self.y.abs() < self.z.abs()) {
            // y is smallest
            //println!("y is smallest");
            if self.y < 0 {
                normmove = Move::N;
            } else {
                normmove = Move::S;
            }
        } else {
            // z is smallest
            //println!("z is smallest");
            if self.z < 0 {
                normmove = Move::SW;
            } else {
                normmove = Move::NE;
            }
        }
        let mut normpos = self.clone();
        let mut moves: usize = 0;
        while (normpos.x != 0) && (normpos.y != 0) && (normpos.z != 0) {
            //println!("Normalise move: {:?}", normmove);
            normpos.domove(&normmove);
            //println!("  => {:?}", normpos);
            moves += 1;
        }
        //println!("Normalised to coord: {:?}", normpos);
        // Normalised to a diagonal.  Take the max, and add that
        if (normpos.x > normpos.y) && (normpos.x > normpos.z) {
            moves + (normpos.x as usize)
        } else if (normpos.y > normpos.x) && (normpos.y > normpos.z) {
            moves + (normpos.y as usize)
        } else {
            moves + (normpos.z as usize)
        }
    }
}

fn runtest1(seq: Vec<Move>, expdist: usize) {
    let mut pos = HexPos::new(0, 0, 0);
    pos.doseq(&seq);
    let dist = pos.dist_to_origin();
    println!("After seq, coords are: {:?} which is {} from orig (want {})",
             pos, dist, expdist);
    assert!(dist == expdist);
}

fn problem1 (input: &Vec<Move>) -> usize {
    let mut pos = HexPos::new(0, 0, 0);
    pos.doseq(input);
    pos.dist_to_origin()
}

fn problem2 (input: &Vec<Move>) -> usize {
    let mut pos = HexPos::new(0, 0, 0);
    pos.doseq_farthest(input)
}

fn main() {
    runtest1(vec![Move::NE, Move::NE, Move::NE], 3);
    runtest1(vec![Move::NE, Move::NE, Move::SW, Move::SW], 0);
    runtest1(vec![Move::NE, Move::NE, Move::S, Move::S], 2);
    runtest1(vec![Move::SE, Move::SW, Move::SE, Move::SW, Move::SW], 3);
    let moves = load_instructions();
    println!("Loaded {} moves", moves.len());
    println!("Problem 1: {}", problem1(&moves));
    println!("Problem 2: {}", problem2(&moves));
}
