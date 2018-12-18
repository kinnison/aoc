use aoc2018::*;

#[derive(Debug, Copy, Clone)]
enum Acre {
    Open,
    Wooded,
    Lumberyard,
}

use self::Acre::*;

impl Acre {
    fn from_char(c: char) -> Result<Acre> {
        match c {
            '.' => Ok(Open),
            '|' => Ok(Wooded),
            '#' => Ok(Lumberyard),
            _ => Err(format!("Unknown acre character: '{}'", c))?,
        }
    }

    fn as_char(self) -> char {
        match self {
            Open => '.',
            Wooded => '|',
            Lumberyard => '#',
        }
    }

    fn rule(self, woods: usize, yards: usize) -> Acre {
        match self {
            Open => {
                if woods >= 3 {
                    Wooded
                } else {
                    Open
                }
            }
            Wooded => {
                if yards >= 3 {
                    Lumberyard
                } else {
                    Wooded
                }
            }
            Lumberyard => {
                if (woods > 0) && (yards > 0) {
                    Lumberyard
                } else {
                    Open
                }
            }
        }
    }
}

#[derive(Clone)]
struct Area {
    acres: Vec<Acre>,
    spares: Vec<Acre>,
    width: usize,
    height: usize,
}

impl Area {
    fn from_str(input: &str) -> Result<Area> {
        let input = input.trim();
        let height = input.lines().count();
        if height == 0 {
            Err("No input lines?")?;
        }
        let width = input.lines().next().ok_or("What happened?")?.len();
        if width == 0 {
            Err("First input line is empty!")?;
        }
        let mut spares = Vec::with_capacity(width * height);
        let mut acres = Vec::with_capacity(width * height);
        for c in input.chars().filter(|&c| c != '\n') {
            acres.push(Acre::from_char(c)?);
            spares.push(Acre::Open);
        }
        if acres.len() != width * height {
            Err("Incorrect number of input characters!")?;
        }
        Ok(Area {
            acres,
            width,
            height,
            spares,
        })
    }

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        x + (y * self.width)
    }

    fn get_cell(&self, x: usize, y: usize) -> Acre {
        let idx = self.index(x, y);
        self.acres[idx]
    }

    /// Returns (open, wooded, lumberyard) triple
    fn gather_neighbours(&self, x: usize, y: usize) -> (usize, usize, usize) {
        let minx = if x == 0 { 0 } else { x - 1 };
        let miny = if y == 0 { 0 } else { y - 1 };
        let maxx = if x == self.width - 1 {
            self.width - 1
        } else {
            x + 1
        };
        let maxy = if y == self.height - 1 {
            self.height - 1
        } else {
            y + 1
        };
        let mut open = 0;
        let mut wooded = 0;
        let mut lumberyards = 0;

        for ay in miny..=maxy {
            for ax in minx..=maxx {
                if !(ax == x && ay == y) {
                    match self.get_cell(ax, ay) {
                        Open => open += 1,
                        Wooded => wooded += 1,
                        Lumberyard => lumberyards += 1,
                    }
                }
            }
        }
        (open, wooded, lumberyards)
    }

    fn tick(&mut self) {
        let mut idx = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let (_, wooded, lumberyards) = self.gather_neighbours(x, y);
                self.spares[idx] = self.get_cell(x, y).rule(wooded, lumberyards);
                idx += 1;
            }
        }
        std::mem::swap(&mut self.acres, &mut self.spares);
    }

    fn resource_values(&self) -> (usize, usize, usize) {
        let mut wooded = 0;
        let mut yards = 0;
        for c in self.acres.iter() {
            match c {
                Open => {}
                Wooded => wooded += 1,
                Lumberyard => yards += 1,
            }
        }
        (wooded, yards, wooded * yards)
    }

    fn resource_value(&self) -> usize {
        let (_, _, value) = self.resource_values();
        value
    }

    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_cell(x, y).as_char());
            }
            println!();
        }
    }
}

static TEST_INPUT: &str = r"
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
";

fn part1(input: &Area) -> usize {
    let mut area = input.clone();
    for i in 1..=10 {
        area.tick();
        if cfg!(debug_assertions) {
            println!("After tick {}", i);
            area.display();
        }
    }
    area.resource_value()
}

fn part2(input: &Area) -> usize {
    let mut area = input.clone();
    let mut reshash: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut cyclens: HashMap<usize, usize> = HashMap::new();
    for i in 0..1_000 {
        area.tick();
        let rv = area.resource_values();
        if let Some(previ) = reshash.get(&rv) {
            // We've already seen this score...
            //println!(
            //    "We're at {} and previously it was at {}, cyclelen {}",
            //    i,
            //    previ,
            //    i - previ
            //);
            let cyclelen = i - previ;
            *cyclens.entry(cyclelen).or_insert(0) += 1;
        }
        reshash.insert(rv, i);
    }
    let mut cyclens: Vec<(usize, usize)> = cyclens.iter().map(|(&k, &v)| (k, v)).collect();
    cyclens.sort_by(|(_, c1), (_, c2)| c2.cmp(c1));
    let cyclen = cyclens[0].0;
    let remaining = 1_000_000_000 - 1000;
    let offset = remaining % cyclen;
    for _ in 0..offset {
        area.tick();
    }
    area.resource_value()
}

fn main() -> Result<()> {
    let test_input = Area::from_str(TEST_INPUT)?;
    println!("Test 1: {}", part1(&test_input));
    let input = Area::from_str(&read_input(18)?)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
