extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

struct Doors {
    up: bool,
    down: bool,
    left: bool,
    right: bool
}

fn possible_doors(passcode: &str, route: &str) -> Doors {
    let mut hash = Md5::new();
    hash.reset();
    hash.input(passcode.as_bytes());
    hash.input(route.as_bytes());
    let mut output = [0; 16];
    hash.result(&mut output);
    Doors {
        up: (output[0] & 0xf0) > 0xa0,
        down: (output[0] & 0x0f) > 0x0a,
        left: (output[1] & 0xf0) > 0xa0,
        right: (output[1] & 0x0f) > 0x0a,
    }
}

fn filter_doors(posx: usize, posy: usize, doors: &mut Doors) {
    if posx == 1 { doors.left = false; }
    if posx == 4 { doors.right = false; }
    if posy == 1 { doors.up = false; }
    if posy == 4 { doors.down = false; }
}

fn find_route(passcode: &str) -> String {
    let mut routes: Vec<(String, usize, usize)> = Vec::new();
    routes.push(("".to_string(), 1, 1));
    loop {
        let mut newroutes: Vec<(String, usize, usize)> = Vec::new();
        for (oldroute, posx, posy) in routes.drain(..) {
            if (posx == 4) && (posy == 4) { return oldroute; }
            let mut doors: Doors = possible_doors(passcode, &oldroute);
            filter_doors(posx, posy, &mut doors);
            if doors.up {
                newroutes.push((format!("{}U", oldroute), posx, posy - 1));
            }
            if doors.down {
                newroutes.push((format!("{}D", oldroute), posx, posy + 1));
            }
            if doors.left {
                newroutes.push((format!("{}L", oldroute), posx - 1, posy));
            }
            if doors.right {
                newroutes.push((format!("{}R", oldroute), posx + 1, posy));
            }
        }
        routes = newroutes;
    }
}

fn find_longest_route(passcode: &str) -> usize {
    let mut routes: Vec<(String, usize, usize)> = Vec::new();
    let mut found: Vec<String> = Vec::new();
    routes.push(("".to_string(), 1, 1));
    while routes.len() > 0 {
        let mut newroutes: Vec<(String, usize, usize)> = Vec::new();
        for (oldroute, posx, posy) in routes.drain(..) {
            if (posx == 4) && (posy == 4) {
                found.push(oldroute);
                continue;
            }
            let mut doors: Doors = possible_doors(passcode, &oldroute);
            filter_doors(posx, posy, &mut doors);
            if doors.up {
                newroutes.push((format!("{}U", oldroute), posx, posy - 1));
            }
            if doors.down {
                newroutes.push((format!("{}D", oldroute), posx, posy + 1));
            }
            if doors.left {
                newroutes.push((format!("{}L", oldroute), posx - 1, posy));
            }
            if doors.right {
                newroutes.push((format!("{}R", oldroute), posx + 1, posy));
            }
        }
        routes = newroutes;
    }
    found.sort_by_key(String::len);
    found.pop().unwrap().len()
}

fn main() {
    println!("Problem 1: {}", find_route("lpvhkcbi"));
    println!("Problem 2: {}", find_longest_route("lpvhkcbi"));
}
