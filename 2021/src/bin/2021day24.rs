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

fn part1(input: &[Instruction]) -> String {
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

fn part2(input: &[Instruction]) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#""#;

    #[test]
    fn testcase1() {
        let input: Vec<Instruction> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), "7");
    }

    #[test]
    fn testcase2() {
        let input: Vec<Instruction> = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 5);
    }
}

fn main() -> Result<()> {
    let input: Vec<Instruction> = read_input_as_vec(24)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
