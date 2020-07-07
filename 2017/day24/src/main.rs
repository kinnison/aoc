use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Clone, PartialEq, Eq)]
struct Part {
    a: usize,
    b: usize,
}

impl Part {
    fn new(s: &str) -> Part {
        let parts: Vec<usize> = s.split("/").map(|s| s.trim().parse().unwrap()).collect();
        assert!(parts.len() == 2);
        Part {
            a: parts[0],
            b: parts[1],
        }
    }

    fn has(&self, n: usize) -> bool {
        self.a == n || self.b == n
    }

    fn other(&self, n: usize) -> usize {
        if self.a == n {
            self.b
        } else {
            self.a
        }
    }

    fn strength(&self) -> usize {
        self.a + self.b
    }
}

fn load_instructions(s: &str) -> Vec<Part> {
    let infile = File::open(s).unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        let inst = Part::new(&line);
        ret.push(inst);
    }
    ret
}

#[derive(Clone)]
struct Bridge {
    seq: Vec<Part>,
    desired: usize,
}

impl Bridge {
    fn new() -> Bridge {
        Bridge {
            seq: Vec::new(),
            desired: 0,
        }
    }

    fn add_part(&self, part: &Part) -> Bridge {
        let mut ret = self.clone();
        ret.seq.push(part.clone());
        ret.desired = part.other(self.desired);
        ret
    }

    fn build_all(&self, remaining: &Vec<Part>, bridges: &mut Vec<Bridge>) {
        for nextpart in remaining.iter().filter(|p| p.has(self.desired)) {
            // Construct a bridge out of self, nextpart
            let nextbridge = self.add_part(nextpart);
            bridges.push(nextbridge.clone());
            let nextset = remaining
                .iter()
                .filter(|p| *p != nextpart)
                .map(|p| p.clone())
                .collect();
            nextbridge.build_all(&nextset, bridges);
        }
    }

    fn strength(&self) -> usize {
        // Strength of bridge is the sum of the parts' strengths
        self.seq.iter().map(|p| p.strength()).sum()
    }

    fn strongest(bridges: &Vec<Bridge>) -> usize {
        bridges.iter().map(|b| b.strength()).max().unwrap()
    }

    fn len(&self) -> usize {
        self.seq.len()
    }
}

fn problem(input: &Vec<Part>) -> (usize, usize) {
    let mut all_valid = Vec::new();
    let base = Bridge::new();
    base.build_all(&input, &mut all_valid);
    let strongest = Bridge::strongest(&all_valid);
    let maxlen = all_valid.iter().map(|b| b.len()).max().unwrap();
    let longs = all_valid.drain(..).filter(|b| b.len() == maxlen).collect();
    let longest = Bridge::strongest(&longs);
    (strongest, longest)
}

fn main() {
    let example = load_instructions("example");
    println!("Loaded {} example parts", example.len());
    let (example_strongest, example_longest) = problem(&example);
    println!("Strongest example bridge is {}", example_strongest);
    println!("Strength of longest bridge is {}", example_longest);
    let input = load_instructions("input");
    println!("Loaded {} input parts", input.len());
    let (input_strongest, input_longest) = problem(&input);
    println!("Strongest input bridge is {}", input_strongest);
    println!("Strength of longest bridge is {}", input_longest);
}
