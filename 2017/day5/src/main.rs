use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

fn load_instructions() -> Vec<i32> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(line.parse().unwrap());
    }
    ret
}

fn problem1(istr_: &Vec<i32>) -> usize {
    let mut prog: Vec<i32> = istr_.clone();
    let mut ip = 0;
    let mut ret = 0;
    loop {
        let jump = prog[ip];
        let newip = ((ip as i32) + jump) as usize;
        prog[ip] = jump + 1;
        ret += 1;
        if newip >= prog.len() {
            return ret;
        }
        ip = newip;
    }
}

fn problem2(istr_: &Vec<i32>) -> usize {
    let mut prog: Vec<i32> = istr_.clone();
    let mut ip = 0;
    let mut ret = 0;
    loop {
        let jump = prog[ip];
        let newip = ((ip as i32) + jump) as usize;
        if jump >= 3 {
            prog[ip] = jump - 1;
        } else {
            prog[ip] = jump + 1;
        }
        ret += 1;
        if newip >= prog.len() {
            return ret;
        }
        ip = newip;
    }
}

fn main() {
    let istr: Vec<i32> = load_instructions();
    println!("Loaded {} instructions", istr.len());
    println!("Problem 1: {}", problem1(&istr));
    println!("Problem 2: {}", problem2(&istr));
}
