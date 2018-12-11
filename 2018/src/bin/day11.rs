use aoc2018::*;

struct FuelGrid {
    cells: Vec<i8>,
    width: usize,
    height: usize,
    serial: usize,
}

impl FuelGrid {
    fn new(width: usize, height: usize, serial: usize) -> FuelGrid {
        let mut cells = Vec::new();
        cells.resize(width * height, 0);
        FuelGrid {
            cells,
            width,
            height,
            serial,
        }
    }

    fn celloffset(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x
    }

    fn calc_power_level(&self, x: usize, y: usize) -> i8 {
        let rackid = x + 10;
        let power = rackid * y;
        let power = power + self.serial;
        let power = power * rackid;
        let power = (power / 100) % 10; // Extract 100s digit
        (power as i8) - 5
    }

    fn set_power_levels(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.celloffset(x, y);
                self.cells[idx] = self.calc_power_level(x, y);
            }
        }
    }

    fn score_square(&self, x: usize, y: usize, size: usize) -> i32 {
        let mut tot: i32 = 0;
        for x_ in x..x + size {
            for y_ in y..y + size {
                let val = self.cells[self.celloffset(x_, y_)];
                tot += i32::from(val);
            }
        }
        tot
    }

    fn find_best_square(&self, size: usize) -> (usize, usize) {
        let mut best = (0, 0);
        let mut bestscore = std::i32::MIN;
        for y in 0..=self.height - size {
            for x in 0..=self.width - size {
                let score = self.score_square(x, y, size);
                if score > bestscore {
                    bestscore = score;
                    best = (x, y);
                }
            }
        }
        best
    }

    fn find_best_arbitrary(&self) -> (usize, usize, usize) {
        let mut best = (0, 0, 1);
        let mut bestscore = std::i32::MIN;
        // Our cutoff here is 30x30 because none of the examples need bigger
        // than half that, so let's just "hope", we'll likely turn out right
        for size in 1..=30 {
            //println!("size: {}", size);
            let (x, y) = self.find_best_square(size);
            let score = self.score_square(x, y, size);
            if score > bestscore {
                best = (x, y, size);
                bestscore = score;
            }
        }
        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT_CELLS: &[(usize, usize, usize, i8)] = &[
        (3, 5, 8, 4),
        (122, 79, 57, -5),
        (217, 196, 39, 0),
        (101, 153, 71, 4),
    ];

    static TEST_INPUT_GRIDS: &[(usize, usize, usize, i32)] = &[(18, 33, 45, 29), (42, 21, 61, 30)];

    #[test]
    fn run_tests() -> Result<()> {
        for test in TEST_INPUT_CELLS {
            let grid = FuelGrid::new(300, 300, test.2);
            assert_eq!(grid.calc_power_level(test.0, test.1), test.3);
        }
        for test in TEST_INPUT_GRIDS {
            let mut grid = FuelGrid::new(300, 300, test.0);
            grid.set_power_levels();
            assert_eq!(test.3, grid.score_square(test.1, test.2, 3));
            let (x, y) = grid.find_best_square(3);
            assert_eq!((x, y), (test.1, test.2));
            assert_eq!(test.3, grid.score_square(x, y, 3));
        }
        Ok(())
    }

    static TEST2: &[(usize, usize, usize, usize, i32)] =
        &[(18, 90, 269, 16, 113), (42, 232, 251, 12, 119)];

    #[test]
    fn run_tests2() {
        for test in TEST2 {
            let mut grid = FuelGrid::new(300, 300, test.0);
            grid.set_power_levels();
            let (x, y, size) = grid.find_best_arbitrary();
            assert_eq!((x, y, size), (test.1, test.2, test.3));
            assert_eq!(grid.score_square(x, y, size), test.4);
        }
    }
}
fn part1(input: usize) -> (usize, usize) {
    let mut grid = FuelGrid::new(300, 300, input);
    grid.set_power_levels();
    grid.find_best_square(3)
}

fn part2(input: usize) -> (usize, usize, usize) {
    let mut grid = FuelGrid::new(300, 300, input);
    grid.set_power_levels();
    grid.find_best_arbitrary()
}

fn main() -> Result<()> {
    let input: usize = read_input(11)?.parse()?;
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
    Ok(())
}
