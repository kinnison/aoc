use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Debug, Clone)]
struct Firewall {
    scanners: Vec<usize>,
    depths: Vec<usize>,
    directions: Vec<bool>,
    maxdepth: usize,
}

impl Firewall {
    fn new() -> Firewall {
        Firewall {
            scanners: Vec::new(),
            depths: Vec::new(),
            directions: Vec::new(),
            maxdepth: 0,
        }
    }

    fn add_scanner(&mut self, line: &str) {
        let bits: Vec<usize> = line.split(":").map(|s| s.trim().parse().unwrap()).collect();
        assert!(bits.len() == 2);
        let layer = bits[0];
        let depth = bits[1];
        self.add_scanner_(layer, depth);
    }

    fn add_scanner_(&mut self, layer: usize, depth: usize) {
        while (layer as i32) > ((self.scanners.len() as i32) - 1) {
            self.scanners.push(0);
            self.depths.push(0);
            self.directions.push(true);
        }
        self.depths[layer] = depth;
        if self.maxdepth < depth {
            self.maxdepth = depth;
        }
    }

    fn move_scanner(&mut self, layer: usize) {
        if self.depths[layer] == 0 {
            // Short-circuit, there's no scanner
            return;
        }
        let pos = self.scanners[layer];
        if self.directions[layer] {
            // moving "down"
            if pos == self.depths[layer] - 1 {
                // bouncing off the bottom
                self.directions[layer] = false;
                self.scanners[layer] = pos - 1;
            } else {
                self.scanners[layer] = pos + 1;
            }
        } else {
            // moving "up"
            if pos == 0 {
                // bouncing off the top
                self.directions[layer] = true;
                self.scanners[layer] = 1;
            } else {
                self.scanners[layer] = pos - 1;
            }
        }
    }

    fn run_scanners(&mut self) {
        for i in 0..self.scanners.len() {
            self.move_scanner(i);
        }
    }

    fn severity(&mut self, caught: bool) -> usize {
        let mut total = 0;
        let mut layer = 0;
        loop {
            // We entered /layer/ at this point
            if self.scanners[layer] == 0 {
                if caught {
                    total += self.depths[layer];
                } else {
                    total += layer * self.depths[layer];
                }
            }
            // Now the scanners move
            self.run_scanners();
            layer += 1;
            if layer == self.scanners.len() {
                break;
            }
        }
        total
    }

    #[allow(dead_code)]
    fn print_firewall(&self, pos: usize) {
        for depth in 0..self.maxdepth {
            for layer in 0..self.scanners.len() {
                if self.depths[layer] > depth {
                    let (open, close) = if (pos == layer) && (depth == 0) {
                        ('(', ')')
                    } else {
                        ('[', ']')
                    };
                    let scanner = if self.scanners[layer] == depth {
                        if (pos == layer) && (depth == 0) {
                            '*'
                        } else {
                            'S'
                        }
                    } else {
                        '.'
                    };
                    print!("{}{}{} ", open, scanner, close);
                } else {
                    if depth == 0 {
                        if layer == pos {
                            print!("(.) ");
                        } else {
                            print!("... ");
                        }
                    } else {
                        print!("    ");
                    }
                }
            }
            println!("");
        }
        println!("");
    }
}

fn load_instructions() -> Firewall {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Firewall::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.add_scanner(&line);
    }
    ret
}

fn load_example() -> Firewall {
    let mut ret = Firewall::new();
    ret.add_scanner_(0, 3);
    ret.add_scanner_(1, 2);
    ret.add_scanner_(4, 4);
    ret.add_scanner_(6, 4);
    ret
}

fn problem2(input: &Firewall) -> usize {
    let mut runner = input.clone();
    for delay in 0.. {
        let caught = runner.clone().severity(true);
        if caught == 0 {
            return delay;
        }
        if (delay % 100000) == 0 {
            println!("Delay {}", delay);
        }
        runner.run_scanners();
    }
    unreachable!()
}

fn main() {
    let example = load_example();
    println!("Severity of example: {}", example.clone().severity(false));

    println!("Min delay of example: {}", problem2(&example));
    let input = load_instructions();
    println!("Loaded Firewall with {} layers", input.scanners.len());
    println!("Problem 1: {}", input.clone().severity(false));
    println!("Problem 2: {}", problem2(&input));
}
