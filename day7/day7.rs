use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Address {
    supernet : Vec<String>,
    hypernet : Vec<String>
}

fn has_abba(seg : &String) -> bool {
    let mut vseg : Vec<char> = seg.chars().collect();
    while vseg.len() > 3 {
        let ch4 = vseg.pop().unwrap();
        let ch3 = vseg[vseg.len()-1];
        let ch2 = vseg[vseg.len()-2];
        let ch1 = vseg[vseg.len()-3];
        if (ch1 == ch4) && (ch2 == ch3) && (ch1 != ch2) {
            return true;
        }
    }
    false
}

fn bab_for(ch1 : char, ch2 : char, hyps : &Vec<String>) -> bool {
    for hyp in hyps.iter() {
        let mut vhyp : Vec<char> = hyp.chars().collect();
        while vhyp.len() > 2 {
            let hch3 = vhyp.pop().unwrap();
            let hch2 = vhyp[vhyp.len()-1];
            let hch1 = vhyp[vhyp.len()-2];
            if (hch1 == ch2) && (hch3 == ch2) && (hch2 == ch1) {
                return true;
            }
        }
    }
    false
}

fn aba_bab(seg : &String, hyps : &Vec<String>) -> bool {
    let mut vseg : Vec<char> = seg.chars().collect();
    while vseg.len() > 2 {
        let ch3 = vseg.pop().unwrap();
        let ch2 = vseg[vseg.len()-1];
        let ch1 = vseg[vseg.len()-2];
        if (ch1 == ch3) && (ch1 != ch2) {
            if bab_for(ch1, ch2, hyps) {
                return true;
            }
        }
    }
    false
}

impl Address {
    fn new(addr : String) -> Address {
        let mut supernets : Vec<String> = Vec::new();
        let mut hypernets : Vec<String> = Vec::new();
        // foobar[bazzle]wibble[cheese]possible
        let mut supernet = true;
        for bit in addr.split(|c| c == '[' || c == ']') {
            if supernet {
                supernets.push(bit.to_string());
            } else {
                hypernets.push(bit.to_string());
            }
            supernet = !supernet;
        }
        Address { supernet: supernets, hypernet: hypernets }
    }

    fn does_tls(&self) -> bool {
        for hyp in self.hypernet.iter() {
            if has_abba(hyp) {
                return false;
            }
        }
        for sup in self.supernet.iter() {
            if has_abba(sup) {
                return true;
            }
        }
        false
    }

    fn does_ssl(&self) -> bool {
        for sup in self.supernet.iter() {
            if aba_bab(sup, &self.hypernet) {
                return true;
            }
        }
        false
    }
}


fn load_addresses () -> Vec<Address> {
    let mut ret : Vec<Address> = Vec::new();
    let mut f = File::open("day7.input").unwrap();
    let mut reader = BufReader::new(f);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        ret.push(Address::new(line));
    }
    return ret;
}

fn problem1 () -> usize {
    let mut addrs = load_addresses();
    addrs.retain(|a| a.does_tls());
    return addrs.len();
}

fn problem2 () -> usize {
    let mut addrs = load_addresses();
    addrs.retain(|a| a.does_ssl());
    return addrs.len();
}

fn main () {
    println!("Result 1: {}", problem1());
    println!("Result 2: {}", problem2());
}
