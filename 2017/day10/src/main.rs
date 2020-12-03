use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::vec::Vec;

fn load_instructions() -> Vec<usize> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        for num_ in line.split(',') {
            ret.push(usize::from_str(num_).unwrap());
        }
    }
    ret
}

fn load_instr2_from(s: &str) -> Vec<usize> {
    let mut ret = Vec::new();
    for ch in s.chars() {
        ret.push(ch as usize);
    }
    // 17, 31, 73, 47, 23
    ret.push(17);
    ret.push(31);
    ret.push(73);
    ret.push(47);
    ret.push(23);
    ret
}

fn load_instructions_2() -> Vec<usize> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let line = freader.lines().next().unwrap().unwrap();
    load_instr2_from(&line)
}

struct KnotHash {
    size: usize,
    entries: Vec<usize>,
    curpos: usize,
    skip: usize,
}

impl KnotHash {
    fn new(size: usize) -> KnotHash {
        let mut ret = KnotHash {
            size,
            entries: Vec::new(),
            curpos: 0,
            skip: 0,
        };
        for v in 0..size {
            ret.entries.push(v);
        }
        ret
    }

    fn print(&self) {
        for i in 0..self.size {
            if self.curpos == i {
                print!("[{}] ", self.entries[i]);
            } else {
                print!("{} ", self.entries[i]);
            }
        }
        println!("  S={}", self.skip);
    }

    fn run_instruction(&mut self, len: usize) {
        let revvec: Vec<usize> = (0..len)
            .rev()
            .map(|v| (self.curpos + v) % self.size)
            .map(|p| self.entries[p])
            .collect();
        #[allow(clippy::needless_range_loop)]
        for p in 0..len {
            self.entries[(self.curpos + p) % self.size] = revvec[p];
        }
        self.curpos = (self.curpos + len + self.skip) % self.size;
        self.skip += 1;
    }

    fn run_prog(&mut self, prog: &[usize], printing: bool) {
        for elem in prog {
            self.run_instruction(*elem);
            if printing {
                self.print();
            }
        }
    }

    fn check_value(&self) -> usize {
        self.entries[0] * self.entries[1]
    }

    fn run_rounds(&mut self, prog: &[usize]) {
        for _i in 0..64 {
            self.run_prog(prog, false);
        }
    }

    fn dense_hash(&self) -> String {
        let mut ret = String::new();
        for base in 0..16 {
            let mut val = 0;
            for sub in 0..16 {
                val ^= self.entries[(base * 16) + sub];
            }
            ret.push_str(&format!("{:02x}", val));
        }
        ret
    }
}

fn problem1(prog: &[usize]) -> usize {
    let mut knot = KnotHash::new(256);
    knot.run_prog(prog, false);
    knot.check_value()
}

fn problem2(prog: &[usize]) -> String {
    let mut knot = KnotHash::new(256);
    knot.run_rounds(prog);

    knot.dense_hash()
}

fn main() {
    let mut testknot = KnotHash::new(5);
    let testinput = vec![3, 4, 1, 5];
    testknot.print();
    testknot.run_prog(&testinput, true);
    assert!(testknot.check_value() == 12);
    let input = load_instructions();
    println!("Loaded {} entries from input", input.len());
    println!("Problem 1: {}", problem1(&input));

    {
        let testinput = load_instr2_from("1,2,3");
        println!("testinput = {:?}", testinput);
        let mut knot = KnotHash::new(256);
        knot.run_rounds(&testinput);
        println!("hash = {}", knot.dense_hash());
    }

    let input2 = load_instructions_2();
    println!("Loaded {} entries from input", input2.len());
    println!("Problem 2: {}", problem2(&input2));
}
