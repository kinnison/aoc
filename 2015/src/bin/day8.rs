use aoc2015::*;

struct Sticks {
    inner: String,
}

impl Sticks {
    fn new(val: &str) -> Sticks {
        Sticks {
            inner: val.to_owned(),
        }
    }

    fn code_len(&self) -> usize {
        self.inner.len()
    }

    fn value_len(&self) -> usize {
        let mut tot = 0;
        let mut iter = self.inner.chars();
        while let Some(ch) = iter.next() {
            if ch != '\\' {
                tot += 1;
                continue;
            }
            if let Some(ch) = iter.next() {
                if ch == '\\' || ch == '\"' {
                    tot += 1;
                    continue;
                }
                assert!(ch == 'x');
                iter.next();
                iter.next();
                tot += 1;
            }
        }
        // There are always ""s so remove 2
        tot - 2
    }

    fn encoded_len(&self) -> usize {
        let mut tot = 0;
        for ch in self.inner.chars() {
            match ch {
                '"' => tot += 2,  // \"
                '\\' => tot += 2, // \\
                _ => tot += 1,
            }
        }
        // And add 2 for the ""s
        tot + 2
    }
}

fn part1(input: &Vec<Sticks>) -> usize {
    let mut totcode = 0;
    let mut totlen = 0;
    for stick in input {
        totcode += stick.code_len();
        totlen += stick.value_len();
    }
    totcode - totlen
}

fn part2(input: &Vec<Sticks>) -> usize {
    let mut enclen = 0;
    let mut codelen = 0;
    for stick in input {
        codelen += stick.code_len();
        enclen += stick.encoded_len();
    }
    enclen - codelen
}

fn main() -> Result<()> {
    let testvec = vec![
        Sticks::new(r#""""#),
        Sticks::new(r#""abc""#),
        Sticks::new(r#""aaa\"aaa""#),
        Sticks::new(r#""\x27""#),
    ];
    println!("Test 1: {}", part1(&testvec));
    println!("Test 2: {}", part2(&testvec));
    let input: Vec<Sticks> = read_input(8)?.lines().map(Sticks::new).collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
