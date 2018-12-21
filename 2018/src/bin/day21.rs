#![allow(non_snake_case)]
#![allow(clippy::trivial_regex)]
use aoc2018::*;

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Copy, Clone, Hash, ParseByRegex)]
enum Op {
    #[regex = "^addr$"]
    AddR,
    #[regex = "^addi$"]
    AddI,
    #[regex = "^mulr$"]
    MulR,
    #[regex = "^muli$"]
    MulI,
    #[regex = "^banr$"]
    BanR,
    #[regex = "^bani$"]
    BanI,
    #[regex = "^borr$"]
    BorR,
    #[regex = "^bori$"]
    BorI,
    #[regex = "^setr$"]
    SetR,
    #[regex = "^seti$"]
    SetI,
    #[regex = "^gtir$"]
    GtIR,
    #[regex = "^gtri$"]
    GtRI,
    #[regex = "^gtrr$"]
    GtRR,
    #[regex = "^eqir$"]
    EqIR,
    #[regex = "^eqri$"]
    EqRI,
    #[regex = "^eqrr$"]
    EqRR,
}

use self::Op::*;

#[derive(Copy, Clone, ParseByRegex)]
#[regex = r"^(?P<op>[a-z]+) (?P<A>-?\d+) (?P<B>-?\d+) (?P<C>-?\d+)"]
struct Instr {
    op: Op,
    A: i32,
    B: i32,
    C: i32,
}

#[derive(Clone)]
struct VM {
    regs: [i32; 6],
    pc: usize,
    ip: i32,
    prog: Vec<Instr>,
    opcount: usize,
}

impl VM {
    fn from_pair<S: AsRef<str>>(prog: Vec<Instr>, ipline: S) -> Result<VM> {
        let ip = &ipline.as_ref()[4..];
        let ip: usize = ip.parse()?;
        Ok(VM::new(ip, prog))
    }

    fn new(pc: usize, prog: Vec<Instr>) -> VM {
        VM {
            regs: [0; 6],
            pc,
            prog,
            ip: 0,
            opcount: 0,
        }
    }

    fn exec_(&mut self, instr: Op, A: i32, B: i32, C: i32) -> Result<()> {
        if C < 0 || C > 5 {
            Err(format!("C ({}) is out of range", C))?
        }
        let rA = if A >= 0 && A <= 5 {
            Ok(self.regs[A as usize])
        } else {
            Err(format!("A ({}) is out of range", A))
        };
        let rB = if B >= 0 && B <= 5 {
            Ok(self.regs[B as usize])
        } else {
            Err(format!("B ({}) is out of range", B))
        };
        self.regs[C as usize] = match instr {
            AddR => rA?.wrapping_add(rB?),
            AddI => rA?.wrapping_add(B),
            MulR => rA?.wrapping_mul(rB?),
            MulI => rA?.wrapping_mul(B),
            BanR => rA? & rB?,
            BanI => rA? & B,
            BorR => rA? | rB?,
            BorI => rA? | B,
            SetR => rA?,
            SetI => A,
            GtIR => {
                if A > rB? {
                    1
                } else {
                    0
                }
            }
            GtRI => {
                if rA? > B {
                    1
                } else {
                    0
                }
            }
            GtRR => {
                if rA? > rB? {
                    1
                } else {
                    0
                }
            }
            EqIR => {
                if A == rB? {
                    1
                } else {
                    0
                }
            }
            EqRI => {
                if rA? == B {
                    1
                } else {
                    0
                }
            }
            EqRR => {
                if rA? == rB? {
                    1
                } else {
                    0
                }
            }
        };
        Ok(())
    }

    fn run(&mut self, stopatpc: Option<i32>, stopatop: Option<usize>) -> Result<()> {
        while (self.ip >= 0) && ((self.ip as usize) < self.prog.len()) {
            let instr = self.prog[self.ip as usize];
            if cfg!(debug_assertions) {
                print!(
                    "ip={} {:?} {:?} {} {} {} ",
                    self.ip, self.regs, instr.op, instr.A, instr.B, instr.C
                );
            }
            // Exec sequence is always "put IP into reg[PC]" which is semi-side-effect
            self.regs[self.pc] = self.ip;
            // then it's run instruction...
            self.exec_(instr.op, instr.A, instr.B, instr.C)?;
            self.opcount += 1;
            // Then copy back into IP
            self.ip = self.regs[self.pc];
            // And now it's increment the IP
            self.ip += 1;
            if cfg!(debug_assertions) {
                println!("{:?}", self.regs);
            }
            if let Some(stopat) = stopatpc {
                if self.ip == stopat {
                    if cfg!(debug_assertions) {
                        println!("Stopping because ip={}", self.ip);
                    }
                    break;
                }
            }
            if let Some(stopat) = stopatop {
                if self.opcount == stopat {
                    if cfg!(debug_assertions) {
                        println!("Stopping because opcount={}", self.opcount);
                    }
                    break;
                }
            }
        }
        Ok(())
    }
}

fn part1(input: &VM) -> Result<i32> {
    let mut vm = input.clone();

    vm.run(Some(28), None)?;
    // At this point, we're looking to make new r0 be old r3
    let r3 = vm.regs[3];
    let mut vm = input.clone();
    vm.regs[0] = r3;
    vm.run(None, None)?;

    Ok(vm.regs[0])
}

fn part2(input: &VM) -> Result<i32> {
    let mut vm = input.clone();

    let mut seenops: HashSet<i32> = HashSet::new();
    let mut prev = 0;
    loop {
        vm.run(Some(28), None)?;
        // At this point, we know the minimum r0 which will halt in the fewest
        // instructions
        let r3 = vm.regs[3];
        // Our goal is to achieve the highest op count while still halting
        // which means we need to work out if there is some value of r3 at this
        // point which happens before a loop
        //println!("r3 = 0x{:08x}, delta from previous loop: {} {}", r3, if (r3 - prev) < 0 { "NEG" } else {"POS"},r3 - prev);
        if !seenops.insert(r3) {
            break;
        }
        prev = r3;
    }
    // if r0 were this value we'd have looped, so use the previous

    Ok(prev)
}

fn main() -> Result<()> {
    let (prog, ipline): (Vec<Instr>, String) = read_input_as_vec_and_first(21)?;
    let input = VM::from_pair(prog, ipline)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);

    Ok(())
}

/*
#ip 1

 0: seti 123 0 3        r3 = 123
 1: bani 3 456 3        r3 = r3 & 456
 2: eqri 3 72 3         r3 = (r3 == 72)
 3: addr 3 1 1          ifeq, goto 5
 4: seti 0 0 1          else, goto 1
 5: seti 0 9 3          r3 = 0
 6: bori 3 65536 5      r5 = r3 | 65536  (0x1_0000)
 7: seti 15028787 4 3   r3 = 15028787
 8: bani 5 255 2        r2 = r5 & 255
 9: addr 3 2 3          r3 = r3 + r2
10: bani 3 16777215 3   r3 = r3 & 0xFF_FFFF
11: muli 3 65899 3      r3 = r3 * 65899 (0x1_016B)
12: bani 3 16777215 3   r3 = r3 & 0xFF_FFFF
13: gtir 256 5 2        r2 = 256 > r5
14: addr 2 1 1          ifso, goto 16
15: addi 1 1 1          else, goto 17
16: seti 27 3 1         goto 28
17: seti 0 9 2          r2 = 0
18: addi 2 1 4          r4 = r2 + 1
19: muli 4 256 4        r4 = r4 * 256  (<<8)
20: gtrr 4 5 4          r4 = r4 > r5
21: addr 4 1 1          ifso, goto 23
22: addi 1 1 1          else goto 24
23: seti 25 1 1         goto 26
24: addi 2 1 2          r2 = r2 + 2
25: seti 17 8 1         goto 18
26: setr 2 4 5          r5 = r2 + r4
27: seti 7 3 1          goto 8
28: eqrr 3 0 2          r2 = r3 == r0
29: addr 2 1 1          ifeq, goto 31  (Halt)
30: seti 5 3 1          goto 6

 0: while (123 & 456) != 72 {}
 5: r3 = 0;
 6: do {
    r5 = r3 | 65536;
    r3 = 0xE5_5233;
 8: r2 = r5 & 0xFF
    r3 = r3 + r2;
    r3 = r3 & 0xFF_FFFF;
    r3 = r3 * 0x1_016B;
    if r5 > 256 {
        r2 = 0
18:     r4 = r2 + 1
        r4 = r4 * 256
        if r4 > r5 {
            r5 = r2 + r4
            goto 8
        } else {
            r2 = r2 + 2
            goto 18
        }
    }
28: } while r3 != r0

 0: while (123 & 456) != 72 {}
 5: r3 = 0;
 6: do {
        r5 = r3 | 65536;
        r3 = 0xE5_5233;
 8:     r2 = r5 & 0xFF
        r3 = r3 + r2;
        r3 = r3 & 0xFF_FFFF;
        r3 = r3 * 0x1_016B;
        if r5 > 256 {
            r4 = 0;  // Safety for the for loop condition
18:         for(r2 = 0; r5 <= r4; r2 += 2) {
                r4 = r2 + 1
                r4 = r4 * 256
            }
            r5 = r2 + r4;
            goto 8
        }
    // This is the only use of r0, so our goal is to make *a* r5 not be > 256
    // and *b* make r0 set to whatever r3 would be at that point.
28: } while r3 != r0

// One final refactor to remove a goto...

 0: while (123 & 456) != 72 {}
 5: r3 = 0;
 6: do {
        r5 = r3 | 65536;
        r3 = 0xE5_5233;
 8:     loop {
            r2 = r5 & 0xFF
            r3 = r3 + r2;
            r3 = r3 & 0xFF_FFFF;
            r3 = r3 * 0x1_016B;
            if r5 <= 256 { break }
            r4 = 0;  // Safety for the for loop condition
            r2 = 0;
18:         while(r5 <= r4) {
                r4 = r2 + 1
                r4 = r4 * 256
                r2 += 2
            }
            r5 = r2 + r4;
        }
    // This is the only use of r0, so our goal is to make *a* r5 not be > 256
    // and *b* make r0 set to whatever r3 would be at that point.
28: } while r3 != r0

*/
