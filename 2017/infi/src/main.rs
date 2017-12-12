#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashSet;
use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

#[derive(Clone)]
struct State {
    robots: Vec<(i32, i32)>,
    moves: Vec<(i32, i32)>
}

impl State {
    fn new () -> State {
        State { robots: Vec::new(), moves: Vec::new() }
    }

    fn load (&mut self, line: &str) {
        lazy_static! {
            static ref ROBOTS_RE: Regex = Regex::new(r"\[(-?[0-9]+),(-?[0-9]+)\]").unwrap();
            static ref MOVES_RE: Regex = Regex::new(r"\((-?[0-9]+),(-?[0-9]+)\)").unwrap();
        }
        for cap in ROBOTS_RE.captures_iter(line) {
            self.robots.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
        }
        for cap in MOVES_RE.captures_iter(line) {
            self.moves.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
        }
    }

    fn collided (&self) -> bool {
        // We're collided if all the robots are in the same location
        let targ = self.robots[0];
        let mut same = true;
        for loc in &self.robots {
            if *loc != targ {
                same = false;
            }
        }
        same
    }

    fn run_moves (&mut self) -> usize {
        // returns the number of times the robots collide
        let mut collided = 0;
        let mut robnr = 0;
        for move_ in &self.moves {
            let robpos = self.robots[robnr];
            let newpos = (robpos.0 + move_.0, robpos.1 + move_.1);
            self.robots[robnr] = newpos;
            robnr = (robnr + 1) % self.robots.len();
            if self.collided() {
                collided += 1;
            }
        }

        collided
    }

    fn all_locs (&mut self) -> Vec<(i32, i32)> {
        // returns the number of times the robots collide
        let mut robnr = 0;
        let mut ret = self.robots.clone();
        for move_ in &self.moves {
            let robpos = self.robots[robnr];
            let newpos = (robpos.0 + move_.0, robpos.1 + move_.1);
            self.robots[robnr] = newpos;
            robnr = (robnr + 1) % self.robots.len();
            ret.push(newpos);
        }
        ret
    }
}

fn load_instructions () -> State {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = State::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.load(&line);
    }
    ret
}

fn pattern (mut state: State) {
    let poss = state.all_locs();
    let (mut minx, mut miny) = poss[0];
    for pos in &poss {
        if pos.0 < minx {
            minx = pos.0;
        }
        if pos.1 < miny {
            miny = pos.1;
        }
    }
    // Now we generate normalised coords...
    println!("normalising by {}, {}", -minx, -miny);
    let newposs: Vec<(i32, i32)> = poss.iter().map(|&(x,y)| (x-minx,y-miny)).collect();
    let (mut maxx, mut maxy) = newposs[0];
    for pos in &newposs {
        if pos.0 > maxx {
            maxx = pos.0;
        }
        if pos.1 > maxy {
            maxy = pos.1;
        }
    }
    println!("Image is {} by {} pixels", maxx, maxy);
    let pixels: HashSet<(i32,i32)> = newposs.iter().map(|&(x,y)|(x,y)).collect();
    for x in 0..maxx+1 {
        for y in 0..maxy+1 {
            if pixels.contains(&(x,y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let instate = load_instructions();
    println!("Loaded {} robots with {} moves total", instate.robots.len(), instate.moves.len());
    println!("There's a total of {} collisions", instate.clone().run_moves());
    pattern(instate.clone());
}
