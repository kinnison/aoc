use aoc2018::*;

struct FuelGrid {
    cells: Vec<i8>,
    sat: Vec<i32>,
    width: usize,
    height: usize,
    serial: usize,
}

impl FuelGrid {
    fn new(width: usize, height: usize, serial: usize) -> FuelGrid {
        let mut cells = Vec::new();
        let mut sat = Vec::new();
        cells.resize(width * height, 0);
        sat.resize((width + 1) * (height + 1), 0);
        FuelGrid {
            cells,
            sat,
            width,
            height,
            serial,
        }
    }

    fn celloffset(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x
    }

    fn satoffset(&self, x: i32, y: i32) -> usize {
        (x + 1) as usize + ((y + 1) as usize * (self.width + 1))
    }

    fn calc_power_level(&self, x: usize, y: usize) -> i8 {
        let rackid = x + 10;
        let power = rackid * y;
        let power = power + self.serial;
        let power = power * rackid;
        let power = (power / 100) % 10; // Extract 100s digit
        (power as i8) - 5
    }

    fn fill_sat(&mut self, x: i32, y: i32, lvl: i8) {
        // SAT for x,y is val(x,y)+val(x-1,y)+val(x,y-1)-val(x-1,y-1)
        let thisidx = self.satoffset(x, y);
        self.sat[thisidx] = i32::from(lvl)
            + self.sat[self.satoffset(x - 1, y)]
            + self.sat[self.satoffset(x, y - 1)]
            - self.sat[self.satoffset(x - 1, y - 1)];
    }

    fn set_power_levels(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.celloffset(x, y);
                let level = self.calc_power_level(x, y);
                self.cells[idx] = level;
                self.fill_sat(x as i32, y as i32, level);
            }
        }
    }

    fn score_square(&self, x: usize, y: usize, size: usize) -> i32 {
        let x = x as i32;
        let y = y as i32;
        let size = size as i32;
        // With SAT, the score of a square is the score of the bottom right
        // plus the score above/left top/left
        // minus the score above top/right
        // minus the score left of left/bottom
        let bottomright = self.sat[self.satoffset(x + size - 1, y + size - 1)];
        let topperlefter = self.sat[self.satoffset(x - 1, y - 1)];
        let bottomlefter = self.sat[self.satoffset(x - 1, y + size - 1)];
        let topperright = self.sat[self.satoffset(x + size - 1, y - 1)];
        bottomright + topperlefter - bottomlefter - topperright
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
        for size in 1..=self.width {
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

fn test_sat() {
    let mut grid = FuelGrid::new(5, 5, 5);
    grid.set_power_levels();
    println!("Grid:");
    for y in 0..5 {
        for x in 0..5 {
            print!("{:3} ", grid.cells[grid.celloffset(x, y)]);
        }
        println!();
    }
    println!("Sats:");
    for y in 0..5 {
        for x in 0..5 {
            print!("{:4} ", grid.sat[grid.satoffset(x, y)]);
        }
        println!();
    }
    for y in 0..5 {
        for x in 0..5 {
            assert_eq!(
                grid.score_square(x, y, 1),
                i32::from(grid.cells[grid.celloffset(x, y)])
            );
        }
    }
    println!("Score of 3x3 at 1,1 is: {}", grid.score_square(1, 1, 3));
}

static TEST_INPUT_CELLS: &[(usize, usize, usize, i8)] = &[
    (3, 5, 8, 4),
    (122, 79, 57, -5),
    (217, 196, 39, 0),
    (101, 153, 71, 4),
];

static TEST_INPUT_GRIDS: &[(usize, usize, usize, i32)] = &[(18, 33, 45, 29), (42, 21, 61, 30)];

fn run_tests() {
    println!("Run part 1 tests");
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
}

static TEST2: &[(usize, usize, usize, usize, i32)] =
    &[(18, 90, 269, 16, 113), (42, 232, 251, 12, 119)];

fn run_tests2() {
    println!("Run part 2 tests");
    for test in TEST2 {
        let mut grid = FuelGrid::new(300, 300, test.0);
        grid.set_power_levels();
        let (x, y, size) = grid.find_best_arbitrary();
        assert_eq!((x, y, size), (test.1, test.2, test.3));
        assert_eq!(grid.score_square(x, y, size), test.4);
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
    test_sat();
    run_tests();
    run_tests2();
    let input: usize = read_input(11)?.parse()?;
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
    Ok(())
}
