#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug,Clone,Copy)]
enum Val {
    Reg(char),
    Num(i64)
}

impl Val {
    fn new (s: &str) -> Val {
        let fc = s.chars().next().unwrap();
        if fc >= 'a' && fc <= 'z' {
            Val::Reg(fc)
        } else {
            Val::Num(s.parse().unwrap())
        }
    }
}

#[derive(Debug,Clone,Copy)]
enum Inst {
    Snd(Val),
    Set(char, Val),
    Add(char, Val),
    Mul(char, Val),
    Mod(char, Val),
    Rcv(Val),
    Jgz(Val, Val)
}

impl Inst {
    fn new (s: &str) -> Inst {
        lazy_static! {
            static ref ONE_ARG_RE: Regex = Regex::new(r"^(snd|rcv) (-?[a-z0-9]+)$").unwrap();
            static ref TWO_ARG_RE: Regex = Regex::new(r"^(set|add|mul|mod) ([a-z]) (-?[a-z0-9]+)$").unwrap();
            static ref JGZ_ARG_RE: Regex = Regex::new(r"^jgz (-?[a-z0-9]+) (-?[a-z0-9]+)$").unwrap();
        }
        if let Some(cap) = ONE_ARG_RE.captures(s) {
            let cap1 = cap.get(1).unwrap().as_str();
            match cap1 {
                "snd" => Inst::Snd(Val::new(cap.get(2).unwrap().as_str())),
                "rcv" => Inst::Rcv(Val::new(cap.get(2).unwrap().as_str())),
                _     => unreachable!()
            }
        } else if let Some(cap) = TWO_ARG_RE.captures(s) {
            let cap1 = cap.get(1).unwrap().as_str();
            let reg = cap.get(2).unwrap().as_str().chars().next().unwrap();
            match cap1 {
                "set" => Inst::Set(reg, Val::new(cap.get(3).unwrap().as_str())),
                "add" => Inst::Add(reg, Val::new(cap.get(3).unwrap().as_str())),
                "mul" => Inst::Mul(reg, Val::new(cap.get(3).unwrap().as_str())),
                "mod" => Inst::Mod(reg, Val::new(cap.get(3).unwrap().as_str())),
                _     => unreachable!()
            }
        } else if let Some(cap) = JGZ_ARG_RE.captures(s) {
            Inst::Jgz(Val::new(cap.get(1).unwrap().as_str()),
                      Val::new(cap.get(2).unwrap().as_str()))
        } else {
            panic!("Unparseable instructions: {:?}", s)
        }
    }
}

fn load_instructions () -> Vec<Inst> {
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
    regs: HashMap<char, i64>,
    inst: Vec<Inst>,
    pc: usize,
    lastsnd: i64,
    lastrcv: i64,
    sent: usize
}

impl VM {
    fn new (input: &Vec<Inst>) -> VM {
        let mut vm = VM {
            regs: HashMap::new(),
            inst: input.clone(),
            pc: 0,
            lastsnd: 0,
            lastrcv: 0,
            sent: 0
        };
        for i in 0..26 {
            vm.regs.insert(((('a' as u8) + i) as char), 0);
        }
        vm
    }

    fn eval_val (&self, val: &Val) -> i64 {
        match val {
            &Val::Reg(c) => self.regs.get(&c).unwrap().clone(),
            &Val::Num(i) => i
        }
    }
    
    fn run (&mut self, stop_on_rcv: bool) {
        while self.pc < self.inst.len() {
            let inst: &Inst = &self.inst[self.pc];
            let mut nextpc = self.pc + 1;
            match inst {
                &Inst::Snd(ref v_) => {
                    let v = self.eval_val(v_);
                    self.lastsnd = v;
                },
                &Inst::Set(r, ref v_) => {
                    let v = self.eval_val(v_);
                    self.regs.insert(r, v);
                },
                &Inst::Add(r, ref v_) => {
                    let v = self.eval_val(v_);
                    let rv = self.regs.get(&r).unwrap().clone();
                    self.regs.insert(r, rv + v);
                },
                &Inst::Mul(r, ref v_) => {
                    let v = self.eval_val(v_);
                    let rv = self.regs.get(&r).unwrap().clone();
                    self.regs.insert(r, rv * v);
                },
                &Inst::Mod(r, ref v_) => {
                    let v = self.eval_val(v_);
                    let rv = self.regs.get(&r).unwrap().clone();
                    self.regs.insert(r, rv % v);
                },
                &Inst::Rcv(ref v_) => {
                    let v = self.eval_val(v_);
                    if v != 0 {
                        self.lastrcv = self.lastsnd;
                        if stop_on_rcv {
                            break;
                        }
                    }
                },
                &Inst::Jgz(ref cv_, ref ov_) => {
                    let cv = self.eval_val(cv_);
                    let ov = self.eval_val(ov_);
                    if cv > 0 {
                        nextpc = ((self.pc as i64) + ov) as usize;
                    }
                }
            }
            self.pc = nextpc;
        }
    }

    fn run2 (&mut self, inc: &mut Vec<i64>, out: &mut Vec<i64>) -> bool {
        while self.pc < self.inst.len() {
            let inst: &Inst = &self.inst[self.pc];
            let mut nextpc = self.pc + 1;
            match inst {
                &Inst::Snd(ref v_) => {
                    let v = self.eval_val(v_);
                    out.push(v);
                    self.sent += 1;
                },
                &Inst::Set(r, ref v_) => {
                    let v = self.eval_val(v_);
                    self.regs.insert(r, v);
                },
                &Inst::Add(r, ref v_) => {
                    let v = self.eval_val(v_);
                    let rv = self.regs.get(&r).unwrap().clone();
                    self.regs.insert(r, rv + v);
                },
                &Inst::Mul(r, ref v_) => {
                    let v = self.eval_val(v_);
                    let rv = self.regs.get(&r).unwrap().clone();
                    self.regs.insert(r, rv * v);
                },
                &Inst::Mod(r, ref v_) => {
                    let v = self.eval_val(v_);
                    let rv = self.regs.get(&r).unwrap().clone();
                    self.regs.insert(r, rv % v);
                },
                &Inst::Rcv(ref v_) => {
                    let r = match v_ {
                        &Val::Reg(rn) => rn,
                        _ => panic!("Attempted to rcv to non-reg")
                    };
                    if inc.len() == 0 {
                        return false;
                    }
                    let v = inc.remove(0);
                    self.regs.insert(r, v);
                },
                &Inst::Jgz(ref cv_, ref ov_) => {
                    let cv = self.eval_val(cv_);
                    let ov = self.eval_val(ov_);
                    if cv > 0 {
                        nextpc = ((self.pc as i64) + ov) as usize;
                    }
                }
            }
            self.pc = nextpc;
        }
        return true;
    }
}

fn problem1 (input: &Vec<Inst>) -> i64 {
    let mut vm = VM::new(input);
    vm.run(true);
    vm.lastrcv
}

fn problem2 (input: &Vec<Inst>) -> usize {
    let mut vm0 = VM::new(input);
    let mut vm1 = VM::new(input);
    vm1.regs.insert('p', 1);
    let mut finished0 = false;
    let mut finished1 = false;
    let mut from0to1 = Vec::new();
    let mut from1to0 = Vec::new();
    while !(finished0 || finished1) {
        if !finished0 {
            if vm0.run2(&mut from0to1, &mut from1to0) {
                finished0 = true;
            } else {
                if from0to1.len() == 0 && from1to0.len() == 0 {
                    finished0 = true;
                }
            }
        }
        if !finished1 {
            if vm1.run2(&mut from1to0, &mut from0to1) {
                finished1 = true;
            } else {
                if from0to1.len() == 0 && from1to0.len() == 0 {
                    finished1 = true;
                }
            }
        }
    }
    vm1.sent
}

fn main() {
    let input = load_instructions();
    println!("Loaded {} instructions", input.len());
    println!("Problem 1: {}", problem1(&input));
    println!("Problem 2: {}", problem2(&input));
}
