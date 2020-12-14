use std::convert::Infallible;

use aoc2020::*;
#[derive(ParseByRegex, Copy, Clone)]
enum Instruction {
    #[regex = "mask = (.+)"]
    Mask(Mask),
    #[regex = r"mem\[(\d+)\] = (\d+)"]
    Set(usize, u64),
}

#[derive(Copy, Clone)]
struct Mask {
    and: u64,
    xor: u64,
}

impl FromStr for Mask {
    type Err = Infallible;

    fn from_str(value: &str) -> StdResult<Self, Self::Err> {
        let mut and = 0;
        let mut xor = 0;
        for ch in value.chars() {
            and <<= 1;
            xor <<= 1;
            match ch {
                'x' | 'X' => and |= 1,
                '1' => xor |= 1,
                '0' => {}
                _ => panic!("Unknown mask character {}", ch),
            }
        }
        Ok(Self { and, xor })
    }
}

struct Chip {
    mask: Mask,
    mem: HashMap<usize, u64>,
}

impl Chip {
    fn new() -> Chip {
        Chip {
            mask: Mask {
                and: 0xf_ffff_ffff,
                xor: 0,
            },
            mem: HashMap::new(),
        }
    }

    fn ramtotal(&self) -> u64 {
        self.mem.values().copied().sum()
    }

    fn run1(&mut self, prog: &[Instruction]) {
        for instr in prog {
            match *instr {
                Instruction::Mask(m) => self.mask = m,
                Instruction::Set(loc, val) => {
                    *(self.mem.entry(loc).or_default()) = (val & self.mask.and) ^ self.mask.xor;
                }
            }
        }
    }

    fn run2(&mut self, prog: &[Instruction]) {
        for instr in prog {
            match *instr {
                Instruction::Mask(m) => self.mask = m,
                Instruction::Set(loc, val) => {
                    // X will be given 0 and 1, so invert the and mask
                    // 0 unchanged, 1 overwrites, so or in the xor mask rather than xor
                    let base_loc = (loc as u64 & !self.mask.and) | self.mask.xor;
                    // Now for every 1 bit in the and mask we need to try 0 and 1
                    for loc in AllLocs::new(base_loc, self.mask.and) {
                        *self.mem.entry(loc).or_default() = val;
                    }
                }
            }
        }
    }
}
#[derive(Debug)]
struct AllLocs {
    base: u64,
    idxs: Vec<usize>,
    n: usize,
    maxn: usize,
}

impl AllLocs {
    fn new(base: u64, mask: u64) -> Self {
        let mut idxs = Vec::new();
        for i in 0..36 {
            if (mask & (1 << i)) != 0 {
                idxs.push(i);
            }
        }
        let maxn = 1 << idxs.len();
        Self {
            base,
            idxs,
            n: 0,
            maxn,
        }
    }
}

impl Iterator for AllLocs {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == self.maxn {
            None
        } else {
            let mut ret = self.base;
            for (bp, idx) in self.idxs.iter().copied().enumerate() {
                if (self.n & (1 << bp)) != 0 {
                    ret |= 1 << idx;
                }
            }
            self.n += 1;
            Some(ret as usize)
        }
    }
}

fn part1(input: &[Instruction]) -> u64 {
    let mut chip = Chip::new();
    chip.run1(input);
    chip.ramtotal()
}

fn part2(input: &[Instruction]) -> u64 {
    let mut chip = Chip::new();
    chip.run2(input);
    chip.ramtotal()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(&input), 165);
    }

    const TEST_INPUT2: &str = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT2).unwrap();
        assert_eq!(part2(&input), 208);
    }
}

fn main() -> Result<()> {
    let input = read_input_as_vec(14)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
