use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Grid {
    content: Vec<Vec<char>>,
    rowcount: usize,
    colcount: usize,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            content: Vec::new(),
            rowcount: 0,
            colcount: 0,
        }
    }

    fn normalise(&mut self) {
        let mut maxw = 0;
        for row in 0..self.content.len() {
            if self.content[row].len() > maxw {
                maxw = self.content[row].len();
            }
        }
        for row in 0..self.content.len() {
            while self.content[row].len() < maxw {
                self.content[row].push(' ');
            }
        }
        self.colcount = maxw;
        self.rowcount = self.content.len();
    }

    fn next_pos(&self, row: usize, col: usize, dir: Dir) -> Option<(usize, usize, Dir)> {
        // None if can't continue, otherwise new row/col
        let curch = self.content[row][col];
        let mut nextrow = row;
        let mut nextcol = col;
        let mut nextdir = dir;
        match dir {
            Dir::Up | Dir::Down => {
                if curch == '+' {
                    // Look left/right
                    if col > 0 && self.content[row][col - 1] != ' ' {
                        nextcol = col - 1;
                        nextdir = Dir::Left;
                    } else if col < (self.colcount - 1) && self.content[row][col + 1] != ' ' {
                        nextcol = col + 1;
                        nextdir = Dir::Right;
                    } else {
                        // We're at a + and we can't go left or right
                        return None;
                    }
                } else if dir == Dir::Up && row > 0 {
                    nextrow = row - 1;
                } else if dir == Dir::Down && row < (self.rowcount - 1) {
                    nextrow = row + 1;
                } else {
                    // Can't go up/down, done.
                    return None;
                }
            }
            Dir::Left | Dir::Right => {
                if curch == '+' {
                    // Look Up/Down
                    if row > 0 && self.content[row - 1][col] != ' ' {
                        nextrow = row - 1;
                        nextdir = Dir::Up;
                    } else if row < (self.rowcount - 1) && self.content[row + 1][col] != ' ' {
                        nextrow = row + 1;
                        nextdir = Dir::Down;
                    } else {
                        // We're at a + and we can't go up or down
                        return None;
                    }
                } else if dir == Dir::Left && col > 0 {
                    nextcol = col - 1;
                } else if dir == Dir::Right && col < (self.colcount - 1) {
                    nextcol = col + 1;
                } else {
                    // Can't go left/right, done.
                    return None;
                }
            }
        }
        assert!((row != nextrow) || (col != nextcol));
        if self.content[nextrow][nextcol] == ' ' {
            None
        } else {
            Some((nextrow, nextcol, nextdir))
        }
    }

    fn problem(&self, tell: bool) -> (String, usize) {
        let mut ret = String::new();
        let mut currow = 0;
        let mut curcol = 0;
        // Scan for input point
        for i in 0..self.colcount {
            if self.content[0][i] == '|' {
                curcol = i;
            }
        }
        let mut curdir = Dir::Down;
        let mut steps = 1;
        while let Some((nextrow, nextcol, nextdir)) = self.next_pos(currow, curcol, curdir) {
            let curch = self.content[currow][curcol];
            if tell {
                println!("Output currently {:?}", ret);
                println!(
                    "At row={}, col={}, dir={:?}, ch={}",
                    currow, curcol, curdir, curch
                );
                println!(
                    " => row={}, col={}, dir={:?}, ch={}",
                    nextrow, nextcol, nextdir, self.content[nextrow][nextcol]
                );
            }
            if (curch as u8) >= b'A' && (curch as u8) <= b'Z' {
                ret.push(curch);
            }
            currow = nextrow;
            curcol = nextcol;
            curdir = nextdir;
            steps += 1;
        }
        let curch = self.content[currow][curcol];
        if (curch as u8) >= b'A' && (curch as u8) <= b'Z' {
            ret.push(curch);
        }
        if tell {
            println!("finished!");
        }
        (ret, steps)
    }
}

fn load_instructions(s: &str) -> Grid {
    let infile = File::open(s).unwrap();
    let freader = BufReader::new(&infile);
    let mut ret = Grid::new();
    for line_ in freader.lines() {
        let line = line_.unwrap();
        ret.content.push(line.chars().collect());
    }
    ret.normalise();
    ret
}

fn main() {
    let example = load_instructions("example");
    println!(
        "Loaded example which is {}x{}",
        example.rowcount, example.colcount
    );
    println!("Puzzle for example: {:?}", example.problem(true));
    let input = load_instructions("input");
    println!(
        "Loaded input which is {}x{}",
        input.rowcount, input.colcount
    );
    println!("Puzzle for input: {:?}", input.problem(false));
}
