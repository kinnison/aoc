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
enum Command {
    Increment,
    Decrement
}

impl Command {
    fn new (s: &str) -> Command {
        match s {
            "inc" => Command::Increment,
            "dec" => Command::Decrement,
            _ => panic!("Unable to parse command: {}", s)
        }
    }
}

#[derive(Debug,Clone,Copy)]
enum Condition {
    LE, LT, EQ, NE, GT, GE
}

impl Condition {
    fn new (s: &str) -> Condition {
        match s {
            "<=" => Condition::LE,
            "<" => Condition::LT,
            "==" => Condition::EQ,
            "!=" => Condition::NE,
            ">" => Condition::GT,
            ">=" => Condition::GE,
            _ => panic!("Unable to parse condition: {}", s)
        }
    }
}

#[derive(Debug,Clone)]
struct Instruction {
    reg: String,
    cmd: Command,
    amt: i32,
    test_reg: String,
    test_cond: Condition,
    test_amt: i32
}

impl Instruction {
    fn new (input: &str) -> Instruction {
        lazy_static! {
            static ref INSTR_RE: Regex = Regex::new(r"^([^ ]+) (inc|dec) (-?[0-9]+) if ([^ ]+) ([<=!>]+) (-?[0-9]+)$").unwrap();
        }
        if let Some(cap) = INSTR_RE.captures(input) {
            let reg = cap.get(1).unwrap().as_str().to_owned();
            let cmd = Command::new(cap.get(2).unwrap().as_str());
            let amt: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
            let test_reg = cap.get(4).unwrap().as_str().to_owned();
            let test_cond = Condition::new(cap.get(5).unwrap().as_str());
            let test_amt: i32 = cap.get(6).unwrap().as_str().parse().unwrap();
            return Instruction {
                reg: reg,
                cmd: cmd,
                amt: amt,
                test_reg: test_reg,
                test_cond: test_cond,
                test_amt: test_amt,
            }
        }
        panic!("Unable to parse instruction: {:?}", input);
    }
}

fn load_instructions () -> Vec<Instruction> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Instruction::new(&line));
    }
    ret
}

struct VM {
    regs: HashMap<String, i32>,
    biggest: i32
}

impl VM {
    fn new () -> VM {
        VM { regs: HashMap::new(), biggest: 0 }
    }

    fn run_instruction (&mut self, instr: &Instruction) {
        let testval = self.regs.get(&instr.test_reg).map(|v| *v).unwrap_or(0);
        if match instr.test_cond {
            Condition::LE => testval <= instr.test_amt,
            Condition::LT => testval < instr.test_amt,
            Condition::EQ => testval == instr.test_amt,
            Condition::NE => testval != instr.test_amt,
            Condition::GT => testval > instr.test_amt,
            Condition::GE => testval >= instr.test_amt
        } {
            // Test passed, execute change...
            let curval = self.regs.get(&instr.reg).map(|v| *v).unwrap_or(0);
            let newval = match instr.cmd {
                Command::Increment => curval + instr.amt,
                Command::Decrement => curval - instr.amt
            };
            self.regs.insert(instr.reg.clone(), newval);
            if newval > self.biggest {
                self.biggest = newval;
            }
        }
    }

    fn run_program (&mut self, prog: &Vec<Instruction>) {
        for instr in prog {
            self.run_instruction(instr);
        }
    }
}

fn problem1 (prog: &Vec<Instruction>) -> i32 {
    let mut vm = VM::new();
    vm.run_program(prog);
    let mut biggest = 0;
    for (_, &value) in &vm.regs {
        if value > biggest {
            biggest = value;
        }
    }
    biggest
}

fn problem2 (prog: &Vec<Instruction>) -> i32 {
    let mut vm = VM::new();
    vm.run_program(prog);
    vm.biggest
}

fn main() {
    let input = load_instructions();
    println!("Loaded {} instructions", input.len());
    println!("Problem 1: {}", problem1(&input));
    println!("Problem 2: {}", problem2(&input));
}
