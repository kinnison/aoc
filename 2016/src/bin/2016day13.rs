use std::collections::HashSet;
use std::vec::Vec;

static PUZZLE_INPUT: usize = 1350;

fn is_open(x: usize, y: usize) -> bool {
    let mut locn = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + PUZZLE_INPUT;
    let mut count = 0;
    while locn != 0 {
        locn = locn & (locn - 1);
        count += 1;
    }
    return (count & 1) == 0;
}

fn maybe_next(
    posx: usize,
    posy: usize,
    branches: &mut Vec<(usize, usize)>,
    visited: &HashSet<(usize, usize)>,
) {
    let coord = (posx, posy);
    if is_open(posx, posy) && !visited.contains(&coord) {
        branches.push(coord);
    }
}

fn push_nexts(
    posx: usize,
    posy: usize,
    branches: &mut Vec<(usize, usize)>,
    visited: &HashSet<(usize, usize)>,
) {
    if posx > 0 {
        maybe_next(posx - 1, posy, branches, visited);
    }
    if posy > 0 {
        maybe_next(posx, posy - 1, branches, visited);
    }
    maybe_next(posx + 1, posy, branches, visited);
    maybe_next(posx, posy + 1, branches, visited);
}

fn path_len(goalx: usize, goaly: usize) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut branches: Vec<(usize, usize)> = Vec::new();
    let mut depth: usize = 0;
    visited.insert((1, 1));
    push_nexts(1, 1, &mut branches, &visited);
    'outer: loop {
        assert!(branches.len() > 0);
        depth += 1;
        let mut newbranches: Vec<(usize, usize)> = Vec::new();
        for (newx, newy) in branches.drain(..) {
            if newx == goalx && newy == goaly {
                break 'outer;
            }
            visited.insert((newx, newy));
            push_nexts(newx, newy, &mut newbranches, &visited);
        }
        branches = newbranches;
    }
    depth
}

fn count_locs(maxn: usize) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut branches: Vec<(usize, usize)> = Vec::new();
    let _depth: usize = 0;
    visited.insert((1, 1));
    push_nexts(1, 1, &mut branches, &visited);
    for _ in 0..maxn {
        assert!(branches.len() > 0);
        let mut newbranches: Vec<(usize, usize)> = Vec::new();
        for (newx, newy) in branches.drain(..) {
            visited.insert((newx, newy));
            push_nexts(newx, newy, &mut newbranches, &visited);
        }
        branches = newbranches;
    }
    visited.len()
}

fn problem1() -> usize {
    path_len(31, 39)
}

fn problem2() -> usize {
    count_locs(50)
}

fn main() {
    println!("Result 1: {}", problem1());
    println!("Result 2: {}", problem2());
}
