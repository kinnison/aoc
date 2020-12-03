use aoc2015::*;

#[derive(Debug)]
enum Reg {
    A,
    B,
}

#[derive(Debug)]
enum Instr {
    Hlf { reg: Reg },
    Tpl { reg: Reg },
    Inc { reg: Reg },
    Jmp { ofs: i32 },
    Jie { reg: Reg, ofs: i32 },
    Jio { reg: Reg, ofs: i32 },
}

impl Reg {
    fn from_str(reg: &str) -> Reg {
        match reg {
            "a" => Reg::A,
            "b" => Reg::B,
            _ => panic!("Unknown register: {}", reg),
        }
    }
}

impl Instr {
    fn from_str(input: &str) -> Instr {
        lazy_static! {
            static ref HLF: Regex = Regex::new("^hlf ([ab])$").unwrap();
            static ref TPL: Regex = Regex::new("^tpl ([ab])$").unwrap();
            static ref INC: Regex = Regex::new("^inc ([ab])$").unwrap();
            static ref JMP: Regex = Regex::new("^jmp ([+-][0-9]+)$").unwrap();
            static ref JIE: Regex = Regex::new("^jie ([ab]), ([+-][0-9]+)$").unwrap();
            static ref JIO: Regex = Regex::new("^jio ([ab]), ([+-][0-9]+)$").unwrap();
        }
        if let Some(cap) = HLF.captures(input) {
            Instr::Hlf {
                reg: Reg::from_str(cap.get(1).unwrap().as_str()),
            }
        } else if let Some(cap) = TPL.captures(input) {
            Instr::Tpl {
                reg: Reg::from_str(cap.get(1).unwrap().as_str()),
            }
        } else if let Some(cap) = INC.captures(input) {
            Instr::Inc {
                reg: Reg::from_str(cap.get(1).unwrap().as_str()),
            }
        } else if let Some(cap) = JMP.captures(input) {
            Instr::Jmp {
                ofs: cap.get(1).unwrap().as_str().parse().unwrap(),
            }
        } else if let Some(cap) = JIE.captures(input) {
            Instr::Jie {
                reg: Reg::from_str(cap.get(1).unwrap().as_str()),
                ofs: cap.get(2).unwrap().as_str().parse().unwrap(),
            }
        } else if let Some(cap) = JIO.captures(input) {
            Instr::Jio {
                reg: Reg::from_str(cap.get(1).unwrap().as_str()),
                ofs: cap.get(2).unwrap().as_str().parse().unwrap(),
            }
        } else {
            panic!("Unable to parse instruction: {}", input)
        }
    }
}

struct VM {
    reg_a: usize,
    reg_b: usize,
    pc: i32,
}

impl VM {
    fn new() -> VM {
        VM {
            reg_a: 0,
            reg_b: 0,
            pc: 0,
        }
    }

    fn run_instruction(&mut self, instr: &Instr) {
        match instr {
            Instr::Hlf { ref reg } => match reg {
                Reg::A => self.reg_a >>= 1,
                Reg::B => self.reg_b >>= 1,
            },
            Instr::Tpl { ref reg } => match reg {
                Reg::A => self.reg_a *= 3,
                Reg::B => self.reg_b *= 3,
            },
            Instr::Inc { ref reg } => match reg {
                Reg::A => self.reg_a += 1,
                Reg::B => self.reg_b += 1,
            },
            Instr::Jmp { ofs } => self.pc += ofs - 1,
            Instr::Jie { ref reg, ofs } => {
                if (match reg {
                    Reg::A => self.reg_a,
                    Reg::B => self.reg_b,
                } & 1)
                    == 0
                {
                    self.pc += ofs - 1;
                }
            }
            Instr::Jio { ref reg, ofs } => {
                if (match reg {
                    Reg::A => self.reg_a,
                    Reg::B => self.reg_b,
                }) == 1
                {
                    self.pc += ofs - 1;
                }
            }
        };
        self.pc += 1;
    }

    fn run_program(&mut self, prog: &[Instr]) {
        while self.pc >= 0 && self.pc < (prog.len() as i32) {
            self.run_instruction(&prog[self.pc as usize]);
        }
    }
}

fn part1(input: &[Instr]) -> usize {
    let mut vm = VM::new();
    vm.run_program(input);
    vm.reg_b
}

fn part2(input: &[Instr]) -> usize {
    let mut vm = VM::new();
    vm.reg_a = 1;
    vm.run_program(input);
    vm.reg_b
}

fn main() -> Result<()> {
    let input: Vec<Instr> = read_input(23)?.lines().map(Instr::from_str).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
