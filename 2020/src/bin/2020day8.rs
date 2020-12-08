use aoc2020::*;

#[derive(Debug, ParseByRegex, Clone, Copy)]
enum Instruction {
    #[regex = r"nop (.+)"]
    Nop(i32),
    #[regex = r"acc (.+)"]
    Add(i32),
    #[regex = r"jmp (.+)"]
    Jump(i32),
}

enum VMOutput {
    Terminated(i32),
    InfiniteLoop(i32),
}

struct VM {
    pc: i32,
    acc: i32,
}

impl VM {
    fn new() -> VM {
        Self { pc: 0, acc: 0 }
    }
    fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Nop(_) => self.pc += 1,
            Instruction::Add(n) => {
                self.acc += n;
                self.pc += 1;
            }
            Instruction::Jump(n) => self.pc += n,
        }
    }

    fn run(&mut self, prog: &[Instruction]) -> VMOutput {
        let mut visited = HashSet::new();
        loop {
            if visited.contains(&self.pc) {
                break VMOutput::InfiniteLoop(self.acc);
            }
            visited.insert(self.pc);
            if self.pc < 0 || self.pc >= prog.len() as i32 {
                break VMOutput::Terminated(self.acc);
            }
            self.execute(prog[self.pc as usize]);
        }
    }
}

fn part1(input: &[Instruction]) -> i32 {
    match VM::new().run(input) {
        VMOutput::Terminated(_) => unreachable!(),
        VMOutput::InfiniteLoop(n) => n,
    }
}

fn part2(input: &[Instruction]) -> i32 {
    for pos in 0..input.len() {
        if matches!(input[pos], Instruction::Add(_)) {
            continue;
        }
        let mut prog = input.to_vec();
        prog[pos] = match input[pos] {
            Instruction::Nop(n) => Instruction::Jump(n),
            Instruction::Jump(n) => Instruction::Nop(n),
            i => i,
        };
        if let VMOutput::Terminated(n) = VM::new().run(&prog) {
            return n;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 8);
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec(8)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
