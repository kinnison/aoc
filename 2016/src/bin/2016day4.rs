#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

#[derive(Debug, Clone)]
struct Room {
    label: String,
    name: String,
    sector: String,
    sectornum: u32,
    check: String,
}

impl Room {
    pub fn new(t_: String) -> Room {
        lazy_static! {
            static ref RE: Regex = Regex::new("^([a-z-]*)-([0-9]*)\\[(.....)\\]$").unwrap();
        }
        if !RE.is_match(&t_) {
            panic!(t_);
        }
        if let Some(cap) = RE.captures_iter(&t_).next() {
            let name = cap.get(1);
            let sector = cap.get(2);
            let check = cap.get(3);
            let sectornum: u32 = sector.unwrap().as_str().parse().unwrap();
            Room {
                label: t_.clone(),
                name: name.unwrap().as_str().to_string(),
                sector: sector.unwrap().as_str().to_string(),
                check: check.unwrap().as_str().to_string(),
                sectornum,
            }
        } else {
            panic!("Unable to construct room!")
        }
    }

    fn is_valid(&self) -> bool {
        /* Valid if top 5 chars excl. - sorted equals the check */
        let mut chs: HashMap<char, i32> = HashMap::new();
        for ch in self.name.chars() {
            if ch != '-' {
                let mut count: i32 = 0;
                if let Some(c) = chs.get(&ch) {
                    count = *c;
                }
                chs.insert(ch, count + 1);
            }
        }
        let mut chv: Vec<(char, i32)> = chs.iter().map(|(a, b)| (*a, *b)).collect();
        chv.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        if chv.len() < 5 {
            return false;
        }
        chv.truncate(5);
        let checkv: Vec<char> = self.check.chars().collect();
        let resv: Vec<char> = chv.iter().map(|&(a, _)| a).collect();
        checkv == resv
    }

    fn decrypt(&self) -> String {
        let mut ret: String = String::new();
        for ch in self.name.chars() {
            if ch == '-' {
                ret.push(' ');
            } else {
                let mut chn = ch as u32;
                chn -= b'a' as u32;
                chn += self.sectornum;
                chn %= 26;
                chn += b'a' as u32;
                ret.push((chn as u8) as char);
            }
        }

        ret
    }
}

fn load_rooms() -> Vec<Room> {
    let infile = File::open("day4.input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Vec<Room> = Vec::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.push(Room::new(line));
    }
    ret
}

fn problem1() -> u32 {
    let mut rooms = load_rooms();
    rooms.retain(|r| r.is_valid());
    return rooms.iter().fold(0, |s, e| s + e.sectornum);
}

fn problem2() -> u32 {
    let mut rooms = load_rooms();
    rooms.retain(|r| r.is_valid());
    for room in rooms.iter() {
        if room.decrypt() == "northpole object storage" {
            return room.sectornum;
        }
    }
    0
}

fn main() {
    let r = Room::new(("aaaaa-bbb-z-y-x-123[abxyz]").to_string());
    println!("Test room: {:?}", r);
    println!("Test room validity: {}", r.is_valid());
    let r = Room::new(("a-b-c-d-e-f-g-h-987[abcde]").to_string());
    println!("Test room: {:?}", r);
    println!("Test room validity: {}", r.is_valid());
    let r = Room::new(("not-a-real-room-404[oarel]").to_string());
    println!("Test room: {:?}", r);
    println!("Test room validity: {}", r.is_valid());
    let r = Room::new(("totally-real-room-200[decoy]").to_string());
    println!("Test room: {:?}", r);
    println!("Test room validity: {}", r.is_valid());
    println!("Result 1: {}", problem1());
    println!("Result 2: {}", problem2());
}
