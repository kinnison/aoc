use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_ranges() -> Vec<(u32, u32)> {
    let infile = File::open("day20.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<(u32, u32)> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        let mut elems = line.split('-');
        let low: u32 = elems.next().unwrap().parse().unwrap();
        let high: u32 = elems.next().unwrap().parse().unwrap();
        ret.push((low, high));
    }
    ret.sort_by_key(|&t| t.0);
    ret
}

fn collapse_ranges_a_bit(inr: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut ret: Vec<(u32, u32)> = Vec::new();
    for &(low, high) in inr.iter() {
        // Try and find a range already in the output containing low
        let mut found: bool = false;
        let mut ofs: usize = 0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..ret.len() {
            if (ret[i].0 <= low) && (ret[i].1 >= (low - 1)) {
                found = true;
                ofs = i;
                break;
            }
        }
        if found {
            // Okay, we found something, so merge the ranges
            ret[ofs].1 = if ret[ofs].1 < high { high } else { ret[ofs].1 };
        } else {
            // Not found, so shove this range into the loop
            ret.push((low, high));
        }
    }
    ret.sort_by_key(|&t| t.0);
    ret
}

fn collapse_ranges(inr: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut cur = inr;
    loop {
        let oldlen = cur.len();
        cur = collapse_ranges_a_bit(cur);
        if cur.len() == oldlen {
            break;
        }
    }
    cur
}

fn problem1() -> u32 {
    let ranges = collapse_ranges(load_ranges());
    assert_eq!(ranges[0].0, 0);
    ranges[0].1 + 1
}

fn problem2() -> u32 {
    let ranges = collapse_ranges(load_ranges());
    let mut tot: u32 = 0; // Nothing below range 0...
    let mut pos: u32 = ranges[0].1;
    #[allow(clippy::needless_range_loop)]
    for i in 1..ranges.len() {
        tot += ranges[i].0 - pos - 1;
        pos = ranges[i].1;
    }

    tot += u32::max_value() - pos;

    tot
}

fn main() {
    println!("Problem 1: {}", problem1());
    println!("Problem 2: {}", problem2());
}
