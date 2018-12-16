#![allow(non_snake_case)]
use aoc2018::*;

#[derive(PartialOrd, PartialEq, Ord, Eq, Debug, Copy, Clone, Hash)]
enum Op {
    AddR,
    AddI,
    MulR,
    MulI,
    BanR,
    BanI,
    BorR,
    BorI,
    SetR,
    SetI,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

use self::Op::*;

impl Op {
    fn from_raw(v: u8) -> Op {
        match v {
            0 => AddR,
            1 => AddI,
            2 => MulR,
            3 => MulI,
            4 => BanR,
            5 => BanI,
            6 => BorR,
            7 => BorI,
            8 => SetR,
            9 => SetI,
            10 => GtIR,
            11 => GtRI,
            12 => GtRR,
            13 => EqIR,
            14 => EqRI,
            15 => EqRR,
            _ => unreachable!(),
        }
    }
}

struct VM {
    regs: [i32; 4],
}

impl VM {
    fn new() -> VM {
        VM { regs: [0; 4] }
    }

    fn exec(&mut self, instr: Op, A: i32, B: i32, C: i32) -> Result<()> {
        if C < 0 || C > 3 {
            Err(format!("C ({}) is out of range", C))?
        }
        let rA = if A >= 0 && A <= 3 {
            Ok(self.regs[A as usize])
        } else {
            Err(format!("A ({}) is out of range", A))
        };
        let rB = if B >= 0 && B <= 3 {
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
}

#[derive(ParseByRegex, Debug, Copy, Clone)]
enum InputLine {
    #[regex = r"^Before: \[(-?\d+), (-?\d+), (-?\d+), (-?\d+)\]$"]
    Before(i32, i32, i32, i32),
    #[regex = r"^(\d+) (-?\d+) (-?\d+) (-?\d+)$"]
    Instr(u8, i32, i32, i32),
    #[regex = r"^After:  \[(-?\d+), (-?\d+), (-?\d+), (-?\d+)\]$"]
    After(i32, i32, i32, i32),
}

impl InputLine {
    fn is_before(&self) -> bool {
        match self {
            InputLine::Before(_, _, _, _) => true,
            _ => false,
        }
    }
    fn is_instr(&self) -> bool {
        match self {
            InputLine::Instr(_, _, _, _) => true,
            _ => false,
        }
    }
    fn is_after(&self) -> bool {
        match self {
            InputLine::After(_, _, _, _) => true,
            _ => false,
        }
    }

    fn get_args(&self) -> (i32, i32, i32) {
        match self {
            InputLine::Instr(_, A, B, C) => (*A, *B, *C),
            _ => unreachable!(),
        }
    }

    fn get_opnum(&self) -> u8 {
        match self {
            InputLine::Instr(op, _, _, _) => *op,
            _ => unreachable!(),
        }
    }

    fn get_regs(&self) -> [i32; 4] {
        match self {
            InputLine::Before(r0, r1, r2, r3) => [*r0, *r1, *r2, *r3],
            InputLine::After(r0, r1, r2, r3) => [*r0, *r1, *r2, *r3],
            _ => unreachable!(),
        }
    }
}

fn count_possibilities(input: &[InputLine]) -> Result<Vec<Op>> {
    if input.len() != 3 {
        Err("Incorrect input to count_possibilities: Not 3 lines")?
    }
    if !input[0].is_before() {
        Err("Incorrect input to count_possibilities: First is not Before")?
    }
    if !input[1].is_instr() {
        Err("Incorrect input to count_possibilities: Middle is not Instr")?
    }
    if !input[2].is_after() {
        Err("Incorrect input to count_possibilities: last is not After")?
    }
    let mut ret = Vec::new();
    let (A, B, C) = input[1].get_args();

    for op in (0..16).map(Op::from_raw) {
        let mut vm = VM::new();
        vm.regs = input[0].get_regs();
        vm.exec(op, A, B, C)?;
        if vm.regs == input[2].get_regs() {
            ret.push(op);
        }
    }

    Ok(ret)
}

fn test1() -> Result<()> {
    let input: Result<Vec<InputLine>> = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"
        .lines()
        .map(ParseByRegex::parse_by_regex)
        .collect();
    let input = input?;
    let poss = count_possibilities(&input[0..3])?;
    assert_eq!(poss.len(), 3);
    assert_eq!(poss[0], AddI);
    assert_eq!(poss[1], MulR);
    assert_eq!(poss[2], SetI);
    Ok(())
}

fn part1(input: &[InputLine]) -> Result<usize> {
    let mut ret = 0;
    for chunk in input.chunks(3) {
        if chunk[0].is_before() {
            let poss = count_possibilities(&chunk)?;
            if poss.len() >= 3 {
                ret += 1;
            }
        }
    }
    Ok(ret)
}

fn part2(input: &[InputLine]) -> Result<i32> {
    let mut opmap: HashMap<u8, HashSet<Op>> = HashMap::new();
    let full_opset: HashSet<Op> = (0..16).map(Op::from_raw).collect();
    for chunk in input.chunks(3) {
        if chunk[0].is_before() {
            let poss = count_possibilities(&chunk)?;
            let opnum = chunk[1].get_opnum();
            let curposs = opmap.get(&opnum).unwrap_or(&full_opset);
            let poss: HashSet<Op> = poss.into_iter().collect();
            opmap.insert(opnum, curposs.intersection(&poss).cloned().collect());
        }
    }
    // Counted every possibility, print them out...
    //println!("After running basic scan:");
    //for i in 0..16 {
    //    let opset = opmap
    //        .get(&i)
    //        .ok_or_else(|| format!("Opnum {} not defined", i))?;
    //    println!("OpNum {} has {} possibilities", i, opset.len());
    //}
    // Now we effectively play sudoku until we can resolve the ops
    loop {
        let mut dropops: Vec<Op> = Vec::new();
        for i in 0..16 {
            let opset = opmap.get_mut(&i).expect("Oddness!");
            if opset.len() == 1 {
                dropops.push(opset.iter().next().expect("Oddness!").clone());
            }
        }
        let mut reduced = 0;
        for op in dropops.into_iter() {
            for i in 0..16 {
                let opset = opmap.get_mut(&i).expect("Oddness!");
                if opset.len() != 1 && opset.contains(&op) {
                    opset.remove(&op);
                    reduced += 1;
                }
            }
        }
        if reduced == 0 {
            break;
        }
    }
    for i in 0..16 {
        let opset = opmap
            .get(&i)
            .ok_or_else(|| format!("After run, opnum {} undefined!", i))?;
        if opset.len() != 1 {
            Err(format!(
                "After run, opnum {} has possibility set: {:?}",
                i, opset
            ))?
        }
    }
    let opmap: HashMap<u8, Op> = opmap
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().next().expect("Oddness!")))
        .collect();

    // Now find the index at which we get instructions *only*
    let mut lastafter = 0;
    for (i, inpl) in input.iter().enumerate() {
        if inpl.is_after() {
            lastafter = i;
        }
    }

    let mut vm = VM::new();
    for instr in input[lastafter + 1..].iter() {
        let op = opmap[&instr.get_opnum()];
        let (A, B, C) = instr.get_args();
        vm.exec(op, A, B, C)?;
    }

    Ok(vm.regs[0])
}

fn main() -> Result<()> {
    let raw_input = read_input(16)?;
    let input: Result<Vec<InputLine>> = raw_input
        .lines()
        .filter(|l| !l.is_empty())
        .map(ParseByRegex::parse_by_regex)
        .collect();
    let input = input?;
    test1()?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
