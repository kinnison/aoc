use std::vec::Vec;

struct Hurricane {
    entries: Vec<usize>,
    cap: usize,
    pos: usize,
    step: usize,
}

impl Hurricane {
    fn new(cap: usize, step: usize) -> Hurricane {
        let mut ret = Hurricane {
            entries: Vec::with_capacity(cap),
            cap,
            pos: 0,
            step,
        };
        ret.entries.push(0);
        ret
    }

    fn run(&mut self) {
        for v in 1..self.cap {
            // First we step through
            self.pos = (self.pos + self.step) % self.entries.len();
            // Next we "insert"
            if self.pos == self.entries.len() - 1 {
                self.entries.push(v);
            } else {
                self.entries.insert(self.pos + 1, v);
            }
            // Now we use the newly inserted as our position
            self.pos += 1;
        }
    }
}

fn problem1(input: usize) -> usize {
    let mut hurricane = Hurricane::new(2018, input);
    hurricane.run();
    hurricane.entries[(hurricane.pos + 1) % hurricane.entries.len()]
}

fn problem2(input: usize) -> usize {
    // Faking the hurricane instead is funsies
    // Since we only care about the value at location 1 which is only
    // ever changed if pos is zero on insertion, we can fake the
    // hurricane with two values
    let mut pos = 0;
    let mut afterpos = 0; // Lies, but who cares?
    for size in 1..50_000_001 {
        pos = (pos + input) % size;
        if pos == 0 {
            afterpos = size;
        }
        pos += 1;
    }
    afterpos
}

fn main() {
    println!("Problem1 example: {}", problem1(3));
    println!("Problem 1: {}", problem1(356));
    println!("Problem 2: {}", problem2(356));
}
