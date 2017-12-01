use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn load_words () -> Vec<String> {
    let infile = File::open("day6.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret : Vec<String> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(line);
    }
    return ret;
}

fn problem1 () -> String {
    let words = load_words();
    let mut ret : String = String::new();
    for i in 0..8 {
        let mut counts : HashMap<char, u32> = HashMap::new();
        for word in words.iter() {
            let ch : char = word.chars().nth(i).unwrap();
            let mut c = 0;
            if let Some(n) = counts.get(&ch) {
                c = *n;
            }
            counts.insert(ch, c + 1);
        }
        let mut cs : Vec<(&char, &u32)> = counts.iter().collect();
        cs.sort_by_key(|&(_,n)| *n);
        ret.push(*(cs.pop().unwrap().0));
    }
    return ret;
}

fn problem2 () -> String {
    let words = load_words();
    let mut ret : String = String::new();
    for i in 0..8 {
        let mut counts : HashMap<char, u32> = HashMap::new();
        for word in words.iter() {
            let ch : char = word.chars().nth(i).unwrap();
            let mut c = 0;
            if let Some(n) = counts.get(&ch) {
                c = *n;
            }
            counts.insert(ch, c + 1);
        }
        let mut cs : Vec<(&char, &u32)> = counts.iter().collect();
        cs.sort_by_key(|&(_,n)| words.len() - (*n as usize));
        ret.push(*(cs.pop().unwrap().0));
    }
    return ret;
}

fn main () {
    println!("Result 1: {}", problem1());
    println!("Result 2: {}", problem2());
}
