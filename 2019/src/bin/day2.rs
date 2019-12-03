use aoc2019::*;

struct IntCodeMachine {
    code: Vec<usize>,
}

impl IntCodeMachine {
    fn new(code: &str) -> Self {
        let spl: Vec<usize> = code
            .split(',')
            .map(|n| n.parse().expect("Unable to parse"))
            .collect();
        Self { code: spl }
    }

    fn value_at(&self, idx: usize) -> Option<usize> {
        self.code.get(idx).copied()
    }

    fn set_at(&mut self, pos: usize, value: usize) {
        self.code[pos] = value;
    }

    fn run(&mut self) -> bool {
        let mut pc = 0;
        while let Some(instr) = self.code.get(pc) {
            if *instr == 99 {
                return true;
            }
            let first = self.code[pc + 1];
            let second = self.code[pc + 2];
            let third = self.code[pc + 3];
            match instr {
                1 => self.code[third] = self.code[first] + self.code[second],
                2 => self.code[third] = self.code[first] * self.code[second],
                _ => panic!("Unknown instruction {} at {}", instr, pc),
            }
            pc += 4;
        }
        false // Did not exit cleanly
    }

    #[cfg(test)]
    fn run_and_ret(code: &str, pos: usize) -> Option<usize> {
        let mut vm = Self::new(code);
        if !vm.run() {
            None
        } else {
            vm.value_at(pos)
        }
    }
}

#[cfg(test)]
mod test {
    use super::IntCodeMachine;
    #[test]
    fn test_cases_1() {
        assert_eq!(IntCodeMachine::run_and_ret("1,0,0,0,99", 0), Some(2));
        assert_eq!(IntCodeMachine::run_and_ret("2,3,0,3,99", 3), Some(6));
        assert_eq!(IntCodeMachine::run_and_ret("2,4,4,5,99,0", 5), Some(9801));
        assert_eq!(
            IntCodeMachine::run_and_ret("1,1,1,4,99,5,6,0,99", 0),
            Some(30)
        );
    }

    #[test]
    fn test_cases_2() {}
}

fn run_nv(code: &str, noun: usize, verb: usize) -> usize {
    let mut vm = IntCodeMachine::new(code);
    vm.set_at(1, noun);
    vm.set_at(2, verb);
    assert!(vm.run());
    vm.value_at(0).expect("bug?")
}

fn part1(code: &str) -> usize {
    run_nv(code, 12, 2)
}

fn part2(code: &str) -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_nv(code, noun, verb) == 19_690_720 {
                return (100 * noun) + verb;
            }
        }
    }
    unreachable!()
}

fn main() -> Result<()> {
    let input = read_input(2)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
