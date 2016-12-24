use std::fs::File;
use std::vec::Vec;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
struct Maze {
    // true if open, false if wall
    cells: Vec<Vec<bool>>,
    digits: Vec<(usize, usize)>,
    paths: Vec<Vec<Option<usize>>>,
}

impl Maze {
    fn new() -> Maze {
        Maze {
            cells: Vec::new(),
            digits: Vec::new(),
            paths: Vec::new(),
        }
    }

    fn add_line(&mut self, line: String) {
        let mut newcells: Vec<bool> = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                newcells.push(false);
            } else {
                newcells.push(true);
                if ch != '.' {
                    let val: usize = ((ch as u8) - ('0' as u8)) as usize;
                    while self.digits.len() <= val {
                        self.digits.push((0, 0));
                    }
                    self.digits[val] = (x, self.cells.len());
                }
            }
        }
        self.cells.push(newcells);
    }

    fn spot_open(&self, x: usize, y: usize) -> bool {
        self.cells[y][x]
    }

    fn exits(&self, pos: (usize, usize), lst: &mut HashSet<(usize, usize)>) {
        if self.spot_open(pos.0 - 1, pos.1) {
            lst.insert((pos.0 - 1, pos.1));
        }
        if self.spot_open(pos.0 + 1, pos.1) {
            lst.insert((pos.0 + 1, pos.1));
        }
        if self.spot_open(pos.0, pos.1 - 1) {
            lst.insert((pos.0, pos.1 - 1));
        }
        if self.spot_open(pos.0, pos.1 + 1) {
            lst.insert((pos.0, pos.1 + 1));
        }
    }

    fn path_len(&self, from: (usize, usize), to: (usize, usize)) -> usize {
        // Calculate the length of the path between from and to in steps.
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut next: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(from);
        self.exits(from, &mut next);
        let mut depth = 0;
        loop {
            depth += 1;
            let mut oldset = next;
            next = HashSet::new();
            for pos in oldset.drain() {
                if pos == to {
                    return depth;
                }
                if visited.contains(&pos) {
                    // If we have already visited this location, stop
                    continue;
                }
                // Something new for us \o/
                visited.insert(pos);
                self.exits(pos, &mut next);
            }
        }
    }

    fn calc_paths(&mut self) {
        for i in 0..self.digits.len() {
            let mut mrow: Vec<Option<usize>> = Vec::new();
            for j in 0..self.digits.len() {
                if i == j {
                    mrow.push(None)
                } else {
                    mrow.push(Some(self.path_len(self.digits[i], self.digits[j])));
                }
            }
            self.paths.push(mrow);
        }
    }

    fn shortest_route(&self, ret0: bool) -> usize {
        let mut visited = HashSet::new();
        visited.insert(0);
        self.shortest_route_(0, ret0, &mut visited)
    }

    fn shortest_route_(&self, at: usize, ret0: bool, mut visited: &mut HashSet<usize>) -> usize {
        // We need to visit each digit we've not yet visited, calculating
        // the length and trying it...
        let mut bestlen: usize = usize::max_value();
        if self.digits.len() == visited.len() {
            // Can't get anywhere, zero to go
            if ret0 {
                // We need to return to zero, what's the cost from here?
                return self.paths[at][0].unwrap();
            } else {
                return 0;
            }
        }
        for i in 0..self.digits.len() {
            // Skip the digit if we've visited it already
            if visited.contains(&i) {
                continue;
            }
            if i == at {
                continue;
            }
            let steplen = self.paths[at][i].unwrap();
            visited.insert(i);
            let thislen = self.shortest_route_(i, ret0, &mut visited);
            visited.remove(&i);
            if (thislen + steplen) < bestlen {
                bestlen = thislen + steplen;
            }
        }
        bestlen
    }
}

fn load_maze(pth: &str) -> Maze {
    let infile = File::open(pth).unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Maze = Maze::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.add_line(line);
    }
    ret.calc_paths();
    ret
}

fn test() {
    let maze = load_maze("test.input");
    println!("test maze shortest route is {} long",
             maze.shortest_route(false));
}

fn problem1() -> usize {
    let maze = load_maze("day24.input");
    maze.shortest_route(false)
}

fn problem2() -> usize {
    let maze = load_maze("day24.input");
    maze.shortest_route(true)
}

fn main() {
    test();
    println!("Problem 1: {}", problem1());
    println!("Problem 2: {}", problem2());
}
