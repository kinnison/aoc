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
            AddR => rA? + rB?,
            AddI => rA? + B,
            MulR => rA? * rB?,
            MulI => rA? * B,
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

    fn run(&mut self, stopat: Option<i32>) -> Result<()> {
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
            // Then copy back into IP
            self.ip = self.regs[self.pc];
            // And now it's increment the IP
            self.ip += 1;
            if cfg!(debug_assertions) {
                println!("{:?}", self.regs);
            }
            if let Some(stopat) = stopat {
                if self.ip == stopat {
                    if cfg!(debug_assertions) {
                        println!("Stopping because ip={}", self.ip);
                    }
                    break;
                }
            }
        }
        Ok(())
    }
}

static TEST_INPUT: &str = r"
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";

fn part1(input: &VM) -> Result<i32> {
    let mut vm = input.clone();
    vm.run(None)?;
    Ok(vm.regs[0])
}

fn part2(input: &VM) -> Result<usize> {
    let mut vm = input.clone();
    vm.regs[0] = 1;
    // For explanation of why we stop at 1, and then calculate from there
    // see comment below main
    vm.run(Some(1))?;
    // Next calculate the target register number (see below as well)
    let target_reg = 'target: loop {
        for i in 1..vm.prog.len() {
            let instr = vm.prog[i];
            if instr.op == EqRR {
                let ignore = vm.prog[i - 1].C;
                break 'target if instr.A == ignore { instr.B } else { instr.A };
            }
        }
    };
    // Finally calculate the factor sum
    let target = vm.regs[target_reg as usize] as usize;
    let mut factorsum: usize = 0;
    for i in 1..=target {
        if (target % i) == 0 {
            factorsum += i;
        }
    }
    Ok(factorsum)
}

fn main() -> Result<()> {
    let (test_prog, test_ipline): (Vec<Instr>, String) = input_as_vec_and_first(TEST_INPUT)?;
    let test_input = VM::from_pair(test_prog, test_ipline)?;

    println!("Test 1: {}", part1(&test_input)?);

    let (prog, ipline): (Vec<Instr>, String) = read_input_as_vec_and_first(19)?;
    let input = VM::from_pair(prog, ipline)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);

    Ok(())
}

/* Part 2 is complex

#ip 2

 0: addi 2 16 2  (since 2 is IP, this is absolute jump to 17 (16+inc))
 1: seti 1 1 5  r5 = 1
 2: seti 1 1 3  r3 = 1
 3: mulr 5 3 4  r4 = r5 * r3
 4: eqrr 4 1 4  r4 = (r4 == r1)
 5: addr 4 2 2  Goto 7 if r4 was equal to r1
 6: addi 2 1 2  Goto 8
 7: addr 5 0 0  r0 = r0 + r5
 8: addi 3 1 3  r3 = r3 + 1
 9: gtrr 3 1 4  r4 = (r3 > r1)
10: addr 2 4 2  Goto 12 if r3 was greater than r1
11: seti 2 8 2  Goto 3
12: addi 5 1 5  r5 = r5 + 1
13: gtrr 5 1 4  r4 = r5 > r1
14: addr 4 2 2  goto 16 if r5 > r1
15: seti 1 5 2  goto 2
16: mulr 2 2 2  r2 = r2 * r2   (goto 256, off the end)

This is a setup routine.

17: addi 1 2 1  r2 += 2        (r2 is forming a 0 2 4 6 sequence)
18: mulr 1 1 1  r1 = r1 * r1
19: mulr 2 1 1  r1 = r2 * r1
20: muli 1 11 1 r1 = r1 * 11   (r1 = 11 * r2 * r1 * r1
21: addi 4 3 4  r4 = r4 + 3
22: mulr 4 2 4  r4 = r4 * r2
23: addi 4 7 4  r4 = r4 + 7
24: addr 1 4 1  r1 = r1 + r4   (r1 += 7 + (r2 * (r4 + 3)))
25: addr 2 0 2  with r0=1 this skips the jump below
26: seti 0 4 2  Jump to 1

This is extra setup for r0=1

27: setr 2 8 4  r4 = r2
28: mulr 4 2 4  r2 = r4 * r2
29: addr 2 4 4  r4 = r2 + r4
30: mulr 2 4 4  r4 = r2 * r4
31: muli 4 14 4 r4 = r4 * 14
32: mulr 4 2 4  r4 = r4 * r2
33: addr 1 4 1  r1 = r1 + r4
34: seti 0 5 0  r0 = 0
35: seti 0 8 2  Jump to 1

The setup doesn't loop which means we can probably determine its calculation

Part 1 enters the upper routine with:

ip=1 [0, 909, 0, 0, 73, 0] SetI 1 1 5 [0, 909, 1, 0, 73, 1]
ip=2 [0, 909, 1, 0, 73, 1] SetI 1 1 3 [0, 909, 2, 1, 73, 1]
ip=3 [0, 909, 2, 1, 73, 1] MulR 5 3 4 [0, 909, 3, 1, 1, 1]
ip=4 [0, 909, 3, 1, 1, 1] EqRR 4 1 4 [0, 909, 4, 1, 0, 1]
ip=5 [0, 909, 4, 1, 0, 1] AddR 4 2 2 [0, 909, 5, 1, 0, 1]
ip=6 [0, 909, 5, 1, 0, 1] AddI 2 1 2 [0, 909, 7, 1, 0, 1]
ip=8 [0, 909, 7, 1, 0, 1] AddI 3 1 3 [0, 909, 8, 2, 0, 1]
ip=9 [0, 909, 8, 2, 0, 1] GtRR 3 1 4 [0, 909, 9, 2, 0, 1]
ip=10 [0, 909, 9, 2, 0, 1] AddR 2 4 2 [0, 909, 10, 2, 0, 1]
ip=11 [0, 909, 10, 2, 0, 1] SetI 2 8 2 [0, 909, 2, 2, 0, 1]

r1 = 909 everything else appears redundant

Part 1 calculates r0=1326 at the end

Part 2 enters with:

ip=1 [0, 10551309, 0, 0, 10550400, 0] SetI 1 1 5 [0, 10551309, 1, 0, 10550400, 1]
ip=2 [0, 10551309, 1, 0, 10550400, 1] SetI 1 1 3 [0, 10551309, 2, 1, 10550400, 1]
ip=3 [0, 10551309, 2, 1, 10550400, 1] MulR 5 3 4 [0, 10551309, 3, 1, 1, 1]
ip=4 [0, 10551309, 3, 1, 1, 1] EqRR 4 1 4 [0, 10551309, 4, 1, 0, 1]
ip=5 [0, 10551309, 4, 1, 0, 1] AddR 4 2 2 [0, 10551309, 5, 1, 0, 1]
ip=6 [0, 10551309, 5, 1, 0, 1] AddI 2 1 2 [0, 10551309, 7, 1, 0, 1]
ip=8 [0, 10551309, 7, 1, 0, 1] AddI 3 1 3 [0, 10551309, 8, 2, 0, 1]
ip=9 [0, 10551309, 8, 2, 0, 1] GtRR 3 1 4 [0, 10551309, 9, 2, 0, 1]
ip=10 [0, 10551309, 9, 2, 0, 1] AddR 2 4 2 [0, 10551309, 10, 2, 0, 1]
ip=11 [0, 10551309, 10, 2, 0, 1] SetI 2 8 2 [0, 10551309, 2, 2, 0, 1]

r1 = 10551309 with everything else redundant


         r1 = inputmagic
         r5 = 1
loop_r3: r3 = 1
         r4 = r5 * r3
         if r4 == r1:
            r0 = r0 + r5
         r3 = r3 + 1
         if r3 <= r1:
            goto loop_r3
         r5 = r5 + 1
         if r5 <= r1
            goto loop_r3
         END




for (r5 = 1; r5++)
  for (r3 = 1; r3++)
     if (r5 * r3) == magic:
        r0 += r5

Sums of factors?

input 909 has factors:

1,3,9,101,303,909

909+303+101+9+3+1
909+404+13
900+400+9+4+13
900+400+13+13
1300+26
1326

Given that, we want to calculate the sum of factors for part 2 when ip=1 and
the target is in r1

Given we can be confident the inputs will vary, it's reasonable to assume that
the target value might not be in R1, however puzzle generation means that
the actual instructions are likely to remain the same, so... we look for an
eqrr, there'll be 2 possible registers to consider from there, A and B.  The
instruction before the eqrr will be a mulr and its target is the one to ignore.

So approx.. find index of eqrr, find instruction before, get target register of
that instruction (C).  Target value is in whichever of A and B of the EQRR is not
the C of the previous instruction.


*/
