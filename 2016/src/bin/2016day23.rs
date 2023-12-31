#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Reg {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Copy)]
enum Arg {
    AReg(Reg),
    AVal(i32),
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Inc(Arg),
    Dec(Arg),
    Tgl(Arg),
    Cpy(Arg, Arg),
    Jnz(Arg, Arg),
    Hlt,
}

impl Instr {
    fn new(t_: String) -> Instr {
        lazy_static! {
            static ref INC_RE: Regex = Regex::new("^inc ([abcd])$").unwrap();
            static ref DEC_RE: Regex = Regex::new("^dec ([abcd])$").unwrap();
            static ref CPY_RE: Regex = Regex::new("^cpy (-?[0-9abcd]+) (-?[0-9abcd]+)$").unwrap();
            static ref JNZ_RE: Regex = Regex::new("^jnz (-?[0-9abcd]+) (-?[0-9abcd]+)$").unwrap();
            static ref TGL_RE: Regex = Regex::new("^tgl (-?[0-9abcd]+)$").unwrap();
        }

        fn arg_from(s: &str) -> Arg {
            match s {
                "a" => Arg::AReg(Reg::A),
                "b" => Arg::AReg(Reg::B),
                "c" => Arg::AReg(Reg::C),
                "d" => Arg::AReg(Reg::D),
                _ => {
                    let v: i32 = s.parse().unwrap();
                    Arg::AVal(v)
                }
            }
        }

        if let Some(cap) = CPY_RE.captures(&t_) {
            let src_ = cap.get(1);
            let tgt_ = cap.get(2);
            let src = arg_from(src_.unwrap().as_str());
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Cpy(src, tgt)
        } else if let Some(cap) = JNZ_RE.captures(&t_) {
            let tst_ = cap.get(1);
            let ofs_ = cap.get(2);
            let tst = arg_from(tst_.unwrap().as_str());
            let ofs = arg_from(ofs_.unwrap().as_str());
            Instr::Jnz(tst, ofs)
        } else if let Some(cap) = INC_RE.captures(&t_) {
            let tgt_ = cap.get(1);
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Inc(tgt)
        } else if let Some(cap) = DEC_RE.captures(&t_) {
            let tgt_ = cap.get(1);
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Dec(tgt)
        } else if let Some(cap) = TGL_RE.captures(&t_) {
            let tgt_ = cap.get(1);
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Tgl(tgt)
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug)]
struct VM {
    instr: Vec<Instr>,
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: usize,
}

impl VM {
    fn new(instr: Vec<Instr>) -> VM {
        VM {
            instr,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
        }
    }

    fn get_arg(&self, arg: Arg) -> i32 {
        match arg {
            Arg::AReg(Reg::A) => self.a,
            Arg::AReg(Reg::B) => self.b,
            Arg::AReg(Reg::C) => self.c,
            Arg::AReg(Reg::D) => self.d,
            Arg::AVal(v) => v,
        }
    }

    fn set_arg(&mut self, arg: Arg, val: i32) {
        match arg {
            Arg::AReg(Reg::A) => self.a = val,
            Arg::AReg(Reg::B) => self.b = val,
            Arg::AReg(Reg::C) => self.c = val,
            Arg::AReg(Reg::D) => self.d = val,
            Arg::AVal(_) => {}
        }
    }

    fn toggle(&mut self, tgtinstr: i32) {
        if (tgtinstr < 0) || ((tgtinstr as usize) >= self.instr.len()) {
            return;
        }
        let i: Instr = self.instr[tgtinstr as usize];
        self.instr[tgtinstr as usize] = match i {
            Instr::Inc(arg) => Instr::Dec(arg),
            Instr::Dec(arg) => Instr::Inc(arg),
            Instr::Tgl(arg) => Instr::Inc(arg),
            Instr::Jnz(a, b) => Instr::Cpy(a, b),
            Instr::Cpy(a, b) => Instr::Jnz(a, b),
            Instr::Hlt => Instr::Hlt,
        };
    }

    fn exec_instr(&mut self, i: &Instr) -> bool {
        // Returns true if 'halted'
        match *i {
            Instr::Cpy(from, to) => {
                let v = self.get_arg(from);
                self.set_arg(to, v);
            }
            Instr::Inc(arg) => {
                let v = self.get_arg(arg);
                self.set_arg(arg, v + 1);
            }
            Instr::Dec(arg) => {
                let v = self.get_arg(arg);
                self.set_arg(arg, v - 1);
            }
            Instr::Jnz(tst, ofs_) => {
                let v = self.get_arg(tst);
                let ofs = self.get_arg(ofs_);
                if v != 0 {
                    self.pc = ((self.pc as i32) + ofs - 1) as usize;
                }
            }
            Instr::Tgl(tgt_) => {
                let tgt = self.get_arg(tgt_);
                let tgtinstr = (self.pc as i32) + tgt;
                self.toggle(tgtinstr);
            }
            Instr::Hlt => {
                return true;
            }
        };
        self.pc += 1;
        false
    }

    fn run_program(&mut self) {
        loop {
            let i = self.instr[self.pc];
            if self.exec_instr(&i) {
                break;
            }
            if self.pc >= self.instr.len() {
                panic!("Fell way off!");
            }
        }
    }
}

fn load_program() -> Vec<Instr> {
    let infile = File::open("day23.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<Instr> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Instr::new(line));
    }
    ret.push(Instr::Hlt);
    ret
}

fn problem1() -> i32 {
    let mut prog = load_program();
    prog.insert(0, Instr::Cpy(Arg::AVal(7), Arg::AReg(Reg::A)));
    let mut vm = VM::new(prog);
    vm.run_program();
    vm.a
}

fn problem2() -> i32 {
    let mut prog = load_program();
    prog.insert(0, Instr::Cpy(Arg::AVal(12), Arg::AReg(Reg::A)));
    let mut vm = VM::new(prog);
    vm.run_program();
    vm.a
}

fn test() -> i32 {
    let mut prog: Vec<Instr> = Vec::new();
    prog.push(Instr::new("cpy 2 a".into()));
    prog.push(Instr::new("tgl a".into()));
    prog.push(Instr::new("tgl a".into()));
    prog.push(Instr::new("tgl a".into()));
    prog.push(Instr::new("cpy 1 a".into()));
    prog.push(Instr::new("dec a".into()));
    prog.push(Instr::new("dec a".into()));
    prog.push(Instr::Hlt);
    let mut vm = VM::new(prog);
    vm.run_program();
    vm.a
}

fn main() {
    println!("Test: {} (should be 3)", test());
    println!("Result 1: {}", problem1());
    println!("Result 2: {}", problem2());
}
