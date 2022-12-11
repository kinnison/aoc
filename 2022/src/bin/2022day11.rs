use aoc2022::*;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: u64,
    iftrue: usize,
    iffalse: usize,
    inspections: usize,
}

#[derive(ParseByRegex, Debug, Copy, Clone)]
enum Operation {
    #[regex = r"new = old \* old"]
    Square,
    #[regex = r"new = old \* (\d+)"]
    Times(u64),
    #[regex = r"new = old \+ (\d+)"]
    Add(u64),
}

impl Operation {
    fn consider(&self, n: u64) -> u64 {
        match *self {
            Operation::Square => n * n,
            Operation::Times(m) => n * m,
            Operation::Add(m) => n + m,
        }
    }
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let lines = s.lines().skip(1).map(str::trim).collect_vec();

        let items = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect_vec();

        let op = Operation::parse_by_regex(lines[1].split_once(": ").unwrap().1.trim()).unwrap();

        let test = lines[2].split_once(" by ").unwrap().1.parse().unwrap();

        let iftrue = lines[3]
            .split_once("to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        let iffalse = lines[4]
            .split_once("to monkey ")
            .unwrap()
            .1
            .parse()
            .unwrap();

        Ok(Self {
            items,
            op,
            test,
            iftrue,
            iffalse,
            inspections: 0,
        })
    }
}

impl Monkey {
    fn target(&self, n: u64) -> usize {
        if (n % self.test) == 0 {
            self.iftrue
        } else {
            self.iffalse
        }
    }
}

fn run_round1(monkeys: &mut Vec<Monkey>) {
    for n in 0..monkeys.len() {
        while !monkeys[n].items.is_empty() {
            let item = monkeys[n].items.remove(0);
            let item = monkeys[n].op.consider(item) / 3;
            let target = monkeys[n].target(item);
            monkeys[target].items.push(item);
            monkeys[n].inspections += 1;
        }
    }
}

fn part1(input: &[Monkey]) -> usize {
    let mut monkeys = input.iter().cloned().collect_vec();
    // run 20 rounds
    for _ in 0..20 {
        run_round1(&mut monkeys);
    }
    monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

fn run_round2(monkeys: &mut Vec<Monkey>, field: u64) {
    for n in 0..monkeys.len() {
        while !monkeys[n].items.is_empty() {
            let item = monkeys[n].items.remove(0);
            let item = monkeys[n].op.consider(item) % field;
            let target = monkeys[n].target(item);
            monkeys[target].items.push(item);
            monkeys[n].inspections += 1;
        }
    }
}

fn part2(input: &[Monkey]) -> usize {
    let mut monkeys = input.iter().cloned().collect_vec();
    let field = monkeys.iter().map(|m| m.test).product();
    for _ in 0..10_000 {
        run_round2(&mut monkeys, field);
    }
    monkeys
        .iter()
        .map(|m| m.inspections)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    fn testcase1() {
        let input: Vec<Monkey> = input_as_chunks(TEST_INPUT).unwrap();
        assert_eq!(part1(dbg!(&input)), 10605);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Monkey> = input_as_chunks(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 2713310158);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<Monkey> = read_input_as_chunks(11)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
