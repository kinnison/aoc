use aoc2022::*;

type MVal = i64;

#[derive(Clone, Debug, ParseByRegex)]
enum Operation {
    #[regex = r"(\d+)"]
    Const(MVal),
    #[regex = r"(.+?) \+ (.+)"]
    Add(String, String),
    #[regex = r"(.+?) \- (.+)"]
    Sub(String, String),
    #[regex = r"(.+?) \* (.+)"]
    Mul(String, String),
    #[regex = r"(.+?) / (.+)"]
    Div(String, String),
}

impl Operation {
    fn left(&self) -> &str {
        match self {
            Operation::Const(_) => unreachable!(),
            Operation::Add(a, _) => a,
            Operation::Sub(a, _) => a,
            Operation::Mul(a, _) => a,
            Operation::Div(a, _) => a,
        }
    }
    fn right(&self) -> &str {
        match self {
            Operation::Const(_) => unreachable!(),
            Operation::Add(_, b) => b,
            Operation::Sub(_, b) => b,
            Operation::Mul(_, b) => b,
            Operation::Div(_, b) => b,
        }
    }
}

#[derive(ParseByRegex, Clone, Debug)]
#[regex = r"(?P<name>[^:]+): (?P<op>.+)"]
struct MonkeyDef {
    name: String,
    op: Operation,
}

struct Troop<'a> {
    rulemap: HashMap<&'a str, &'a MonkeyDef>,
    cache: HashMap<&'a str, MVal>,
}

impl<'a> Troop<'a> {
    fn new(rules: &'a [MonkeyDef]) -> Self {
        let rulemap = rules.iter().map(|def| (def.name.as_str(), def)).collect();
        Self {
            rulemap,
            cache: HashMap::new(),
        }
    }

    fn evaluate(&mut self, name: &'a str) -> MVal {
        if let Some(val) = self.cache.get(name) {
            //println!("Cached: {name} => {val}");
            return *val;
        }
        //println!("Compute {name}");
        let op = self.rulemap.get(name).unwrap();
        //println!("Operation: {op:?}");
        let val = match &op.op {
            Operation::Const(n) => *n,
            Operation::Add(a, b) => self.evaluate(a) + self.evaluate(b),
            Operation::Sub(a, b) => self.evaluate(a) - self.evaluate(b),
            Operation::Mul(a, b) => self.evaluate(a) * self.evaluate(b),
            Operation::Div(a, b) => self.evaluate(a) / self.evaluate(b),
        };
        self.cache.insert(name, val);
        val
    }
    fn try_evaluate(&mut self, name: &'a str) -> Option<MVal> {
        if name == "humn" {
            return None;
        }
        if let Some(val) = self.cache.get(name) {
            //println!("Cached: {name} => {val}");
            return Some(*val);
        }
        //println!("Compute {name}");
        let op = self.rulemap.get(name).unwrap();
        //println!("Operation: {op:?}");
        let val = match &op.op {
            Operation::Const(n) => Some(*n),
            Operation::Add(a, b) => {
                let a = self.try_evaluate(a);
                let b = self.try_evaluate(b);
                if let (Some(a), Some(b)) = (a, b) {
                    Some(a + b)
                } else {
                    None
                }
            }
            Operation::Sub(a, b) => {
                let a = self.try_evaluate(a);
                let b = self.try_evaluate(b);
                if let (Some(a), Some(b)) = (a, b) {
                    Some(a - b)
                } else {
                    None
                }
            }
            Operation::Mul(a, b) => {
                let a = self.try_evaluate(a);
                let b = self.try_evaluate(b);
                if let (Some(a), Some(b)) = (a, b) {
                    Some(a * b)
                } else {
                    None
                }
            }
            Operation::Div(a, b) => {
                let a = self.try_evaluate(a);
                let b = self.try_evaluate(b);
                if let (Some(a), Some(b)) = (a, b) {
                    Some(a / b)
                } else {
                    None
                }
            }
        };
        if let Some(val) = val {
            //println!("Caching {name} => {val}");
            self.cache.insert(name, val);
        }
        val
    }
}

fn part1(input: &[MonkeyDef]) -> MVal {
    let mut troop = Troop::new(input);
    troop.evaluate("root")
}

fn part2(input: &[MonkeyDef]) -> MVal {
    // Step one, compute as much as we possibly can.
    let mut troop = Troop::new(input);
    troop.try_evaluate("root");

    // At this point, one side of root is computed, the other we need to work out
    let (target, start) = {
        let root = troop.rulemap["root"];
        if let Some(val) = troop.cache.get(root.op.left()) {
            (root.op.right(), *val)
        } else {
            (root.op.left(), troop.cache[root.op.right()])
        }
    };

    //println!("To compute root, we start with value {start} and reverse rule for {target}");

    let mut value = start;
    let mut rule = troop.rulemap[target];

    while rule.name != "humn" {
        // Undo one computation step, find which side of the rule we already know, and compute the
        // value we need out of the other side in order for this rule to evaluate to value
        let (left, right) = (rule.op.left(), rule.op.right());
        let (leftval, rightval) = (troop.cache.get(left), troop.cache.get(right));
        if let Some(val) = leftval {
            // we have a left value, we need a right value, compute...
            match &rule.op {
                Operation::Const(_) => unreachable!(),
                Operation::Add(_, _) => value -= *val, // We have value = val + ??? which means ??? = value - val
                Operation::Sub(_, _) => value = val - value, // We have value = val - ??? which means ??? = val - value
                Operation::Mul(_, _) => value /= *val, // We have value = val * ??? which means ??? = value / val
                Operation::Div(_, _) => value = val / value, // We have value = val / ??? which means ??? = val / value
            }
            rule = troop.rulemap[right];
        } else if let Some(val) = rightval {
            // We have a right value, we need a left value, compute ...
            match &rule.op {
                Operation::Const(_) => unreachable!(),
                Operation::Add(_, _) => value -= *val, // Whichever way around, we always need val fewer
                Operation::Sub(_, _) => value += *val, // We have value = ??? - val, which means ??? = value + val
                Operation::Mul(_, _) => value /= *val, // Whichever way around, we always need val times fewer
                Operation::Div(_, _) => value *= *val, // We have value = ??? / val, so ??? = value * val
            }
            rule = troop.rulemap[left];
        } else {
            unreachable!()
        }
    }

    value
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part1(dbg!(&input)), 152);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 301);
    }
}

pub fn main() -> Result<()> {
    let input: Vec<MonkeyDef> = read_input_as_vec(21)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
