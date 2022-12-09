use aoc2022::*;

#[derive(ParseByRegex, Debug, Copy, Clone)]
#[regex = r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)"]
struct Op {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
struct Piles {
    crates: Vec<Vec<char>>,
}

impl<T> From<T> for Piles
where
    T: AsRef<str>,
{
    fn from(input: T) -> Self {
        let input = input.as_ref();
        let mut ret = Self { crates: vec![] };
        for line in input.lines().rev().skip(1) {
            for (col, crateval) in line
                .chars()
                .chunks(4)
                .into_iter()
                .map(|mut cv| cv.nth(1).unwrap())
                .enumerate()
            {
                while ret.crates.len() <= col {
                    ret.crates.push(vec![]);
                }
                if crateval != ' ' {
                    ret.crates[col].push(crateval);
                }
            }
        }
        ret
    }
}

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maxn = self.crates.iter().map(Vec::len).max().unwrap();
        writeln!(f, "Column count: {}", self.crates.len())?;
        writeln!(f, "Max height: {}", maxn)?;
        for row in (0..maxn).rev() {
            for col in self.crates.iter() {
                if col.len() <= row {
                    write!(f, "    ")?
                } else {
                    write!(f, "[{}] ", col[row])?
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Input {
    piles: Piles,
    ops: Vec<Op>,
}

impl<T> From<T> for Input
where
    T: AsRef<str>,
{
    fn from(input: T) -> Self {
        let input = input.as_ref();
        let (piles, ops) = input.split_once("\n\n").expect("No piles/ops separator");
        let piles = Piles::from(piles);
        let ops = input_as_vec(ops).unwrap();
        Self { piles, ops }
    }
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Input")
            .field("piles", &self.piles)
            .field("ops", &self.ops)
            .finish()
    }
}

impl Piles {
    fn perform(&mut self, op: &Op) {
        for _ in 0..op.count {
            let crateval = self.crates[op.from - 1].pop().unwrap();
            self.crates[op.to - 1].push(crateval);
        }
    }

    fn perform2(&mut self, op: &Op) {
        let tip = self.crates[op.from - 1].len() - 1;
        for i in (0..op.count).rev() {
            let crateval = self.crates[op.from - 1][tip - i];
            self.crates[op.to - 1].push(crateval);
        }
        for _ in 0..op.count {
            self.crates[op.from - 1].pop();
        }
    }

    fn tips(&self) -> String {
        self.crates
            .iter()
            .map(|v| v.iter().copied().last().unwrap_or(' '))
            .collect()
    }
}

fn part1(input: &Input) -> String {
    let Input { mut piles, ops } = input.clone();
    for op in &ops {
        piles.perform(op);
    }
    piles.tips()
}

fn part2(input: &Input) -> String {
    let Input { mut piles, ops } = input.clone();
    for op in &ops {
        piles.perform2(op);
    }
    piles.tips()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn testcase1() {
        let input = Input::from(TEST_INPUT);
        println!("{:#?}", input);
        assert_eq!(part1(&input), "CMZ");
    }

    #[test]
    fn testcase2() {
        let input = Input::from(TEST_INPUT);
        println!("{:#?}", input);
        assert_eq!(part2(&input), "MCD");
    }
}

pub fn main() -> Result<()> {
    let input = read_input(5)?;
    let input = Input::from(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
