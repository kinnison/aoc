#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug, Clone)]
enum Instr {
    Rect(usize, usize),
    RotRow(usize, usize),
    RotCol(usize, usize),
}

impl Instr {
    fn new(t_: String) -> Instr {
        lazy_static! {
            static ref RECT_RE: Regex = Regex::new("^rect ([0-9]+)x([0-9+])$").unwrap();
            static ref ROT_ROW_RE: Regex =
                Regex::new("^rotate row y=([0-9]+) by ([0-9]+)$").unwrap();
            static ref ROT_COL_RE: Regex =
                Regex::new("^rotate column x=([0-9]+) by ([0-9]+)$").unwrap();
        }
        let mut ret: Instr = Instr::Rect(0, 0);
        if RECT_RE.is_match(&t_) {
            for cap in RECT_RE.captures_iter(&t_) {
                let ref width_ = cap.get(1);
                let ref height_ = cap.get(2);
                let width: usize = width_.unwrap().as_str().parse().unwrap();
                let height: usize = height_.unwrap().as_str().parse().unwrap();
                ret = Instr::Rect(width, height);
            }
        } else if ROT_ROW_RE.is_match(&t_) {
            for cap in ROT_ROW_RE.captures_iter(&t_) {
                let ref y_ = cap.get(1);
                let ref by_ = cap.get(2);
                let y: usize = y_.unwrap().as_str().parse().unwrap();
                let by: usize = by_.unwrap().as_str().parse().unwrap();
                ret = Instr::RotRow(y, by);
            }
        } else if ROT_COL_RE.is_match(&t_) {
            for cap in ROT_COL_RE.captures_iter(&t_) {
                let ref x_ = cap.get(1);
                let ref by_ = cap.get(2);
                let x: usize = x_.unwrap().as_str().parse().unwrap();
                let by: usize = by_.unwrap().as_str().parse().unwrap();
                ret = Instr::RotCol(x, by);
            }
        } else {
            panic!(t_);
        }
        ret
    }
}

struct Screen {
    pixels: [bool; 50 * 6],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: [false; 50 * 6],
        }
    }

    fn count(&self) -> usize {
        self.pixels
            .into_iter()
            .fold(0, |aa, &p| if p { aa + 1 } else { aa })
    }

    fn get_pix(&self, x: usize, y: usize) -> bool {
        self.pixels[(y * 50) + x]
    }

    fn set_pix(&mut self, x: usize, y: usize, val: bool) {
        self.pixels[(y * 50) + x] = val;
    }

    fn run_instr(&mut self, instr: &Instr) {
        match *instr {
            Instr::Rect(width, height) => {
                for x in 0..width {
                    for y in 0..height {
                        self.set_pix(x, y, true);
                    }
                }
            }
            Instr::RotRow(y, n) => {
                for i in 0..n {
                    let lastpix = self.get_pix(49, y);
                    for j in 1..50 {
                        let p1 = self.get_pix(49 - j, y);
                        self.set_pix(50 - j, y, p1);
                    }
                    self.set_pix(0, y, lastpix);
                }
            }
            Instr::RotCol(x, n) => {
                for i in 0..n {
                    let lastpix = self.get_pix(x, 5);
                    for j in 1..6 {
                        let p1 = self.get_pix(x, 5 - j);
                        self.set_pix(x, 6 - j, p1);
                    }
                    self.set_pix(x, 0, lastpix);
                }
            }
        }
    }
    fn show(&self) {
        for row in 0..6 {
            for ch in 0..10 {
                for pix in 0..5 {
                    if self.get_pix((ch * 5) + pix, row) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                print!(" ");
            }
            print!("\n");
        }
    }
}

fn load_instructions() -> Vec<Instr> {
    let infile = File::open("day8.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<Instr> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Instr::new(line));
    }
    return ret;
}

fn problem1() -> usize {
    let instrs = load_instructions();
    let mut screen = Screen::new();
    for instr in &instrs {
        screen.run_instr(instr);
    }
    return screen.count();
}

fn problem2() {
    let instrs = load_instructions();
    let mut screen = Screen::new();
    for instr in &instrs {
        screen.run_instr(instr);
    }
    screen.show();
}

fn main() {
    println!("Result 1: {}", problem1());
    problem2();
}
