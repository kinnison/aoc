use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
enum St {
    C,
    W,
    I,
    F,
}

#[derive(Debug, Clone)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn left(&self) -> Dir {
        match *self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }

    fn right(&self) -> Dir {
        match *self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
        }
    }

    fn rev(&self) -> Dir {
        match *self {
            Dir::U => Dir::D,
            Dir::D => Dir::U,
            Dir::L => Dir::R,
            Dir::R => Dir::L,
        }
    }
}

#[derive(Clone)]
struct Grid {
    infected: HashMap<(i32, i32), St>,
    workerx: i32,
    workery: i32,
    dir: Dir,
}

impl Grid {
    fn new(lines: Vec<String>) -> Grid {
        let midx: i32 = ((lines[0].len() + 1) >> 1) as i32;
        let midy: i32 = ((lines.len() + 1) >> 1) as i32;
        let mut ret: Grid = Grid {
            infected: HashMap::new(),
            workerx: 0,
            workery: 0,
            dir: Dir::U,
        };
        for (rown, row) in (1..).zip(lines.iter()) {
            for (coln, ch) in (1..).zip(row.chars()) {
                if ch == '#' {
                    ret.infected.insert(((coln - midx), (rown - midy)), St::I);
                }
            }
        }
        ret
    }

    fn burst1(&mut self) -> bool {
        let workerpos = (self.workerx, self.workery);
        let curinfected = self.infected.contains_key(&workerpos);
        let newdir = if curinfected {
            self.infected.remove(&workerpos);
            self.dir.right()
        } else {
            self.infected.insert(workerpos, St::I);
            self.dir.left()
        };
        match newdir {
            Dir::U => self.workery -= 1,
            Dir::D => self.workery += 1,
            Dir::L => self.workerx -= 1,
            Dir::R => self.workerx += 1,
        }
        self.dir = newdir;
        // Return if we chose to infect the cell
        !curinfected
    }

    fn burst2(&mut self) -> bool {
        let workerpos = (self.workerx, self.workery);
        let curstate = self.infected.get(&workerpos).unwrap_or(&St::C).clone();
        let newdir = match curstate {
            St::C => {
                // Clean cells are weakened, we turn left
                self.infected.insert(workerpos, St::W);
                self.dir.left()
            }
            St::W => {
                // Weakened cells are infected, we do not change direction
                self.infected.insert(workerpos, St::I);
                self.dir.clone()
            }
            St::I => {
                // Infected cells are flagged, we turn right
                self.infected.insert(workerpos, St::F);
                self.dir.right()
            }
            St::F => {
                // Flagged cells are cleaned, we reverse direction
                self.infected.remove(&workerpos);
                self.dir.rev()
            }
        };
        match newdir {
            Dir::U => self.workery -= 1,
            Dir::D => self.workery += 1,
            Dir::L => self.workerx -= 1,
            Dir::R => self.workerx += 1,
        }
        self.dir = newdir;
        // Return if we chose to infect the cell
        curstate == St::W // We infect weakened cells
    }
}

fn load_instructions(s: &str) -> Grid {
    let infile = File::open(s).unwrap();
    let freader = BufReader::new(&infile);
    Grid::new(freader.lines().map(|l| l.unwrap()).collect())
}

fn problem1(g: &Grid, n: usize) -> usize {
    let mut grid = g.clone();
    let mut count = 0;
    for _ in 0..n {
        if grid.burst1() {
            count += 1;
        }
    }
    count
}

fn problem2(g: &Grid, n: usize) -> usize {
    let mut grid = g.clone();
    let mut count = 0;
    for i in 0..n {
        if grid.burst2() {
            count += 1;
        }
    }
    count
}

fn main() {
    let example = load_instructions("example");
    assert!(problem1(&example, 7) == 5);
    assert!(problem1(&example, 70) == 41);
    assert!(problem1(&example, 10_000) == 5587);
    assert!(problem2(&example, 100) == 26);
    //    assert!(problem2(&example, 10_000_000) == 2_511_944);
    let input = load_instructions("input");
    println!("Problem 1: {}", problem1(&input, 10_000));
    println!("Problem 2: {}", problem2(&input, 10_000_000));
}
