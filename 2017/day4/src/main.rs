use std::collections::HashSet;
use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;

extern crate permutohedron;
use permutohedron::Heap;

struct Passphrase {
    words: Vec<String>
}

impl Passphrase {
    fn from_line (line: String) -> Passphrase {
        Passphrase {
            words: line.split_whitespace().map(|s|s.to_owned()).collect()
        }
    }

    fn is_valid1 (&self) -> bool {
        let mut cont = HashSet::new();
        for word in &self.words {
            if cont.contains(word) {
                return false;
            } else {
                cont.insert(word);
            }
        }
        true
    }

    fn is_valid2 (&self) -> bool {
        if !self.is_valid1() { return false; }
        for wi in 0..self.words.len() - 1 {
            let mut inword: Vec<char> = self.words[wi].chars().collect();
            let heap = Heap::new(&mut inword);
            for permut in heap {
                let word: String = permut.iter().collect();
                for wj in wi+1..self.words.len() {
                    if self.words[wj] == word {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn load_instructions () -> Vec<Passphrase> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Passphrase::from_line(line));
    }
    return ret;
}

fn problem1 (input: &Vec<Passphrase>) -> usize {
    input.iter().filter(|c| c.is_valid1()).count()
}

fn problem2 (input: &Vec<Passphrase>) -> usize {
    input.iter().filter(|c| c.is_valid2()).count()
}

fn main() {
    let phrases = load_instructions();
    println!("loaded {} passphrases", phrases.len());
    println!("Problem 1: {}", problem1(&phrases));
    println!("Problem 2: {}", problem2(&phrases));
}
