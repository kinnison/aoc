#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

struct Prog {
    id: usize,
    pipes: Vec<usize>,
}

impl Prog {
    fn new(line: &str) -> Prog {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^([0-9]+) <-> (.*)$").unwrap();
        }

        if let Some(cap) = LINE_RE.captures(line) {
            let id_ = cap.get(1).unwrap().as_str().parse().unwrap();
            let rest_ = cap.get(2).unwrap().as_str();
            let mut pipes_: Vec<usize> = Vec::new();
            for num_ in rest_.split(",") {
                pipes_.push(num_.trim().parse().unwrap());
            }
            return Prog {
                id: id_,
                pipes: pipes_,
            };
        }
        panic!("Unable to parse line at all")
    }

    fn connected_progs(start: usize, all: &HashMap<usize, Prog>) -> HashSet<usize> {
        let mut ret = HashSet::new();
        let mut to_check = vec![start];
        while to_check.len() > 0 {
            let id = to_check.pop().unwrap();
            let dis = all.get(&id).unwrap();
            if !ret.contains(&id) {
                ret.insert(id);
                for other in &dis.pipes {
                    if !ret.contains(&other) {
                        to_check.push(*other);
                    }
                }
            }
        }
        ret
    }

    fn group_count(all: &HashMap<usize, Prog>) -> usize {
        let mut groups = 0;
        let mut progsleft: HashSet<usize> = all.keys().map(|c| *c).collect();
        while progsleft.len() > 0 {
            let someid = *(progsleft.iter().last().unwrap());
            let connset = Prog::connected_progs(someid, all);
            groups += 1;
            for id in connset {
                progsleft.remove(&id);
            }
        }
        groups
    }
}

fn load_instructions() -> HashMap<usize, Prog> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = HashMap::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        let prog = Prog::new(&line);
        ret.insert(prog.id, prog);
    }
    ret
}

fn main() {
    let input = load_instructions();
    println!("Loaded {} programs", input.len());
    println!("Problem 1: {}", Prog::connected_progs(0, &input).len());
    println!("Problem 2: {}", Prog::group_count(&input));
}
