extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::collections::HashMap;

struct Nibbles<'a> {
    h: &'a [u8; 16],
    l: usize
}

impl<'a> Iterator for Nibbles<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.l == 32 {
            None
        } else {
            self.l += 1;
            if (self.l & 1) == 1 {
                // Upper nibble of div2
                Some(self.h[self.l>>1] >> 4)
            } else {
                // Lower nibble of -1div2
                Some(self.h[(self.l-1)>>1] & 0xf)
            }
        }
    }
}

impl<'a> Nibbles<'a> {
    fn new (h: &[u8; 16]) -> Nibbles {
        Nibbles { h: h, l: 0 }
    }
        
}

fn is_triple(h: &[u8; 16]) -> Option<u8> {
    let mut count = 1;
    let mut val: u8 = 0x10;
    for nibble in Nibbles::new(h) {
        if val == nibble {
            count += 1;
            if count == 3 { return Some(nibble); }
        } else {
            count = 1;
            val = nibble;
        }
    }
    None
}

fn has_five(h: &[u8; 16], val: u8) -> bool {
    let mut count = 0;
    for nibble in Nibbles::new(h) {
        if val == nibble {
            count += 1;
            if count == 5 { return true; }
        } else {
            count = 0;
        }
    }
    false
}

fn problem1(label : &str) -> usize {
    let mut hash = Md5::new();
    let mut keys = 0;
    let mut idx : u64 = 0;
    let chs = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
    'outer: loop {
        hash.reset();
        hash.input(label.as_bytes());
        hash.input(idx.to_string().as_bytes());
        let mut output = [0; 16];
        hash.result(&mut output);
        match is_triple(&output) {
            None => {},
            Some(val) => {
                for nidx in idx..(idx+1000) {
                    hash.reset();
                    hash.input(label.as_bytes());
                    hash.input((nidx+1).to_string().as_bytes());
                    hash.result(&mut output);
                    if has_five(&output, val) {
                        keys += 1;
//                        println!("Key {} found at {}, for {}", keys, idx, chs[val as usize]);
                        if keys == 64 {
                            return idx as usize;
                        }
                    }
                }
            }
        }
        idx += 1;
    }
    0
}

fn stretched_result(hash: &mut Md5, output: &mut [u8; 16], n: usize, cache: &mut HashMap<usize,[u8;16]>) {
    // Input should be hash prepared with appropriate input
    // We will then stretch the hash 2016 times...
    let chs = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
    match cache.get(&n) {
        None => {},
        Some(arr) => {
            for i in 0..16 {
                output[i] = arr[i];
            }
            return;
        }
    }
    for _ in 0..2016 {
        hash.result(output);
        hash.reset();
        for nibble in Nibbles::new(&output) {
            hash.input(&[chs[nibble as usize] as u8]);
        }
    }
    hash.result(output);
    cache.insert(n, *output);
}

fn problem2(label : &str) -> usize {
    let mut hash = Md5::new();
    let mut keys = 0;
    let mut idx : u64 = 0;
    let mut cache : HashMap<usize,[u8;16]> = HashMap::new();
    let chs = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'];
    'outer: loop {
        hash.reset();
        hash.input(label.as_bytes());
        hash.input(idx.to_string().as_bytes());
        let mut output = [0; 16];
        stretched_result(&mut hash, &mut output, idx as usize, &mut cache);
        match is_triple(&output) {
            None => {},
            Some(val) => {
                for nidx in idx..(idx+1000) {
                    hash.reset();
                    hash.input(label.as_bytes());
                    hash.input((nidx+1).to_string().as_bytes());
                    stretched_result(&mut hash, &mut output, (nidx+1) as usize, &mut cache);
                    if has_five(&output, val) {
                        keys += 1;
//                        println!("Key {} found at {}, for {}", keys, idx, chs[val as usize]);
                        if keys == 64 {
                            return idx as usize;
                        }
                    }
                }
            }
        }
        idx += 1;
    }
    0
}

fn main () {
    let puzzleinput = "ihaygndm";
//    println!("Test result: {}", problem1("abc"));
    println!("Result 1 is {}", problem1(&puzzleinput));
//    println!("Test result: {}", problem2("abc"));
    println!("Result 2 is {}", problem2(&puzzleinput));
}
