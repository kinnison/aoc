use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

fn load_instructions() -> Vec<u32> {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<u32> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        for digit in line.chars() {
            ret.push(digit.to_digit(10).unwrap());
        }
    }
    return ret;
}

fn problem1(input: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] {
            sum += input[i];
        }
    }
    if input[0] == input[input.len() - 1] {
        sum += input[0];
    }
    sum
}

fn problem2(input: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    let skip = input.len() >> 1;
    let l = input.len();

    for i in 0..input.len() {
        if input[i] == input[(i + skip) % l] {
            sum += input[i];
        }
    }
    sum
}

fn main() {
    let input = load_instructions();
    println!("Answer 1: {}", problem1(&input));
    println!("Answer 2: {}", problem2(&input));
}
