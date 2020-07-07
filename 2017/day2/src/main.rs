use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::vec::Vec;

struct Sheet {
    cells: Vec<Vec<usize>>,
}

fn load_instructions() -> Sheet {
    let infile = File::open("input").unwrap();
    let freader = BufReader::new(&infile);
    let mut ret: Sheet = Sheet { cells: Vec::new() };
    for line_ in freader.lines() {
        let line = line_.unwrap();
        let mut row: Vec<usize> = Vec::new();
        for num_ in line.split_whitespace() {
            row.push(usize::from_str(num_).unwrap());
        }
        ret.cells.push(row);
    }
    return ret;
}

fn problem1(sheet: &Sheet) -> usize {
    let mut sum = 0;
    for row in &sheet.cells {
        let mut mincol: usize = row[0];
        let mut maxcol: usize = row[0];
        for col_ in row {
            let col: usize = *col_;
            if col < mincol {
                mincol = col
            }
            if col > maxcol {
                maxcol = col
            }
        }
        sum += maxcol - mincol;
    }
    sum
}

fn problem2(sheet: &Sheet) -> usize {
    let mut sum = 0;
    'rows: for row in &sheet.cells {
        for ai in 0..row.len() - 1 {
            for bi in 0..row.len() {
                if ai != bi {
                    let a: usize = row[ai];
                    let b: usize = row[bi];
                    if a > b {
                        if (a % b) == 0 {
                            sum += a / b;
                            continue 'rows;
                        }
                    } else {
                        if (b % a) == 0 {
                            sum += b / a;
                            continue 'rows;
                        }
                    }
                }
            }
        }
    }
    sum
}

fn main() {
    let input = load_instructions();
    println!("Answer 1 is {}", problem1(&input));
    println!("Answer 2 is {}", problem2(&input));
}
