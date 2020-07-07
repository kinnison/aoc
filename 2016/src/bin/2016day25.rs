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
    Out(Arg),
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
            static ref OUT_RE: Regex = Regex::new("^out (-?[0-9abcd]+)$").unwrap();
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
            let ref src_ = cap.get(1);
            let ref tgt_ = cap.get(2);
            let src = arg_from(src_.unwrap().as_str());
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Cpy(src, tgt)
        } else if let Some(cap) = JNZ_RE.captures(&t_) {
            let ref tst_ = cap.get(1);
            let ref ofs_ = cap.get(2);
            let tst = arg_from(tst_.unwrap().as_str());
            let ofs = arg_from(ofs_.unwrap().as_str());
            Instr::Jnz(tst, ofs)
        } else if let Some(cap) = INC_RE.captures(&t_) {
            let ref tgt_ = cap.get(1);
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Inc(tgt)
        } else if let Some(cap) = DEC_RE.captures(&t_) {
            let ref tgt_ = cap.get(1);
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Dec(tgt)
        } else if let Some(cap) = TGL_RE.captures(&t_) {
            let ref tgt_ = cap.get(1);
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Tgl(tgt)
        } else if let Some(cap) = OUT_RE.captures(&t_) {
            let ref tgt_ = cap.get(1);
            let tgt = arg_from(tgt_.unwrap().as_str());
            Instr::Out(tgt)
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
    fn new(instr: &Vec<Instr>) -> VM {
        VM {
            instr: instr.clone(),
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
        }
    }

    fn get_arg(&self, arg: Arg) -> i32 {
        let ret = match arg {
            Arg::AReg(Reg::A) => self.a,
            Arg::AReg(Reg::B) => self.b,
            Arg::AReg(Reg::C) => self.c,
            Arg::AReg(Reg::D) => self.d,
            Arg::AVal(v) => v,
        };
        ret
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
            Instr::Out(arg) => Instr::Inc(arg),
            Instr::Jnz(a, b) => Instr::Cpy(a, b),
            Instr::Cpy(a, b) => Instr::Jnz(a, b),
            Instr::Hlt => Instr::Hlt,
        };
    }

    fn exec_instr(&mut self, i: &Instr, out: &mut Option<i32>) -> bool {
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
            Instr::Out(n) => {
                let v = self.get_arg(n);
                *out = Some(v);
            }
        };
        self.pc += 1;
        false
    }

    fn run_program(&mut self, out: &mut Vec<i32>) {
        let mut idx = 0;
        while idx < out.len() {
            let i = self.instr[self.pc];
            let mut v: Option<i32> = None;
            if self.exec_instr(&i, &mut v) {
                break;
            }
            if self.pc >= self.instr.len() {
                panic!("Fell way off!");
            }
            if let Some(n) = v {
                out[idx] = n;
                idx += 1;
            }
        }
    }
}

fn load_program() -> Vec<Instr> {
    let infile = File::open("day25.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<Instr> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Instr::new(line));
    }
    ret.push(Instr::Hlt);
    return ret;
}

fn problem1() -> i32 {
    let mut goal: Vec<i32> = vec![0; 32];
    for i in 0..goal.len() {
        goal[i] = (i & 0x01) as i32;
    }
    let mut prog = load_program();
    prog.insert(0, Instr::Cpy(Arg::AVal(0), Arg::AReg(Reg::A)));
    let mut signal: Vec<i32> = vec![0; 32];
    let mut i: i32 = 0;
    loop {
        prog[0] = Instr::Cpy(Arg::AVal(i as i32), Arg::AReg(Reg::A));
        let mut vm = VM::new(&prog);
        vm.run_program(&mut signal);
        if signal == goal {
            return i;
        }
        i += 1;
    }
}

fn main() {
    println!("Result 1: {}", problem1());
}
