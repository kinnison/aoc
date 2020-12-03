use aoc2015::*;

#[derive(Clone)]
struct Grid {
    bits: Vec<bool>,
    width: i32,
    height: i32,
}

impl Grid {
    fn from_str(input: &str) -> Grid {
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().len() as i32;
        let mut ret = Grid {
            bits: Vec::new(),
            width,
            height,
        };
        ret.bits.resize((height * width) as usize, false);
        for (row, s) in input.lines().enumerate() {
            for (col, ch) in s.chars().enumerate() {
                if ch == '#' {
                    ret.set(row as i32, col as i32, true)
                }
            }
        }
        ret
    }

    fn set(&mut self, row: i32, col: i32, val: bool) {
        assert!(row >= 0 && row < self.height);
        assert!(col >= 0 && col < self.width);
        self.bits[((row * self.width) + col) as usize] = val;
    }

    fn get(&self, row: i32, col: i32) -> bool {
        if row < 0 || row >= self.height || col < 0 || col >= self.width {
            false
        } else {
            self.bits[((row * self.width) + col) as usize]
        }
    }

    fn count_on_neighbours(&self, row: i32, col: i32) -> usize {
        [
            self.get(row - 1, col - 1),
            self.get(row - 1, col),
            self.get(row - 1, col + 1),
            self.get(row, col - 1),
            self.get(row, col + 1),
            self.get(row + 1, col - 1),
            self.get(row + 1, col),
            self.get(row + 1, col + 1),
        ]
        .iter()
        .filter(|&b| *b)
        .count()
    }

    fn dump(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.get(row, col) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    fn iterate(&mut self) {
        let mut newbits: Vec<bool> = Vec::new();
        newbits.resize(self.bits.len(), false);
        for row in 0..self.height {
            for col in 0..self.width {
                let neighbours = self.count_on_neighbours(row, col);
                newbits[((row * self.width) + col) as usize] = if self.get(row, col) {
                    neighbours == 2 || neighbours == 3
                } else {
                    neighbours == 3
                };
            }
        }
        std::mem::swap(&mut self.bits, &mut newbits);
    }

    fn count_on(&self) -> usize {
        self.bits.iter().filter(|&b| *b).count()
    }

    fn set_bad(&mut self) {
        self.set(0, 0, true);
        self.set(0, self.width - 1, true);
        self.set(self.height - 1, 0, true);
        self.set(self.height - 1, self.width - 1, true);
    }
}

fn do_iterate(input: &Grid, count: usize, report: bool, badgrid: bool) -> usize {
    let mut mygrid = input.clone();
    if badgrid {
        mygrid.set_bad()
    }
    if report {
        println!("Initial state:");
        mygrid.dump();
    }
    for step in 0..count {
        mygrid.iterate();
        if badgrid {
            mygrid.set_bad();
        }
        if report {
            println!(
                "After {} step{}",
                step + 1,
                if step == 0 { "" } else { "s" }
            );
            mygrid.dump()
        }
    }
    mygrid.count_on()
}

fn main() -> Result<()> {
    let test_input = Grid::from_str(".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n");
    println!("Test 1: {}", do_iterate(&test_input, 4, false, false));
    println!("Test 2: {}", do_iterate(&test_input, 5, false, true));
    let input = Grid::from_str(&read_input(18)?);
    println!("Part 1: {}", do_iterate(&input, 100, false, false));
    println!("Part 2: {}", do_iterate(&input, 100, false, true));
    Ok(())
}
