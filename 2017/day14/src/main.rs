#[macro_use]
extern crate itertools;

use std::collections::HashSet;
use std::vec::Vec;

fn instr_from_str(s: &str) -> Vec<usize> {
    let mut ret = Vec::new();
    for ch in s.chars() {
        ret.push(ch as usize);
    }
    // 17, 31, 73, 47, 23
    ret.push(17);
    ret.push(31);
    ret.push(73);
    ret.push(47);
    ret.push(23);
    ret
}

struct KnotHash {
    size: usize,
    entries: Vec<usize>,
    curpos: usize,
    skip: usize,
}

impl KnotHash {
    fn new(size: usize) -> KnotHash {
        let mut ret = KnotHash {
            size,
            entries: Vec::new(),
            curpos: 0,
            skip: 0,
        };
        for v in 0..size {
            ret.entries.push(v);
        }
        ret
    }

    fn print(&self) {
        for i in 0..self.size {
            if self.curpos == i {
                print!("[{}] ", self.entries[i]);
            } else {
                print!("{} ", self.entries[i]);
            }
        }
        println!("  S={}", self.skip);
    }

    fn run_instruction(&mut self, len: usize) {
        let revvec: Vec<usize> = (0..len)
            .rev()
            .map(|v| (self.curpos + v) % self.size)
            .map(|p| self.entries[p])
            .collect();
        #[allow(clippy::needless_range_loop)]
        for p in 0..len {
            self.entries[(self.curpos + p) % self.size] = revvec[p];
        }
        self.curpos = (self.curpos + len + self.skip) % self.size;
        self.skip += 1;
    }

    fn run_prog(&mut self, prog: &[usize], printing: bool) {
        for elem in prog {
            self.run_instruction(*elem);
            if printing {
                self.print();
            }
        }
    }

    #[allow(dead_code)]
    fn check_value(&self) -> usize {
        self.entries[0] * self.entries[1]
    }

    fn run_rounds(&mut self, prog: &[usize]) {
        for _i in 0..64 {
            self.run_prog(prog, false);
        }
    }

    #[allow(dead_code)]
    fn dense_hash(&self) -> String {
        let mut ret = String::new();
        for base in 0..16 {
            let mut val = 0;
            for sub in 0..16 {
                val ^= self.entries[(base * 16) + sub];
            }
            ret.push_str(&format!("{:02x}", val));
        }
        ret
    }

    fn dense_bits(&self) -> Vec<bool> {
        let mut ret = Vec::new();
        for base in 0..16 {
            let mut val = 0;
            for sub in 0..16 {
                val ^= self.entries[(base * 16) + sub];
            }
            ret.push((val & 0x80) != 0);
            ret.push((val & 0x40) != 0);
            ret.push((val & 0x20) != 0);
            ret.push((val & 0x10) != 0);
            ret.push((val & 0x08) != 0);
            ret.push((val & 0x04) != 0);
            ret.push((val & 0x02) != 0);
            ret.push((val & 0x01) != 0);
        }
        ret
    }
}

struct Disk {
    contents: Vec<Vec<bool>>,
}

impl Disk {
    fn new(seed: &str) -> Disk {
        let mut ret = Disk {
            contents: Vec::new(),
        };
        for i in 0..128 {
            let rowseed = format!("{}-{}", seed, i);
            let mut hash = KnotHash::new(256);
            hash.run_rounds(&instr_from_str(&rowseed));
            let bits = hash.dense_bits();
            assert!(bits.len() == 128);
            ret.contents.push(bits);
        }
        ret
    }

    fn occupancy(&self) -> usize {
        let mut total = 0;
        for row in &self.contents {
            for col in row {
                if *col {
                    total += 1;
                }
            }
        }
        total
    }

    fn bit_at(&self, row: usize, col: usize) -> bool {
        self.contents[row][col]
    }

    fn count_groups(&self) -> usize {
        let mut coords: HashSet<(usize, usize)> = iproduct!(0..128, 0..128).collect();
        let mut groups = 0;
        while !coords.is_empty() {
            let mut consider: HashSet<(usize, usize)> = HashSet::new();
            let coord: (usize, usize) = *coords.iter().next().unwrap();
            coords.remove(&coord);
            if !self.bit_at(coord.0, coord.1) {
                continue;
            }
            groups += 1;
            consider.insert(coord);
            while !consider.is_empty() {
                let ponder: (usize, usize) = *consider.iter().next().unwrap();
                consider.remove(&ponder);
                for (rowofs, colofs) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if (ponder.0 as i32) >= -rowofs && (ponder.1 as i32) >= -colofs {
                        let newc: (usize, usize) = (
                            ((ponder.0 as i32) + rowofs) as usize,
                            ((ponder.1 as i32) + colofs) as usize,
                        );
                        if coords.contains(&newc) {
                            coords.remove(&newc);
                            if self.bit_at(newc.0, newc.1) {
                                consider.insert(newc);
                            }
                        }
                    }
                }
            }
        }

        groups
    }
}

fn main() {
    let input = "stpzcrnm";
    let testdisk = Disk::new("flqrgnkx");
    println!("Constructed test disk");
    let testocc = testdisk.occupancy();
    println!("For input 'flqrgnkx' occupancy is {}", testocc);
    assert!(testocc == 8108);
    let testgrps = testdisk.count_groups();
    println!("For input 'flqrgnkx' group count is {}", testgrps);
    assert!(testgrps == 1242);
    let realdisk = Disk::new(input);
    println!("Constructed real disk");
    println!("Problem 1: {}", realdisk.occupancy());
    println!("Problem 2: {}", realdisk.count_groups());
}
