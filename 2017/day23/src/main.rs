#[macro_use]
extern crate lazy_static;
extern crate primal;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

use primal::Sieve;

#[derive(Debug, Clone, Copy)]
enum Val {
    Reg(char),
    Num(i64),
}

impl Val {
    fn new(s: &str) -> Val {
        let fc = s.chars().next().unwrap();
        if fc >= 'a' && fc <= 'z' {
            Val::Reg(fc)
        } else {
            Val::Num(s.parse().unwrap())
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Inst {
    Set(char, Val),
    Sub(char, Val),
    Mul(char, Val),
    Jnz(Val, Val),
}

impl Inst {
    fn new(s: &str) -> Inst {
        lazy_static! {
            static ref TWO_ARG_RE: Regex =
                Regex::new(r"^(set|sub|mul) ([a-z]) (-?[a-z0-9]+)$").unwrap();
            static ref JNZ_ARG_RE: Regex =
                Regex::new(r"^jnz (-?[a-z0-9]+) (-?[a-z0-9]+)$").unwrap();
        }
        if let Some(cap) = TWO_ARG_RE.captures(s) {
            let cap1 = cap.get(1).unwrap().as_str();
            let reg = cap.get(2).unwrap().as_str().chars().next().unwrap();
            match cap1 {
                "set" => Inst::Set(reg, Val::new(cap.get(3).unwrap().as_str())),
                "sub" => Inst::Sub(reg, Val::new(cap.get(3).unwrap().as_str())),
                "mul" => Inst::Mul(reg, Val::new(cap.get(3).unwrap().as_str())),
                _ => unreachable!(),
            }
        } else if let Some(cap) = JNZ_ARG_RE.captures(s) {
            Inst::Jnz(
                Val::new(cap.get(1).unwrap().as_str()),
                Val::new(cap.get(2).unwrap().as_str()),
            )
        } else {
            panic!("Unparseable instructions: {:?}", s)
        }
    }
}

fn load_instructions() -> Vec<Inst> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        let inst = Inst::new(&line);
        ret.push(inst);
    }
    ret
}

struct VM {
    prog: Vec<Inst>,
    regs: HashMap<char, i64>,
    pc: usize,
    mulc: usize,
}

impl VM {
    fn new(input: &[Inst]) -> VM {
        let mut vm = VM {
            prog: input.to_vec(),
            regs: HashMap::new(),
            pc: 0,
            mulc: 0,
        };
        for i in 0..26 {
            vm.regs.insert((b'a' + i) as char, 0);
        }
        vm
    }

    fn eval_val(&self, val: &Val) -> i64 {
        match *val {
            Val::Reg(c) => *self.regs.get(&c).unwrap(),
            Val::Num(i) => i,
        }
    }

    fn run(&mut self) {
        while self.pc < self.prog.len() {
            let curinst = &self.prog[self.pc];
            let mut nextpc = self.pc + 1;
            match *curinst {
                Inst::Set(r, ref v_) => {
                    let v = self.eval_val(v_);
                    self.regs.insert(r, v);
                }
                Inst::Sub(r, ref v_) => {
                    let v = self.eval_val(v_);
                    let rv = *self.regs.get(&r).unwrap();
                    self.regs.insert(r, rv - v);
                }
                Inst::Mul(r, ref v_) => {
                    let v = self.eval_val(v_);
                    let rv = *self.regs.get(&r).unwrap();
                    self.regs.insert(r, rv * v);
                    self.mulc += 1;
                }
                Inst::Jnz(ref cv_, ref ov_) => {
                    let cv = self.eval_val(cv_);
                    let ov = self.eval_val(ov_);
                    if cv != 0 {
                        nextpc = ((self.pc as i64) + ov) as usize;
                    }
                }
            }
            self.pc = nextpc;
        }
    }
}

fn problem1(input: &[Inst]) -> usize {
    let mut vm = VM::new(input);
    vm.run();
    vm.mulc
}

fn problem2(input: &[Inst]) -> usize {
    // we're shortcircuiting...
    let b = match input[0] {
        Inst::Set(_, Val::Num(i)) => i,
        _ => panic!("Unexpected shape for first instruction"),
    };
    let min = 100_000 + ((b as usize) * 100);
    let max = min + 17_000;
    let sieve = Sieve::new(max);
    let mut count = 0;
    let mut val = min;
    while val <= max {
        if !sieve.is_prime(val) {
            count += 1;
        }
        val += 17;
    }
    count
}

fn main() {
    let input = load_instructions();
    println!("Loaded {} instructions", input.len());
    println!("Problem 1: {}", problem1(&input));
    println!("Problem 2: {}", problem2(&input));
}
