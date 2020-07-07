#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Particle {
    n: usize,
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
    ax: i64,
    ay: i64,
    az: i64,
}

impl Particle {
    fn new(n: usize, s: &str) -> Particle {
        lazy_static! {
            static ref PART_RE: Regex = Regex::new(r"p=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, v=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, a=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>").unwrap();
        }
        if let Some(cap) = PART_RE.captures(s) {
            Particle {
                n: n,
                px: cap.get(1).unwrap().as_str().parse().unwrap(),
                py: cap.get(2).unwrap().as_str().parse().unwrap(),
                pz: cap.get(3).unwrap().as_str().parse().unwrap(),
                vx: cap.get(4).unwrap().as_str().parse().unwrap(),
                vy: cap.get(5).unwrap().as_str().parse().unwrap(),
                vz: cap.get(6).unwrap().as_str().parse().unwrap(),
                ax: cap.get(7).unwrap().as_str().parse().unwrap(),
                ay: cap.get(8).unwrap().as_str().parse().unwrap(),
                az: cap.get(9).unwrap().as_str().parse().unwrap(),
            }
        } else {
            panic!("Unable to parse {:?}", s)
        }
    }

    fn absacc(&self) -> i64 {
        self.ax.abs() + self.ay.abs() + self.az.abs()
    }

    fn tick(&mut self) {
        self.vx += self.ax;
        self.vy += self.ay;
        self.vz += self.az;

        self.px += self.vx;
        self.py += self.vy;
        self.pz += self.vz;
    }
}

fn load_instructions() -> Vec<Particle> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        let part = Particle::new(ret.len(), &line);
        ret.push(part);
    }
    ret
}

struct GPU {
    particles: Vec<Particle>,
}

impl GPU {
    fn new(input: &Vec<Particle>) -> GPU {
        GPU {
            particles: input.clone(),
        }
    }

    fn tick(&mut self) {
        for i in 0..self.particles.len() {
            self.particles[i].tick();
        }
    }

    fn collide(&mut self) {
        let mut seenpos: HashSet<(i64, i64, i64)> = HashSet::new();
        let mut collided: HashSet<(i64, i64, i64)> = HashSet::new();
        let mut oldparts = self.particles.clone();

        for part in &oldparts {
            let pos = (part.px, part.py, part.pz);
            if seenpos.contains(&pos) {
                collided.insert(pos);
            } else {
                seenpos.insert(pos);
            }
        }

        if collided.len() > 0 {
            self.particles = oldparts
                .drain(..)
                .filter(|p| !collided.contains(&(p.px, p.py, p.pz)))
                .collect();
        }
    }
}

fn problem1(input: &Vec<Particle>) -> usize {
    let mut parts = input.clone();
    parts.sort_by(|a, b| a.absacc().cmp(&b.absacc()));
    parts[0].n
}

fn problem2(input: &Vec<Particle>) -> usize {
    let mut gpu = GPU::new(input);
    let mut count = 1;
    let mut lastlen = gpu.particles.len();
    loop {
        gpu.tick();
        gpu.collide();
        if gpu.particles.len() == lastlen {
            count += 1;
            if count == 10_000 {
                break;
            }
        } else {
            count = 1;
            lastlen = gpu.particles.len();
        }
    }
    gpu.particles.len()
}

fn main() {
    let input = load_instructions();
    println!("Loaded {} particles!", input.len());
    println!("Problem 1: {}", problem1(&input));
    println!("Problem 2: {}", problem2(&input));
}
