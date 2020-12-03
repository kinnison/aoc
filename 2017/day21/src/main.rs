#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

use regex::Regex;

struct Pattern {
    bits: Vec<Vec<bool>>,
}

impl Pattern {
    fn new(s: &str) -> Pattern {
        lazy_static! {
            static ref TWOBYTWO_RE: Regex = Regex::new(r"^(..)/(..)$").unwrap();
            static ref THREEBYTHREE_RE: Regex = Regex::new(r"^(...)/(...)/(...)").unwrap();
            static ref FOURBYFOUR_RE: Regex = Regex::new(r"^(....)/(....)/(....)/(....)$").unwrap();
        }
        let mut ret = Pattern { bits: Vec::new() };
        if let Some(cap) = TWOBYTWO_RE.captures(s) {
            let row1 = cap.get(1).unwrap().as_str();
            let row2 = cap.get(2).unwrap().as_str();
            ret.bits.push(row1.chars().map(|c| c == '#').collect());
            ret.bits.push(row2.chars().map(|c| c == '#').collect());
        }
        if let Some(cap) = THREEBYTHREE_RE.captures(s) {
            let row1 = cap.get(1).unwrap().as_str();
            let row2 = cap.get(2).unwrap().as_str();
            let row3 = cap.get(3).unwrap().as_str();
            ret.bits.push(row1.chars().map(|c| c == '#').collect());
            ret.bits.push(row2.chars().map(|c| c == '#').collect());
            ret.bits.push(row3.chars().map(|c| c == '#').collect());
        }
        if let Some(cap) = FOURBYFOUR_RE.captures(s) {
            let row1 = cap.get(1).unwrap().as_str();
            let row2 = cap.get(2).unwrap().as_str();
            let row3 = cap.get(3).unwrap().as_str();
            let row4 = cap.get(4).unwrap().as_str();
            ret.bits.push(row1.chars().map(|c| c == '#').collect());
            ret.bits.push(row2.chars().map(|c| c == '#').collect());
            ret.bits.push(row3.chars().map(|c| c == '#').collect());
            ret.bits.push(row4.chars().map(|c| c == '#').collect());
        }
        ret
    }

    fn flip_horz(&self) -> Pattern {
        Pattern {
            bits: self
                .bits
                .iter()
                .map(|row| row.iter().rev().copied().collect())
                .collect(),
        }
    }

    fn flip_vert(&self) -> Pattern {
        Pattern {
            bits: self.bits.iter().rev().cloned().collect(),
        }
    }

    fn rotate(&self) -> Pattern {
        Pattern {
            bits: (0..self.bits.len())
                .map(|c| (0..self.bits.len()).map(|r| self.bits[r][c]).collect())
                .collect(),
        }
    }

    fn new_all(s: &str) -> Vec<Pattern> {
        let toplevel = Pattern::new(s);
        let horzfirst = toplevel.flip_horz();
        let vertfirst = toplevel.flip_vert();
        let both = horzfirst.flip_vert();
        let onerot = toplevel.rotate();
        let threerot = both.rotate();
        vec![toplevel, horzfirst, vertfirst, both, onerot, threerot]
    }
}

struct Rule {
    width: usize,
    inputs: Vec<Pattern>,
    output: Pattern,
}

impl Rule {
    fn new(s: &str) -> Rule {
        lazy_static! {
            static ref TWOBYTWO_RE: Regex = Regex::new(r"^(../..) => (.../.../...)$").unwrap();
            static ref THREEBYTHREE_RE: Regex =
                Regex::new(r"^(.../.../...) => (..../..../..../....)$").unwrap();
        }
        if let Some(cap) = TWOBYTWO_RE.captures(s) {
            Rule {
                width: 2,
                inputs: Pattern::new_all(cap.get(1).unwrap().as_str()),
                output: Pattern::new(cap.get(2).unwrap().as_str()),
            }
        } else if let Some(cap) = THREEBYTHREE_RE.captures(s) {
            Rule {
                width: 3,
                inputs: Pattern::new_all(cap.get(1).unwrap().as_str()),
                output: Pattern::new(cap.get(2).unwrap().as_str()),
            }
        } else {
            panic!("Unable to parse rulebook entry: {:?}", s)
        }
    }
}

struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new() -> Grid {
        // Grids are always square and always either /2 or /3 because output
        // grids are always either /2 or /3
        // Grids always start with the same pattern, a GoL glider:
        // .#.
        // ..#
        // ###
        Grid {
            cells: vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true],
            ],
        }
    }

    fn grid_at(&self, row: usize, col: usize, width: usize) -> Grid {
        let mut ret = Grid { cells: Vec::new() };
        for r in row..row + width {
            let mut rowv = Vec::new();
            for c in col..col + width {
                rowv.push(self.cells[r][c]);
            }
            ret.cells.push(rowv);
        }
        ret
    }

    fn subgrids(&self) -> Vec<Vec<Grid>> {
        let mut ret = Vec::new();
        let div = if self.cells.len() % 2 == 0 { 2 } else { 3 };
        let gs: usize = self.cells.len() / div;
        for r in 0..gs {
            let mut rowv = Vec::new();
            for c in 0..gs {
                rowv.push(self.grid_at(r * div, c * div, div));
            }
            ret.push(rowv);
        }
        ret
    }

    fn matches_patt(&self, patt: &Pattern) -> bool {
        for i in 0..self.cells.len() {
            for j in 0..self.cells.len() {
                if self.cells[i][j] != patt.bits[i][j] {
                    return false;
                }
            }
        }
        true
    }

    fn matches(&self, rule: &Rule) -> bool {
        assert!(self.cells.len() == rule.width);
        for poss in &rule.inputs {
            if self.matches_patt(&poss) {
                return true;
            }
        }
        false
    }

    fn from_patt(patt: &Pattern) -> Grid {
        Grid {
            cells: patt.bits.to_vec(),
        }
    }

    fn as_rule(&self) -> String {
        let mut ret = String::new();
        for r in &self.cells {
            for c in r {
                if *c {
                    ret.push('#');
                } else {
                    ret.push('.');
                }
            }
            ret.push('/');
        }
        ret.pop();
        ret
    }

    fn empty(wid: usize) -> Grid {
        Grid {
            cells: (0..wid)
                .map(|_| (0..wid).map(|_| false).collect())
                .collect(),
        }
    }

    fn insert_at(&mut self, grid: &Grid, row: usize, col: usize) {
        for r in 0..grid.cells.len() {
            for c in 0..grid.cells.len() {
                self.cells[r + row][c + col] = grid.cells[r][c];
            }
        }
    }

    fn count_on(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|v| v.iter().filter(|&&c| c))
            .count()
    }
}

struct Rulebook {
    rules: Vec<Rule>,
}

impl Rulebook {
    fn new() -> Rulebook {
        Rulebook { rules: Vec::new() }
    }

    fn apply_rules(&self, ingrid: &Grid) -> Grid {
        // We apply the rules to convert a single pattern grid into a new one
        for rule in self.rules.iter().filter(|r| r.width == ingrid.cells.len()) {
            if ingrid.matches(rule) {
                //                println!("Performing expansion of {} by rule {} -> {}",
                //                         ingrid.as_rule(),
                //                         Grid::from_patt(&rule.inputs[0]).as_rule(),
                //                         Grid::from_patt(&rule.output).as_rule());
                return Grid::from_patt(&rule.output);
            }
        }
        panic!("Unable to find a rule for grid {}", ingrid.as_rule());
    }

    fn dump_rulebook(&self) {
        for rule in self.rules.iter() {
            let ingrid = Grid::from_patt(&rule.inputs[0]);
            let outgrid = Grid::from_patt(&rule.output);
            println!("{} => {}", ingrid.as_rule(), outgrid.as_rule());
        }
    }

    fn iter_grid(&self, ingrid: &Grid) -> Grid {
        let subs = ingrid.subgrids();
        let outwid = subs[0][0].cells.len() + 1;
        let mut ret = Grid::empty(subs.len() * outwid);
        for (r, rv) in (0..).zip(subs.iter()) {
            for (c, g) in (0..).zip(rv.iter()) {
                let intermed = self.apply_rules(&g);
                ret.insert_at(&intermed, r * outwid, c * outwid);
            }
        }
        ret
    }
}

fn load_instructions(s: &str) -> Rulebook {
    let infile = File::open(s).unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Rulebook::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.rules.push(Rule::new(&line));
    }
    ret
}

fn go_iter(input: &Rulebook, its: usize) -> usize {
    let mut grid = Grid::new();
    for _ in 0..its {
        let newgrid = input.iter_grid(&grid);
        //        println!("{} -> {}", grid.as_rule(), newgrid.as_rule());
        grid = newgrid;
    }
    grid.count_on()
}

fn problem1(input: &Rulebook) -> usize {
    go_iter(input, 5)
}

fn problem2(input: &Rulebook) -> usize {
    go_iter(input, 18)
}

fn main() {
    let example = load_instructions("example");
    println!("Loaded {} rules from example rulebook", example.rules.len());
    example.dump_rulebook();
    let testgrid = Grid::new();
    let twotick = example.iter_grid(&example.iter_grid(&testgrid));
    println!(
        "After two ticks, grid is {} and has {} on",
        twotick.as_rule(),
        twotick.count_on()
    );
    let input = load_instructions("input");
    println!("Loaded {} rules from input rulebook", input.rules.len());
    println!("Problem 1: {}", problem1(&input));
    println!("Problem 2: {}", problem2(&input));
}
