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
enum Instr {
    CpyI(i32, Reg),
    CpyR(Reg, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Reg, i32),
    Jmp(i32),
    Nop,
    Hlt,
}

impl Instr {
    fn new(t_: String) -> Instr {
        lazy_static! {
            static ref CPY_I_RE: Regex = Regex::new("^cpy ([0-9]+) ([abcd])$").unwrap();
            static ref CPY_R_RE: Regex = Regex::new("^cpy ([abcd]) ([abcd])$").unwrap();
            static ref INC_RE: Regex = Regex::new("^inc ([abcd])$").unwrap();
            static ref DEC_RE: Regex = Regex::new("^dec ([abcd])$").unwrap();
            static ref JNZ_R_RE: Regex = Regex::new("^jnz ([abcd]) (-?[0-9]+)$").unwrap();
            static ref JNZ_I_RE: Regex = Regex::new("^jnz (-?[0-9]+) (-?[0-9]+)$").unwrap();
        }

        fn reg_from(s: &str) -> Reg {
            match s {
                "a" => Reg::A,
                "b" => Reg::B,
                "c" => Reg::C,
                "d" => Reg::D,
                _ => unreachable!(),
            }
        }

        if CPY_I_RE.is_match(&t_) {
            for cap in CPY_I_RE.captures_iter(&t_) {
                let ref val_ = cap.get(1);
                let ref reg_ = cap.get(2);
                let reg = reg_from(reg_.unwrap().as_str());
                let val: i32 = val_.unwrap().as_str().parse().unwrap();
                return Instr::CpyI(val, reg);
            }
        } else if CPY_R_RE.is_match(&t_) {
            for cap in CPY_R_RE.captures_iter(&t_) {
                let ref regi_ = cap.get(1);
                let ref rego_ = cap.get(2);
                let regi = reg_from(regi_.unwrap().as_str());
                let rego = reg_from(rego_.unwrap().as_str());
                return Instr::CpyR(regi, rego);
            }
        } else if INC_RE.is_match(&t_) {
            for cap in INC_RE.captures_iter(&t_) {
                let ref reg_ = cap.get(1);
                let reg = reg_from(reg_.unwrap().as_str());
                return Instr::Inc(reg);
            }
        } else if DEC_RE.is_match(&t_) {
            for cap in DEC_RE.captures_iter(&t_) {
                let ref reg_ = cap.get(1);
                let reg = reg_from(reg_.unwrap().as_str());
                return Instr::Dec(reg);
            }
        } else if JNZ_R_RE.is_match(&t_) {
            for cap in JNZ_R_RE.captures_iter(&t_) {
                let ref reg_ = cap.get(1);
                let ref val_ = cap.get(2);
                let reg = reg_from(reg_.unwrap().as_str());
                let val: i32 = val_.unwrap().as_str().parse().unwrap();
                return Instr::Jnz(reg, val);
            }
        } else if JNZ_I_RE.is_match(&t_) {
            for cap in JNZ_I_RE.captures_iter(&t_) {
                let ref tst_ = cap.get(1);
                let ref val_ = cap.get(2);
                let val: i32 = val_.unwrap().as_str().parse().unwrap();
                let tst: i32 = tst_.unwrap().as_str().parse().unwrap();
                if tst != 0 {
                    return Instr::Jmp(val);
                } else {
                    return Instr::Nop;
                }
            }
        }
        panic!(t_);
    }
}

#[derive(Debug)]
struct VM {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: usize,
}

impl VM {
    fn new() -> VM {
        VM {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            pc: 0,
        }
    }

    fn get_reg(&self, reg: Reg) -> i32 {
        match reg {
            Reg::A => self.a,
            Reg::B => self.b,
            Reg::C => self.c,
            Reg::D => self.d,
        }
    }

    fn set_reg(&mut self, reg: Reg, val: i32) {
        match reg {
            Reg::A => self.a = val,
            Reg::B => self.b = val,
            Reg::C => self.c = val,
            Reg::D => self.d = val,
        }
    }

    fn exec_instr(&mut self, i: &Instr) -> bool {
        // Returns true if 'halted'
        match *i {
            Instr::CpyI(val, reg) => {
                self.set_reg(reg, val);
            }
            Instr::CpyR(regi, rego) => {
                let v = self.get_reg(regi);
                self.set_reg(rego, v);
            }
            Instr::Inc(reg) => {
                let v = self.get_reg(reg);
                self.set_reg(reg, v + 1);
            }
            Instr::Dec(reg) => {
                let v = self.get_reg(reg);
                self.set_reg(reg, v - 1);
            }
            Instr::Jnz(reg, ofs) => {
                let v = self.get_reg(reg);
                if v != 0 {
                    self.pc = ((self.pc as i32) + ofs - 1) as usize;
                }
            }
            Instr::Jmp(ofs) => {
                self.pc = ((self.pc as i32) + ofs - 1) as usize;
            }
            Instr::Nop => {}
            Instr::Hlt => {
                return true;
            }
        };
        self.pc += 1;
        false
    }

    fn run_program(&mut self, prog: Vec<Instr>) {
        loop {
            let ref i = prog[self.pc];
            if self.exec_instr(i) {
                break;
            }
            if self.pc >= prog.len() {
                panic!("Fell way off!");
            }
        }
    }
}

fn load_program() -> Vec<Instr> {
    let infile = File::open("day12.input").unwrap();
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
    let prog = load_program();
    let mut vm = VM::new();
    vm.run_program(prog);
    vm.a
}

fn problem2() -> i32 {
    let prog = load_program();
    let mut vm = VM::new();
    vm.c = 1;
    vm.run_program(prog);
    vm.a
}

fn main() {
    println!("Result 1: {}", problem1());
    println!("Result 2: {}", problem2());
}
