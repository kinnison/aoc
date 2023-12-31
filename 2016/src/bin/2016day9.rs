use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn load_compressed() -> String {
    let infile = File::open("day9.input").unwrap();
    let freader = BufReader::new(&infile);
    freader.lines().next().unwrap().unwrap()
}

fn len_compressed(compr_: &str, recurse: bool) -> usize {
    let mut ret: usize = 0;
    let mut compr = compr_.to_string();
    let mut oparen = compr.find('(');
    while oparen.is_some() {
        let mut pfx = String::new();
        let mut ch = compr.remove(0);
        while ch != '(' {
            pfx.push(ch);
            ch = compr.remove(0);
        }
        ch = compr.remove(0);
        ret += pfx.len();
        let mut numof_ = String::new();
        let mut times_ = String::new();
        while ch != 'x' {
            numof_.push(ch);
            ch = compr.remove(0);
        }
        ch = compr.remove(0);
        while ch != ')' {
            times_.push(ch);
            ch = compr.remove(0);
        }
        let numof: usize = numof_.parse().unwrap();
        let times: usize = times_.parse().unwrap();
        let mut chunk = String::new();
        for _i in 0..numof {
            chunk.push(compr.remove(0));
        }
        if recurse {
            let comprlen = len_compressed(&chunk, recurse);
            ret += comprlen * times;
        } else {
            for _i in 0..times {
                ret += chunk.len();
            }
        }
        oparen = compr.find('(');
    }
    ret += compr.len();
    ret
}

fn problem1() -> usize {
    let compr = load_compressed();
    len_compressed(&compr, false)
}

fn problem2() -> usize {
    let compr = load_compressed();
    len_compressed(&compr, true)
}

fn main() {
    println!("Result 1: {}", problem1());
    println!("Result 2: {}", problem2());
}
