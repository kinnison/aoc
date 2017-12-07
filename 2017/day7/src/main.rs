#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
struct Prog {
    name: String,
    weight: usize,
    kids: Vec<String>,
    parent: Option<String>
}

impl Prog {
    fn new (line: &str) -> Prog {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new(r"^([^ ]+) \(([0-9]+)\)").unwrap();
            static ref KIDS_RE: Regex = Regex::new(r"-> (.+)$").unwrap();
            static ref KIDN_RE: Regex = Regex::new(r"([^ ,]+)").unwrap();
        }
        if let Some(cap1) = NAME_RE.captures(line) {
            let name_ = cap1.get(1).unwrap().as_str();
            let weight_ = cap1.get(2).unwrap().as_str().parse().unwrap();
            let mut kids_ = Vec::new();
            if let Some(cap2) = KIDS_RE.captures(line) {
                for kid in KIDN_RE.find_iter(cap2.get(1).unwrap().as_str()) {
                    kids_.push(kid.as_str().to_owned());
                }
            }
            return Prog {
                name: name_.to_owned(),
                weight: weight_,
                kids: kids_,
                parent: None
            }
        }
        panic!("Unable to parse line at all");
    }
}

struct Tower {
    progs: HashMap<String, Prog>,
    masses: HashMap<String, usize>
}

impl Tower {
    fn new (mut progs_: Vec<Prog>) -> Tower {
        let mut parentmap = HashMap::new();
        for p in &progs_ {
            for sub in &p.kids {
                parentmap.insert(sub.clone(), p.name.clone());
            }
        }
        let mut map = HashMap::new();
        for mut p in progs_.drain(..) {
            p.parent = parentmap.get(&p.name).cloned();
            map.insert(p.name.clone(), p);
        }
        let mut ret = Tower { progs: map, masses: HashMap::new() };
        ret.weigh_everyone();
        ret
    }

    fn root (&self) -> String {
        let mut cur = self.progs.keys().last().unwrap().clone();
        while let Some(ref next) = self.progs.get(&cur).unwrap().parent {
            cur = next.clone();
        }
        return cur.clone();
    }

    fn weigh_everyone (&mut self) {
        let mut to_weigh: Vec<String> = Vec::new();
        to_weigh.push(self.root());
        while to_weigh.len() > 0 {
            let weighing = to_weigh[to_weigh.len()-1].clone();
            let mut possible = true;
            let mut totalkids = 0;
            let prog = self.progs.get(&weighing).unwrap();
            for kid in &prog.kids {
                if !self.masses.contains_key(kid) {
                    to_weigh.push(kid.clone());
                    possible = false;
                } else {
                    totalkids += self.masses.get(kid).unwrap();
                }
            }
            if possible {
                self.masses.insert(weighing.clone(), totalkids + prog.weight);
                to_weigh.pop();
            }
        }
    }

    fn list_unbalanced (&self) {
        self.list_unbalanced_(&self.root());
    }

    fn list_unbalanced_ (&self, cur: &String) {
        let prog = self.progs.get(cur).unwrap();
        if prog.kids.len() > 0 {
            let base = self.masses.get(&prog.kids[0]).unwrap();
            let mut unbalanced = false;
            for k in &prog.kids {
                if self.masses.get(k).unwrap() != base {
                    unbalanced = true;
                }
                self.list_unbalanced_(k);
            }
            if unbalanced {
                self.print_disc(cur);
            }
        }
    }

    fn print_disc (&self, disc: &String) {
        let prog = self.progs.get(disc).unwrap();
        println!("Disc {} has weight {} totalling {}", disc, prog.weight,
                 self.masses.get(disc).unwrap());
        for k in &prog.kids {
            println!("  Kid {} weighs {} totalling {}",
                     k,
                     self.progs.get(k).unwrap().weight,
                     self.masses.get(k).unwrap());
        }
    }
}

fn load_instructions () -> Tower {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Prog::new(&line));
    }
    Tower::new(ret)
}

fn main() {
    let tower = load_instructions();
    println!("Loaded {} programs", tower.progs.len());
    println!("Problem 1 (tower root): {}", tower.root());
    tower.list_unbalanced();
}
