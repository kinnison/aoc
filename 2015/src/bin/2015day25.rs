use aoc2015::*;

struct PosGen {
    row: usize,
    col: usize,
    last: usize,
}

impl PosGen {
    fn new() -> PosGen {
        PosGen {
            row: 2,
            col: 0,
            last: 1,
        }
    }
}

impl Iterator for PosGen {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.row == 1 {
            // Reset to col1 on the next row down
            self.last += 1;
            self.row = self.last;
            self.col = 1;
        } else {
            self.col += 1;
            self.row -= 1;
        }
        Some((self.row, self.col))
    }
}

struct ValueGen {
    value: u128,
    gen: PosGen,
}

impl ValueGen {
    fn new() -> ValueGen {
        ValueGen {
            value: 20151125,
            gen: PosGen::new(),
        }
    }

    fn next(&mut self) -> ((usize, usize), u128) {
        let pos = self.gen.next().unwrap();
        let ret = (pos, self.value);
        self.value = (self.value * 252533) % 33554393;
        ret
    }
}

fn main() -> Result<()> {
    let input: Vec<usize> = read_input(25)?
        .split_whitespace()
        .map(|s| s[0..s.len() - 1].to_owned())
        .map(|v| v.parse())
        .filter(|v| v.is_ok())
        .map(|r| r.unwrap())
        .collect();
    assert!(input.len() == 2);
    let input_row = input[0];
    let input_col = input[1];
    println!("Code at row {} col {}", input_row, input_col);
    let mut gen = ValueGen::new();
    loop {
        let ((row, col), val) = gen.next();
        if (row == input_row) && (col == input_col) {
            println!("Value: {}", val);
            break;
        }
    }
    Ok(())
}
