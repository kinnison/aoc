struct Generator {
    curval: usize,
    factor: usize,
    divisor: usize,
}

impl Generator {
    fn new(init: usize, fact: usize) -> Generator {
        Generator {
            curval: init,
            factor: fact,
            divisor: 2147483647,
        }
    }

    fn next(&mut self) -> usize {
        self.curval = (self.curval * self.factor) % self.divisor;
        self.curval
    }

    fn next_masked(&mut self, mask: usize) -> usize {
        loop {
            let candidate = self.next();
            if (candidate & mask) == 0 {
                return candidate;
            }
        }
    }
}

fn problem1(facts: &(usize, usize), inits: &(usize, usize)) -> usize {
    let mut gen_a = Generator::new(inits.0, facts.0);
    let mut gen_b = Generator::new(inits.1, facts.1);
    let mut total = 0;
    for _ in 0..40_000_000 {
        let val_a = gen_a.next();
        let val_b = gen_b.next();
        if (val_a & 0xFFFF) == (val_b & 0xFFFF) {
            total += 1;
        }
    }
    total
}

fn problem2(facts: &(usize, usize), inits: &(usize, usize)) -> usize {
    let mut gen_a = Generator::new(inits.0, facts.0);
    let mut gen_b = Generator::new(inits.1, facts.1);
    let mut total = 0;
    for _ in 0..5_000_000 {
        let val_a = gen_a.next_masked(3); // only multiples of 4
        let val_b = gen_b.next_masked(7); // only multiples of 8
        if (val_a & 0xFFFF) == (val_b & 0xFFFF) {
            total += 1;
        }
    }
    total
}

fn main() {
    let factors: (usize, usize) = (16807, 48271);
    let example: (usize, usize) = (65, 8921);
    println!(
        "Example answer for problem1: {}",
        problem1(&factors, &example)
    );
    println!(
        "Example answer for problem2: {}",
        problem2(&factors, &example)
    );
    let input: (usize, usize) = (277, 349);
    println!("Problem1: {}", problem1(&factors, &input));
    println!("Problem2: {}", problem2(&factors, &input));
}
