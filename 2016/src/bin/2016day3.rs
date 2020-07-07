use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Debug, Copy, Clone)]
struct Triangle {
    a: i32,
    b: i32,
    c: i32,
}

impl Triangle {
    pub fn new(t_: String) -> Triangle {
        let t = t_.trim();
        let ns: Vec<i32> = t
            .split(" ")
            .filter(|x| (x.trim() != ""))
            .map(&str::parse::<i32>)
            .map(&std::result::Result::unwrap)
            .collect();
        let a = ns[0];
        let b = ns[1];
        let c = ns[2];
        return Triangle { a: a, b: b, c: c };
    }

    fn is_valid(&self) -> bool {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        return ((a + b) > c) && ((a + c) > b) && ((b + c) > a);
    }
}

fn load_triangles() -> Vec<Triangle> {
    let infile = File::open("day3.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<Triangle> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Triangle::new(line));
    }
    return ret;
}

fn problem1() -> usize {
    let mut triangles = load_triangles();
    triangles.retain(|t| t.is_valid());
    return triangles.len();
}

fn load_triangles2() -> Vec<Triangle> {
    let infile = File::open("day3.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<Triangle> = Vec::new();
    let mut lines: Vec<String> = freader.lines().map(&std::result::Result::unwrap).collect();
    while lines.len() > 0 {
        let l1 = lines.pop().unwrap();
        let l2 = lines.pop().unwrap();
        let l3 = lines.pop().unwrap();
        /*
         * We need to take three lines of the form "  XXX  XXX  XXX  "
         * And make it so the vertical components are in that form instead
         */
        let ns1: Vec<i32> = l1
            .trim()
            .split(" ")
            .filter(|x| (x.trim() != ""))
            .map(&str::parse::<i32>)
            .map(&std::result::Result::unwrap)
            .collect();
        let ns2: Vec<i32> = l2
            .trim()
            .split(" ")
            .filter(|x| (x.trim() != ""))
            .map(&str::parse::<i32>)
            .map(&std::result::Result::unwrap)
            .collect();
        let ns3: Vec<i32> = l3
            .trim()
            .split(" ")
            .filter(|x| (x.trim() != ""))
            .map(&str::parse::<i32>)
            .map(&std::result::Result::unwrap)
            .collect();
        let t1 = format!("{} {} {}", ns1[0], ns2[0], ns3[0]);
        let t2 = format!("{} {} {}", ns1[1], ns2[1], ns3[1]);
        let t3 = format!("{} {} {}", ns1[2], ns2[2], ns3[2]);
        ret.push(Triangle::new(t1));
        ret.push(Triangle::new(t2));
        ret.push(Triangle::new(t3));
    }
    return ret;
}

fn problem2() -> usize {
    let mut triangles = load_triangles2();
    triangles.retain(|t| t.is_valid());
    return triangles.len();
}

fn main() {
    println!("Result 1: {}", problem1());
    println!("Result 2: {}", problem2());
}
