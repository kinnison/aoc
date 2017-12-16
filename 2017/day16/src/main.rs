#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

use regex::Regex;

enum Inst {
    Spin(usize),
    Exch(usize,usize),
    Part(char, char)
}

impl Inst {
    fn new (s: &str) -> Inst {
        lazy_static! {
            static ref SPIN_RE: Regex = Regex::new(r"^s([0-9]+)$").unwrap();
            static ref EXCH_RE: Regex = Regex::new(r"^x([0-9]+)/([0-9]+)$").unwrap();
            static ref PART_RE: Regex = Regex::new(r"^p([a-p])/([a-p])$").unwrap();
        }

        if let Some(cap) = SPIN_RE.captures(s) {
            let val = cap.get(1).unwrap().as_str().parse().unwrap();
            return Inst::Spin(val);
        } else if let Some(cap) = EXCH_RE.captures(s) {
            let val1 = cap.get(1).unwrap().as_str().parse().unwrap();
            let val2 = cap.get(2).unwrap().as_str().parse().unwrap();
            return Inst::Exch(val1, val2);
        } else if let Some(cap) = PART_RE.captures(s) {
            let prog1: char = cap.get(1).unwrap().as_str().chars().next().unwrap();
            let prog2: char = cap.get(2).unwrap().as_str().chars().next().unwrap();
            return Inst::Part(prog1, prog2);
        }
        panic!("Unable to parse instruction: {:?}", s);
    }
}

fn load_instructions () -> Vec<Inst> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        for chunk in line.split(",") {
            ret.push(Inst::new(chunk));
        }
    }
    ret
}

struct Lineup {
    progs: [char; 16]
}

impl Lineup {
    fn new () -> Lineup {
        Lineup { progs: ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'] }
    }

    fn do_instr (&mut self, inst: &Inst) {
        match inst {
            &Inst::Spin(count) => self.do_spin(count),
            &Inst::Exch(a,b) => self.do_exch(a,b),
            &Inst::Part(a,b) => self.do_part(a,b)
        }
    }

    fn do_spin (&mut self, count: usize) {
        let old_lineup = self.progs;
        for i in 0..self.progs.len() {
            self.progs[i] = old_lineup[((self.progs.len() - count) + i) % 16];
        }
    }

    fn do_exch (&mut self, a: usize, b: usize) {
        let temp = self.progs[a];
        self.progs[a] = self.progs[b];
        self.progs[b] = temp;
    }

    fn do_part (&mut self, a: char, b: char) {
        for i in 0..self.progs.len() {
            if self.progs[i] == a {
                self.progs[i] = b;
            } else if self.progs[i] == b {
                self.progs[i] = a;
            }
        }
    }

    fn run_instr_vec(&mut self, v: &Vec<Inst>) {
        for inst in v {
            self.do_instr(inst);
        }
    }
}

fn problem1 (input: &Vec<Inst>) -> String {
    let mut lineup = Lineup::new();
    lineup.run_instr_vec(input);
    lineup.progs.iter().collect()
}

fn cycle_size (input: &Vec<Inst>) -> usize {
    let mut lineup = Lineup::new();
    let mut maps: HashMap<String, usize> = HashMap::new();
    let mut i = 0;
    while i < 1_000_000_000 {
        let s = lineup.progs.iter().collect();
        if maps.contains_key(&s) {
            // Cycle length is i - maps[s]
            return i - maps[&s];
        } else {
            maps.insert(s, i);
        }
        i += 1;
        lineup.run_instr_vec(input);
        if (i % 10_000_000) == 0 {
            println!("We're at {}%", i / 10_000_000);
        }
    }
    i
}

fn problem2 (input: &Vec<Inst>) -> String {
    let cyc = cycle_size(input);
    let mut lineup = Lineup::new();
    for _ in 0..(1_000_000_000 % cyc) {
        lineup.run_instr_vec(input);
    }
    lineup.progs.iter().collect()
}

fn main() {
    let input = load_instructions();
    println!("Loaded {} instructions", input.len());
    println!("Problem 1: {}", problem1(&input));
    println!("Problem 2: {}", problem2(&input));
}
