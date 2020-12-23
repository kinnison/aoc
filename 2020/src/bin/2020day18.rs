use aoc2020::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Part {
    Number(usize),
    Add,
    Multiply,
    Descend,
    Return,
}

use Part::*;

fn parse_expr(s: &str) -> Result<Vec<Part>> {
    let mut ret = Vec::new();
    let mut numacc = String::new();
    for c in s.chars() {
        if !matches!(c, '0'..='9') && !numacc.is_empty() {
            ret.push(Number(numacc.parse()?));
            numacc = String::new();
        }
        match c {
            ' ' => {}
            '+' => ret.push(Add),
            '*' => ret.push(Multiply),
            '(' => ret.push(Descend),
            ')' => ret.push(Return),
            '0'..='9' => numacc.push(c),
            _ => unimplemented!(),
        }
    }
    if !numacc.is_empty() {
        ret.push(Number(numacc.parse()?));
    }
    Ok(ret)
}

fn expr_to_postfix(input: impl Iterator<Item = Part>) -> Vec<Part> {
    let mut ret = Vec::new();
    let mut opstack = Vec::new();
    for part in input {
        match part {
            Number(_) => {
                //println!("Emit1: {}", n);
                ret.push(part);
            }
            Add | Multiply => {
                // while the opstack is not empty
                while !opstack.is_empty() {
                    let op = opstack[opstack.len() - 1];
                    let cont =
                        // and op is greater precedence than part
                        (op == Add && part == Multiply)
                        // or op is equal to part and left-assoc
                        || (op == part)
                        ;
                    if !cont {
                        break;
                    }
                    // and the operator at the top of the stack is not Descend
                    if op == Descend {
                        break;
                    }
                    // then pop from the opstack and push to ret
                    //println!("Emit2: {:?}", op);
                    ret.push(opstack.pop().unwrap());
                }
                //println!("Stack: {:?}", part);
                opstack.push(part);
            }
            Descend => {
                //println!("Stack: Descend");
                opstack.push(part);
            }
            Return => {
                loop {
                    if opstack.is_empty() {
                        break;
                    };
                    match opstack.pop().unwrap() {
                        Descend => {
                            //println!("Found descend");
                            break;
                        }
                        other => {
                            //println!("Emit3: {:?}", other);
                            ret.push(other);
                        }
                    }
                }
            }
        }
    }
    while let Some(op) = opstack.pop() {
        //println!("Emit5: {:?}", op);
        ret.push(op);
    }
    ret
}

fn p1eval(input: &mut impl Iterator<Item = Part>) -> usize {
    let mut acc = match input.next() {
        Some(Number(number)) => number,
        Some(Descend) => p1eval(input),
        _ => unimplemented!(),
    };
    let mut op = None;
    let mut nval = None;
    //println!("Start with {}", acc);
    loop {
        match input.next() {
            None | Some(Return) => {
                //println!("Returning {}", acc);
                break acc;
            }
            Some(Add) => op = Some(Add),
            Some(Multiply) => op = Some(Multiply),
            Some(Descend) => nval = Some(p1eval(input)),
            Some(Number(n)) => nval = Some(n),
        };
        match (op, nval) {
            (None, _) => {}
            (_, None) => {}
            (Some(Add), Some(donval)) => {
                op = None;
                nval = None;
                acc += donval;
                //println!("Add {} -> {}", donval, acc);
            }
            (Some(Multiply), Some(donval)) => {
                op = None;
                nval = None;
                acc *= donval;
                //println!("Times {} -> {}", donval, acc);
            }
            (Some(_), Some(_)) => unimplemented!(),
        }
    }
}

fn eval_postfix(input: &[Part]) -> usize {
    let mut numstack = Vec::new();
    for part in input.iter().copied() {
        match part {
            Number(n) => numstack.push(n),
            Add => {
                let right = numstack.pop().unwrap();
                let left = numstack.pop().unwrap();
                numstack.push(left + right);
            }
            Multiply => {
                let right = numstack.pop().unwrap();
                let left = numstack.pop().unwrap();
                numstack.push(left * right);
            }
            _ => unimplemented!(),
        }
    }
    numstack.pop().unwrap()
}

fn part1(input: &[Vec<Part>]) -> usize {
    input.iter().map(|v| p1eval(&mut v.iter().copied())).sum()
}

fn part2(input: &[Vec<Part>]) -> usize {
    input
        .iter()
        .map(|v| {
            let pfe = expr_to_postfix(&mut v.iter().copied());
            eval_postfix(&pfe)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT1: &[(&str, usize)] = &[
        (r#"1 + 2 * 3 + 4 * 5 + 6"#, 71),
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("2 * 3 + (4 * 5)", 26),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    ];

    #[test]
    fn testcase1() {
        for (s, n) in TEST_INPUT1.iter().copied() {
            println!("Test case: {} = {}", s, n);
            let input = parse_expr(s).unwrap();
            println!("Test case: {:?}", input);
            assert_eq!(p1eval(&mut input.iter().copied()), n);
        }
    }

    const TEST_INPUT2: &[(&str, usize)] = &[
        ("1 + 2 * 3 + 4 * 5 + 6", 231),
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
    ];

    #[test]
    fn testcase2() {
        for (s, n) in TEST_INPUT2.iter().copied() {
            println!("Test case: {} = {}", s, n);
            let input = parse_expr(s).unwrap();
            println!("Parsed: {:?}", input);
            let input = expr_to_postfix(input.into_iter());
            println!("Postfix: {:?}", input);
            assert_eq!(eval_postfix(&input), n);
        }
    }
}

fn main() -> Result<()> {
    let input = read_input(18)?;
    let input: Result<Vec<Vec<Part>>> = input.trim().lines().map(parse_expr).collect();
    let input = input?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
