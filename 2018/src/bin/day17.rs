use aoc2018::*;

#[derive(ParseByRegex)]
enum LineKind {
    #[regex = "^[yY]$"]
    Horizontal,
    #[regex = "^[xX]$"]
    Vertical,
}

use self::LineKind::*;

#[derive(ParseByRegex)]
#[regex = r"^(?P<kind>.)=(?P<basis>\d+), .=(?P<min>\d+)\.\.(?P<max>\d+)$"]
struct InputLine {
    kind: LineKind,
    basis: usize,
    min: usize,
    max: usize,
}

impl InputLine {
    fn limits(&self) -> (usize, usize, usize, usize) {
        match self.kind {
            Horizontal => (self.min, self.basis, self.max, self.basis),
            Vertical => (self.basis, self.min, self.basis, self.max),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    Clay,
    Sand,
    StillWater,
    FlowingWater,
}

use self::Cell::*;

impl Cell {
    fn passable(self) -> bool {
        match self {
            Sand | FlowingWater => true,
            Clay | StillWater => false,
        }
    }

    fn wet(self) -> bool {
        match self {
            Sand | Clay => false,
            StillWater | FlowingWater => true,
        }
    }

    fn as_char(self) -> char {
        match self {
            Sand => '.',
            Clay => '#',
            FlowingWater => '|',
            StillWater => '~',
        }
    }
}

#[derive(Clone)]
struct Landscape {
    grid: Vec<Cell>,
    minx: usize,
    miny: usize,
    maxx: usize,
    maxy: usize,
}

impl Landscape {
    fn new(input: &[InputLine]) -> Landscape {
        let mut minx = std::usize::MAX;
        let mut miny = std::usize::MAX;
        let mut maxx = std::usize::MIN;
        let mut maxy = std::usize::MIN;
        for line in input.iter() {
            let (lminx, lminy, lmaxx, lmaxy) = line.limits();
            minx = min(minx, lminx);
            miny = min(miny, lminy);
            maxx = max(maxx, lmaxx);
            maxy = max(maxy, lmaxy);
        }
        // Since any X coord is valid, grow the X range by 1 in each
        // direction to ensure that we can cope
        minx -= 1;
        maxx += 1;
        // Now move on and build the grid.
        let width = (maxx - minx) + 1;
        let height = (maxy - miny) + 1;
        let mut grid = Vec::with_capacity(width * height);
        grid.resize(width * height, Sand);
        let mut ret = Landscape {
            grid,
            minx,
            maxx,
            miny,
            maxy,
        };

        for line in input.iter() {
            let (lminx, lminy, lmaxx, lmaxy) = line.limits();
            for x in lminx..=lmaxx {
                for y in lminy..=lmaxy {
                    ret.set_cell(x, y, Clay);
                }
            }
        }

        ret
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        (x - self.minx) + ((y - self.miny) * ((self.maxx - self.minx) + 1))
    }

    fn set_cell(&mut self, x: usize, y: usize, content: Cell) {
        let index = self.idx(x, y);
        self.grid[index] = content;
    }

    fn get_cell(&self, x: usize, y: usize) -> Cell {
        let index = self.idx(x, y);
        self.grid[index]
    }

    fn run_pour(&mut self) {
        // Pour always happens from x=500, y=0
        // That means that what we need to do is project down to x=500 y=miny
        // Ideally we don't encounter a blockage at x=500, y=miny
        assert_eq!(self.get_cell(500, self.miny), Sand);
        loop {
            if self.run_pour_(500, self.miny) {
                break;
            }
        }
    }

    /// Returns true if it fell out of the world
    fn run_pour_(&mut self, drop_x: usize, mut y: usize) -> bool {
        // Running a pour is a rough cycle as follows
        // First, try and move down until we're blocked
        //println!("Running a pour from {},{}.", drop_x, y);
        loop {
            // We fell off the bottom of the grid, bail
            self.set_cell(drop_x, y, FlowingWater);
            if y == self.maxy {
                //println!("Fell off the bottom");
                return true;
            }
            if y < self.maxy && self.get_cell(drop_x, y + 1).passable() {
                y += 1;
            } else {
                break;
            }
        }
        //println!("Fell as far as {},{}, projecting left", drop_x, y);
        // Next we project left, recursing every time we can drop
        let mut x = drop_x;
        let mut felloff = false;
        loop {
            // We can't fall off the edge because we've grown the grid
            if y < self.maxy && self.get_cell(x, y + 1).passable() {
                // We can drop down, so pour from here
                if self.run_pour_(x, y) {
                    felloff = true;
                    break;
                }
            } else {
                // We can't drop down, so flood this cell and move left
                self.set_cell(x, y, FlowingWater);
                if self.get_cell(x - 1, y).passable() {
                    x -= 1;
                } else {
                    // We're blocked to the left, so give up.
                    break;
                }
            }
        }
        let minx = x;
        //println!(
        //    "Stopped projecting left at {}, falloff is {}, now projecting right",
        //    x, felloff
        //);
        x = drop_x;
        // Now project right
        loop {
            // We can't fall off the edge because we've grown the grid
            if y < self.maxy && self.get_cell(x, y + 1).passable() {
                // We can drop down, so pour from here
                if self.run_pour_(x, y) {
                    felloff = true;
                    break;
                }
            } else {
                // We can't drop down, so flood this cell and move right
                self.set_cell(x, y, FlowingWater);
                if self.get_cell(x + 1, y).passable() {
                    x += 1;
                } else {
                    // We're blocked to the right, so give up.
                    break;
                }
            }
        }
        let maxx = x;
        //println!("Stopped projecting right at {}, falloff is {}", x, felloff);
        if !felloff {
            // We didn't fall off during our pouring, so flood the plane
            //println!(
            //    "We didn't fall off, so let's fill from {} to {} on {}",
            //    minx, maxx, y
            //);
            for x in minx..=maxx {
                self.set_cell(x, y, StillWater);
            }
        }

        // Finally propagate the falling off
        felloff
    }

    fn count_wet_cells(&self) -> usize {
        self.grid.iter().filter(|&c| c.wet()).count()
    }

    fn count_stillwater_cells(&self) -> usize {
        self.grid.iter().filter(|&c| *c == StillWater).count()
    }

    fn display(&self) {
        for y in self.miny..=self.maxy {
            for x in self.minx..=self.maxx {
                print!("{}", self.get_cell(x, y).as_char());
            }
            println!();
        }
    }
}

static TEST_INPUT: &str = r"
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";

fn part1_and_2(input: &Landscape) -> (usize, usize) {
    let mut land = input.clone();
    land.run_pour();
    (land.count_wet_cells(), land.count_stillwater_cells())
}

fn main() -> Result<()> {
    let test_input: Landscape = Landscape::new(&input_as_vec(TEST_INPUT)?);
    if cfg!(debug_assertions) {
        test_input.display();
    }
    let input: Landscape = Landscape::new(&read_input_as_vec(17)?);
    println!("Test (1, 2): {:?}", part1_and_2(&test_input));
    println!("Real (1, 2): {:?}", part1_and_2(&input));
    Ok(())
}
