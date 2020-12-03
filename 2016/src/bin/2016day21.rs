#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug, Clone)]
enum Instr {
    Swap(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePos(u8),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl Instr {
    fn new(t_: String) -> Instr {
        lazy_static! {
            static ref SWAP_POS_RE: Regex =
                Regex::new("swap position ([0-9]+) with position ([0-9]+)").unwrap();
            static ref SWAP_LET_RE: Regex = Regex::new("swap letter (.) with letter (.)").unwrap();
            static ref ROT_LEFT_RE: Regex = Regex::new("rotate left ([0-9]+) steps?").unwrap();
            static ref ROT_RIGHT_RE: Regex = Regex::new("rotate right ([0-9]+) steps?").unwrap();
            static ref ROT_LETTR_RE: Regex =
                Regex::new("rotate based on position of letter (.)").unwrap();
            static ref REVERSE_RE: Regex =
                Regex::new("reverse positions ([0-9]+) through ([0-9]+)").unwrap();
            static ref MOVE_RE: Regex =
                Regex::new("move position ([0-9]+) to position ([0-9]+)").unwrap();
        }
        if let Some(cap) = SWAP_POS_RE.captures(&t_) {
            let x_ = cap.get(1);
            let y_ = cap.get(2);
            let x: usize = x_.unwrap().as_str().parse().unwrap();
            let y: usize = y_.unwrap().as_str().parse().unwrap();
            return Instr::Swap(x, y);
        }
        if let Some(cap) = SWAP_LET_RE.captures(&t_) {
            let x_ = cap.get(1);
            let y_ = cap.get(2);
            let x: char = x_.unwrap().as_str().chars().next().unwrap();
            let y: char = y_.unwrap().as_str().chars().next().unwrap();
            return Instr::SwapLetter(x as u8, y as u8);
        }
        if let Some(cap) = ROT_LEFT_RE.captures(&t_) {
            let s_ = cap.get(1);
            let steps: usize = s_.unwrap().as_str().parse().unwrap();
            return Instr::RotateLeft(steps);
        }
        if let Some(cap) = ROT_RIGHT_RE.captures(&t_) {
            let s_ = cap.get(1);
            let steps: usize = s_.unwrap().as_str().parse().unwrap();
            return Instr::RotateRight(steps);
        }
        if let Some(cap) = ROT_LETTR_RE.captures(&t_) {
            let l_ = cap.get(1);
            let letter: char = l_.unwrap().as_str().chars().next().unwrap();
            return Instr::RotatePos(letter as u8);
        }
        if let Some(cap) = REVERSE_RE.captures(&t_) {
            let x_ = cap.get(1);
            let y_ = cap.get(2);
            let x: usize = x_.unwrap().as_str().parse().unwrap();
            let y: usize = y_.unwrap().as_str().parse().unwrap();
            return Instr::Reverse(x, y);
        }
        if let Some(cap) = MOVE_RE.captures(&t_) {
            let x_ = cap.get(1);
            let y_ = cap.get(2);
            let x: usize = x_.unwrap().as_str().parse().unwrap();
            let y: usize = y_.unwrap().as_str().parse().unwrap();
            return Instr::Move(x, y);
        }
        panic!("Input: '{}' unparseable", t_);
    }

    fn perform_step(&self, inp: String) -> String {
        let mut vret: Vec<u8> = inp.into();
        match *self {
            Instr::Swap(x, y) => {
                vret.swap(x, y);
            }
            Instr::SwapLetter(x_, y_) => {
                let x = vret.iter().enumerate().find(|n| *n.1 == x_).unwrap().0;
                let y = vret.iter().enumerate().find(|n| *n.1 == y_).unwrap().0;
                vret.swap(x, y);
            }
            Instr::RotateLeft(n) => {
                // To rotate left, push the result of remove(0)
                for _ in 0..n {
                    let ch = vret.remove(0);
                    vret.push(ch);
                }
            }
            Instr::RotateRight(n) => {
                // To rotate right, insert at 0 the result of pop
                for _ in 0..n {
                    let ch = vret.pop().unwrap();
                    vret.insert(0, ch);
                }
            }
            Instr::RotatePos(ch) => {
                // Rotate based on ch's pos
                let pos = vret.iter().enumerate().find(|n| *n.1 == ch).unwrap().0;
                let cnt = 1 + pos + if pos >= 4 { 1 } else { 0 };
                for _ in 0..cnt {
                    let ch = vret.pop().unwrap();
                    vret.insert(0, ch);
                }
            }
            Instr::Reverse(x, y) => {
                // Reversal of an even size range means no middle to leave
                // alone, reversal of an odd size range means a middle to leave
                // either way, len of range >> 1 gives us the number of swaps
                // to do, and swapping x+n with y-n does the trick
                for n in 0..(((y - x) + 1) >> 1) {
                    vret.swap(x + n, y - n);
                }
            }
            Instr::Move(x, y) => {
                let ch = vret.remove(x);
                vret.insert(y, ch);
            }
        };
        String::from_utf8(vret).unwrap()
    }

    fn reverse_step(&self, inp: String) -> String {
        let mut vret: Vec<u8> = inp.into();
        match *self {
            Instr::Swap(x, y) => {
                vret.swap(x, y);
            }
            Instr::SwapLetter(x_, y_) => {
                let x = vret.iter().enumerate().find(|n| *n.1 == x_).unwrap().0;
                let y = vret.iter().enumerate().find(|n| *n.1 == y_).unwrap().0;
                vret.swap(x, y);
            }
            Instr::RotateLeft(n) => {
                // To rotate right, insert at 0 the result of pop
                for _ in 0..n {
                    let ch = vret.pop().unwrap();
                    vret.insert(0, ch);
                }
            }
            Instr::RotateRight(n) => {
                // To rotate left, push the result of remove(0)
                for _ in 0..n {
                    let ch = vret.remove(0);
                    vret.push(ch);
                }
            }
            Instr::RotatePos(ch) => {
                fn forwrot(pos: usize, l: usize) -> usize {
                    ((1 + pos + if pos >= 4 { 1 } else { 0 }) + pos) % l
                }
                fn backpos(pos: usize, l: usize) -> usize {
                    for i in 0..l {
                        if forwrot(i, l) == pos {
                            return i;
                        }
                    }
                    println!("Cannot reverse position {} for length {}", pos, l);
                    unreachable!();
                }
                // TODO: Ponder Rotate based on ch's pos
                let pos = vret.iter().enumerate().find(|n| *n.1 == ch).unwrap().0;
                let bpos = backpos(pos, vret.len());
                // if pos > bpos, left by pos-bpos
                // if pos < bpos, left by len + pos - bpos
                let cnt = vret.len() + pos - bpos;
                for _ in 0..cnt {
                    let ch = vret.remove(0);
                    vret.push(ch);
                }
            }
            Instr::Reverse(x, y) => {
                // Reversal of an even size range means no middle to leave
                // alone, reversal of an odd size range means a middle to leave
                // either way, len of range >> 1 gives us the number of swaps
                // to do, and swapping x+n with y-n does the trick
                for n in 0..(((y - x) + 1) >> 1) {
                    vret.swap(x + n, y - n);
                }
            }
            Instr::Move(x, y) => {
                let ch = vret.remove(y);
                vret.insert(x, ch);
            }
        };
        String::from_utf8(vret).unwrap()
    }
}

fn load_instructions() -> Vec<Instr> {
    let infile = File::open("day21.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<Instr> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Instr::new(line));
    }
    ret
}

fn problem1(inp: &str) -> String {
    let instrs = load_instructions();
    let mut pass = inp.to_owned();
    for instr in &instrs {
        pass = instr.perform_step(pass);
    }
    pass
}

fn problem2(inp: &str) -> String {
    let mut instrs = load_instructions();
    instrs.reverse();
    let mut pass = inp.to_owned();
    for instr in &instrs {
        pass = instr.reverse_step(pass);
    }
    pass
}

fn main() {
    println!("Problem 1: {}", problem1("abcdefgh"));
    println!("Problem 2: {}", problem2("fbgdceah"));
}
