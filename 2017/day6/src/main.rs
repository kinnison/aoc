use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

fn load_instructions() -> Vec<usize> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        for elem in line.split_whitespace() {
            ret.push(elem.parse().unwrap());
        }
    }
    ret
}

#[allow(clippy::ptr_arg, clippy::needless_range_loop)]
fn problem1(input_: &Vec<usize>) -> usize {
    let mut seen: HashSet<Vec<usize>> = HashSet::new();
    let mut cur = input_.clone();
    let mut redists = 0;
    loop {
        let mut biggest_idx = 0;
        let mut biggest_size = cur[0];
        for i in 1..cur.len() {
            if cur[i] > biggest_size {
                biggest_size = cur[i];
                biggest_idx = i;
            }
        }
        // Biggest selected, extract it
        let mut redist = biggest_size;
        cur[biggest_idx] = 0;
        while redist > 0 {
            biggest_idx = (biggest_idx + 1) % cur.len();
            cur[biggest_idx] += 1;
            redist -= 1;
        }
        // New state created
        redists += 1;
        if seen.contains(&cur) {
            break;
        }
        seen.insert(cur.clone());
    }
    redists
}

#[allow(clippy::ptr_arg, clippy::needless_range_loop)]
fn problem2(input_: &Vec<usize>) -> usize {
    let mut seen: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut cur = input_.clone();
    let mut redists = 0;
    loop {
        let mut biggest_idx = 0;
        let mut biggest_size = cur[0];
        for i in 1..cur.len() {
            if cur[i] > biggest_size {
                biggest_size = cur[i];
                biggest_idx = i;
            }
        }
        // Biggest selected, extract it
        let mut redist = biggest_size;
        cur[biggest_idx] = 0;
        while redist > 0 {
            biggest_idx = (biggest_idx + 1) % cur.len();
            cur[biggest_idx] += 1;
            redist -= 1;
        }
        // New state created
        redists += 1;
        if seen.contains_key(&cur) {
            break;
        }
        seen.insert(cur.clone(), redists);
    }
    // at this point, we know the current cycle count (redists) and
    // how many redists it has been when we met (seen.get(&cur).unwrap())
    redists - seen.get(&cur).unwrap()
}

fn main() {
    let input = load_instructions();
    println!("Loaded {} elements", input.len());
    let test = vec![0, 2, 7, 0];
    println!("Test vector has {} redists", problem1(&test));
    println!("Problem1: {}", problem1(&input));
    println!("Test vector 2 has {} cycles", problem2(&test));
    println!("Problem2: {}", problem2(&input));
}
