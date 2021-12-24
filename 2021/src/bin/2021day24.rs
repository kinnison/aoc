use aoc2021::*;

#[derive(Debug, Clone, Copy, ParseByRegex, Eq, PartialEq, Hash)]
enum Reg {
    #[regex = "^w$"]
    W,
    #[regex = "^x$"]
    X,
    #[regex = "^y$"]
    Y,
    #[regex = "^z$"]
    Z,
}

impl Reg {
    fn regnum(self) -> usize {
        match self {
            Self::W => 0,
            Self::X => 1,
            Self::Y => 2,
            Self::Z => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, ParseByRegex, PartialEq, Eq, Hash)]
enum BVal {
    #[regex = "^([wxyz])$"]
    Reg(Reg),
    #[regex = r"^(-?\d+)$"]
    Num(i64),
}

impl BVal {
    fn value(self, regs: &[i64; 4]) -> i64 {
        match self {
            Self::Reg(r) => regs[r.regnum()],
            Self::Num(n) => n,
        }
    }
}

#[derive(Debug, ParseByRegex, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    #[regex = r"^inp (.)$"]
    Input(Reg),
    #[regex = r"^add (.) (.+)$"]
    Add(Reg, BVal),
    #[regex = r"^mul (.) (.+)$"]
    Mul(Reg, BVal),
    #[regex = r"^div (.) (.+)$"]
    Div(Reg, BVal),
    #[regex = r"^mod (.) (.+)$"]
    Mod(Reg, BVal),
    #[regex = r"^eql (.) (.+)$"]
    Eql(Reg, BVal),
}

#[memoize]
fn exec_instr(instr: Instruction, regs: [i64; 4], input: i64) -> [i64; 4] {
    let mut regs = regs;
    match instr {
        Instruction::Input(r) => regs[r.regnum()] = input,
        Instruction::Add(r, b) => regs[r.regnum()] += b.value(&regs),
        Instruction::Mul(r, b) => regs[r.regnum()] *= b.value(&regs),
        Instruction::Div(r, b) => regs[r.regnum()] /= b.value(&regs),
        Instruction::Mod(r, b) => regs[r.regnum()] %= b.value(&regs),
        Instruction::Eql(r, b) => regs[r.regnum()] = (regs[r.regnum()] == b.value(&regs)) as i64,
    }
    regs
}

fn run_program<I: Iterator<Item = i64>>(prog: &[Instruction], input: I) -> [i64; 4] {
    let mut regs = [0; 4];
    let mut input = input.fuse();
    let mut ival = input.next().expect("At least one input value expected");
    for instr in prog {
        regs = exec_instr(*instr, regs, ival);
        if matches!(instr, Instruction::Input(_)) {
            ival = input.next().unwrap_or(0);
        }
    }
    regs
}

#[allow(unused)]
fn naive_part1(input: &[Instruction]) -> String {
    let digit = (1i64..=9).rev();
    let model = std::iter::repeat(digit).take(14);
    let model = model.multi_cartesian_product();
    for (n, model) in model.enumerate() {
        if (n % 10000) == 0 {
            println!("Trying model {:?}", model);
        }
        let rmodel = model.iter().copied();
        if run_program(input, rmodel)[3] == 0 {
            // We have found a model which matches the criteria
            return model
                .into_iter()
                .map(|v| (v as u8))
                .map(|b| (b + b'0') as char)
                .collect();
        }
    }
    unreachable!()
}

/*
Analysis

The program appears to be blocks of instructions, repeating, with different parameters
inp w
mul x 0
add x z
mod x 26
div z {param 1}
add x {param 2}
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y {param 3}
mul y x
add z y


If we simplify this, we get:

w = input_digit
x = (z % 26)
z //= {param 1}
x += {param 2}
x = x == w
x = x == 0
z *= (x*25) + 1
z += (w + {param 3}) * x

Input parameters in my input start:

(1, 11, 16)
(1, 12, 11)
(1, 13, 12)
(26, -5, 12)
(26, -3, 12)

Seven times, param 1 is a 1, the other seven it's 26

Simplifying the step block some more gets:

w = input_digit
x = ((z % 26) + {param 2}) != w
z //= {param 1}
z *= (x*25)+1
z += (w+{param 3})*x

In cases where param 1 is 1, the //= step can be ignored.
Also in those steps, param 2 is always > 10, so it can never equal a digit
thus the != step is always true (making x == 1)

So in the case that param 1 is 1, the only parameter which matters is param 3, giving:

w = input_digit
x = 1
z *= 26
z += w + {param 3}

This is, to all intents and purposes, "pushing" digit+param 3 onto a stack in z (mod 26)

In the case where param 1 is 26, instead we get a situation where we "pop" the previous
computed w+{param 3} we add {param 2} and we compare with the input digit
seting x to 1 if they differ and 0 if they match

w = input_digit
if (pop_z + {param 2}) != w {
  push_z w + {param 3}
}

To succeed, this virtual stack needs to be empty, essentially any time we're popping, we want
to ensure we do not push again.  Since our input starts with a push instruction and ends with
a pop instruction, all we need to do is match things up cleanly, so that we can be sure we can
resolve a value which will work.

In pushes, the important value is param 3, since that is added to the digit
In pops, the important value is param 2, since that's added to the popped value before comparing with the digit
*/

#[derive(Debug)]
enum Simplified {
    Push(i64),
    Pop(i64),
}

fn simplify_prog(input: &[Instruction]) -> Vec<Simplified> {
    assert_eq!(input.len(), 14 * 18);
    let mut ret = Vec::new();
    for i in 0..14 {
        let base = i * 18;
        assert!(matches!(input[base], Instruction::Input(Reg::W)));
        let i1 = input[base + 4];
        let i2 = input[base + 5];
        let i3 = input[base + 15];
        //println!("Instruction {}", i);
        //println!("i1 = {:?}", i1);
        //println!("i2 = {:?}", i2);
        //println!("i3 = {:?}", i3);
        let input1 = if let Instruction::Div(Reg::Z, BVal::Num(l)) = i1 {
            l
        } else {
            panic!("i1 is not div z {{num}}")
        };
        let input2 = if let Instruction::Add(Reg::X, BVal::Num(l)) = i2 {
            l
        } else {
            panic!("i2 is not add x {{num}}")
        };
        let input3 = if let Instruction::Add(Reg::Y, BVal::Num(l)) = i3 {
            l
        } else {
            panic!("i3 is not add y {{num}}")
        };
        let instr = if input1 == 1 {
            Simplified::Push(input3)
        } else {
            Simplified::Pop(input2)
        };
        ret.push(instr);
    }
    ret
}

/*

With a simplified program we can do digit pairing, this is done by managing the stack
and looking basically for constraints which will be of the form "digit X must equal digit Y + some value"
there will be seven such constraints which will give us what will effectively be our goal constraints

*/

#[derive(Debug, Clone, Copy)]
struct Constraint {
    x: usize,
    y: usize,
    val: i8,
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "digits[{}] must equal digits[{}] + {}",
            self.y, self.x, self.val
        )
    }
}

fn gen_constraints(input: &[Simplified]) -> Vec<Constraint> {
    let mut ret = Vec::new();
    let mut stack = Vec::new();
    for (digit, op) in input.iter().enumerate() {
        match op {
            Simplified::Push(val) => {
                stack.push((digit, (*val as i8)));
            }
            Simplified::Pop(val) => {
                let (digit1, partial) = stack.pop().unwrap();
                ret.push(Constraint {
                    x: digit1,
                    y: digit,
                    val: (*val as i8) + partial,
                });
            }
        }
    }
    ret
}

fn part1(input: &[Constraint]) -> String {
    let mut digits = [0i8; 14];
    for Constraint { x, y, val } in input.iter().copied() {
        if val < 0 {
            digits[x] = 9;
            digits[y] = 9 + val;
        } else {
            digits[y] = 9;
            digits[x] = 9 - val;
        }
    }

    digits
        .into_iter()
        .map(|v| ((v as u8) + b'0') as char)
        .collect()
}
fn part2(input: &[Constraint]) -> String {
    let mut digits = [0i8; 14];
    for Constraint { x, y, val } in input.iter().copied() {
        if val < 0 {
            digits[x] = -val + 1;
            digits[y] = 1;
        } else {
            digits[y] = val + 1;
            digits[x] = 1;
        }
    }

    digits
        .into_iter()
        .map(|v| ((v as u8) + b'0') as char)
        .collect()
}

fn main() -> Result<()> {
    let input: Vec<Instruction> = read_input_as_vec(24)?;
    let simplified = simplify_prog(&input);
    let constraints = gen_constraints(&simplified);
    println!("Part 1: {}", part1(&constraints));
    println!("Part 2: {}", part2(&constraints));
    Ok(())
}
