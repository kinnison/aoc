use aoc2018::*;

#[derive(ParseByRegex)]
enum InputLine {
    #[regex = r"^depth: (\d+)$"]
    Depth(usize),
    #[regex = r"^target: (\d+), *(\d+)$"]
    Target(usize, usize),
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Kind {
    Rocky,
    Wet,
    Narrow,
}

use self::Kind::*;

impl From<usize> for Kind {
    fn from(num: usize) -> Kind {
        match num % 3 {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => unreachable!(),
        }
    }
}

impl Kind {
    fn risk(self) -> usize {
        match self {
            Rocky => 0,
            Wet => 1,
            Narrow => 2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Input {
    depth: usize,
    target_x: usize,
    target_y: usize,
}

impl Input {
    fn from_input<T: AsRef<str>>(input: T) -> Result<Input> {
        let lines: Vec<InputLine> = input_as_vec(input)?;
        Ok(Input::from_lines(&lines)?)
    }
    fn from_lines(input: &[InputLine]) -> Result<Input> {
        if input.len() != 2 {
            Err("Input is not two lines!")?;
        }
        let depth = match input[0] {
            InputLine::Depth(d) => d,
            _ => Err("First input line is not a depth indicator")?,
        };
        let (target_x, target_y) = match input[1] {
            InputLine::Target(x_, y_) => (x_, y_),
            _ => Err("Second input line is not a target indicator")?,
        };
        Ok(Input {
            depth,
            target_x,
            target_y,
        })
    }
}

struct Cave {
    input: Input,
    erosion: HashMap<(usize, usize), usize>,
}

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

impl Tool {
    fn valid_for(self, k: Kind) -> bool {
        match (self, k) {
            (Neither, Rocky) => false,
            (_, Rocky) => true,
            (Torch, Wet) => false,
            (_, Wet) => true,
            (ClimbingGear, Narrow) => false,
            (_, Narrow) => true,
        }
    }

    fn switch(self, k: Kind) -> Tool {
        match (self, k) {
            (Torch, Rocky) => ClimbingGear,
            (ClimbingGear, Rocky) => Torch,
            (Neither, Wet) => ClimbingGear,
            (ClimbingGear, Wet) => Neither,
            (Neither, Narrow) => Torch,
            (Torch, Narrow) => Neither,
            _ => unreachable!(),
        }
    }
}

use self::Tool::*;
type RouteMap = HashMap<(usize, usize, Tool), usize>;

impl Cave {
    fn new(input: &Input) -> Cave {
        Cave {
            input: *input,
            erosion: HashMap::new(),
        }
    }

    fn get_geologic_index(&mut self, x: usize, y: usize) -> usize {
        if (x == 0 && y == 0) || (x == self.input.target_x && y == self.input.target_y) {
            0
        } else if y == 0 {
            x * 16807
        } else if x == 0 {
            y * 48271
        } else {
            let left = self.get_erosion_level(x - 1, y);
            let up = self.get_erosion_level(x, y - 1);
            left * up
        }
    }

    #[allow(clippy::map_entry)]
    fn get_erosion_level(&mut self, x: usize, y: usize) -> usize {
        if self.erosion.contains_key(&(x, y)) {
            self.erosion[&(x, y)]
        } else {
            let geo = self.get_geologic_index(x, y);
            let erosion = (geo + self.input.depth) % 20183;
            self.erosion.insert((x, y), erosion);
            erosion
        }
    }

    fn get_area_kind(&mut self, x: usize, y: usize) -> Kind {
        self.get_erosion_level(x, y).into()
    }

    fn display_cave(&mut self, width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                if x == 0 && y == 0 {
                    print!("M");
                } else if x == self.input.target_x && y == self.input.target_y {
                    print!("T");
                } else {
                    match self.get_area_kind(x, y) {
                        Rocky => print!("."),
                        Wet => print!("="),
                        Narrow => print!("|"),
                    };
                }
            }
            println!();
        }
    }

    fn primary_risk(&mut self) -> usize {
        let tx = self.input.target_x;
        (0..=self.input.target_y)
            .flat_map(|y| (0..=tx).map(move |x| (x, y)))
            .map(|(x, y)| {
                let k = self.get_area_kind(x, y);
                k.risk()
            })
            .sum()
    }

    fn spelunk(&mut self) -> usize {
        // Our goal is, starting from the mouth (0,0) to reach the target
        // We start with the torch equipped and move from region to region
        // *iff* we can.  We should keep a partial map of the costs of each
        // region so that we can abandon a route if it turns out to be
        // more expensive than previously found
        // Of course, which tool we are holding when we enter matters since
        // it takes time to switch tools.  This means the cache is slightly less
        // useful than it could be
        let mut routes: RouteMap = HashMap::new();
        // When we reach the target along a route, we set this.  If any route
        // ends up larger than best_route before reaching the target we abandon
        // it because it won't help.
        // A theoretical worst cost is the manhattan distance to the target,
        // swapping tools every time (so each step costs 8 to do)
        let mut best_route = (self.input.target_x + self.input.target_y) * 8;
        self.spelunk_(0, 0, Torch, 0, &mut routes, &mut best_route);
        best_route
    }

    fn spelunk_(
        &mut self,
        x: usize,
        y: usize,
        t: Tool,
        l: usize,
        rmap: &mut RouteMap,
        best: &mut usize,
    ) {
        // We are at x,y holding t
        // We have moved for l minutes
        // The best route to the target is of length b
        let hk = self.get_area_kind(x, y);
        let swt = t.switch(hk);
        if let Some(rl) = rmap.get(&(x, y, t)) {
            if *rl <= l {
                // We've been here before, and earlier than now
                return;
            } else {
                // We've not been here before this quickly
                rmap.insert((x, y, t), l);
                let rl = rmap.get(&(x, y, swt)).expect("Oddness!");
                if *rl > l + 7 {
                    rmap.insert((x, y, swt), l + 7);
                }
            }
        } else {
            // We've not been here before, so record the fact for future use
            rmap.insert((x, y, t), l);
            // Also since we can switch on entry, it's the same as if we entered
            // with the switched tool, so record that
            rmap.insert((x, y, swt), l + 7);
        }
        // Now have we already taken longer than the best route?
        if l >= *best {
            return;
        }
        // Now the best route from here to target could be the manhattan distance
        let tx = self.input.target_x as i32;
        let ty = self.input.target_y as i32;
        let ix = x as i32;
        let iy = y as i32;
        let manhat = ((ix - tx).abs() + (iy - ty).abs()) as usize;
        // if l + manhat (the best possible time from here to target) is already
        // greater than the best chance we have, short-circuit.
        if (l + manhat) >= *best {
            return;
        }
        // Okay, we're probably worth spelunking some more...
        if cfg!(debug_assertions) {
            println!("Spelunking! We are at {},{} holding {:?}.  We have taken {} minutes, the best time is {}",
        x,y,t,l,*best);
        }
        // Next, if we're at the target, determine our score
        if x == self.input.target_x && y == self.input.target_y {
            if t == Torch {
                // We're done, we found the target
                *best = min(*best, l);
            } else {
                // Not the torch, so we'd have to switch
                *best = min(*best, l + 7);
            }
            // Either way we've finished our exploration, return
            if cfg!(debug_assertions) {
                println!("Found target, best score is {}", *best);
            }
            return;
        }
        // Now, we can move.  We try each direction once with current tool
        // and if we can't move without switching, we switch and try again
        // We'd prefer to order our tests in ascending manhattan distance
        // to the target, so let's do that...
        let mut offsets = [(-1, 0, x > 0), (0, -1, y > 0), (1, 0, true), (0, 1, true)];
        offsets.sort_by(|a, b| {
            let manhat_a = ((ix + a.0) - tx).abs() + ((iy + a.1) - ty).abs();
            let manhat_b = ((ix + b.0) - tx).abs() + ((iy + b.1) - ty).abs();
            manhat_a.cmp(&manhat_b)
        });

        for ofs in offsets.iter().filter(|v| v.2) {
            // Valid offset pairs are here
            let newx = (ix + ofs.0) as usize;
            let newy = (iy + ofs.1) as usize;
            let ok = self.get_area_kind(newx, newy);
            if t.valid_for(ok) {
                self.spelunk_(newx, newy, t, l + 1, rmap, best);
            } else {
                self.spelunk_(newx, newy, swt, l + 8, rmap, best);
            }
        }
    }
}

static TEST_INPUT: &str = r"
depth: 510
target: 10, 10
";

static TEST_EROSION: &[(usize, usize, usize, Kind)] = &[
    (0, 0, 510, Rocky),
    (1, 0, 17317, Wet),
    (0, 1, 8415, Rocky),
    (1, 1, 1805, Narrow),
    (10, 10, 510, Rocky),
];

fn part1(input: &Input) -> usize {
    let mut cave = Cave::new(&input);
    cave.primary_risk()
}

fn part2(input: &Input) -> usize {
    let mut cave = Cave::new(&input);
    cave.spelunk()
}

fn main() -> Result<()> {
    let test_input = Input::from_input(TEST_INPUT)?;

    {
        let mut cave = Cave::new(&test_input);
        for test in TEST_EROSION {
            let ero = cave.get_erosion_level(test.0, test.1);
            assert_eq!(ero, test.2);
            assert_eq!(cave.get_area_kind(test.0, test.1), test.3);
        }
        if cfg!(debug_assertions) {
            cave.display_cave(16, 16);
        }
    }

    println!("Test 1: {}", part1(&test_input));
    println!("Test 2: {}", part2(&test_input));

    let input = Input::from_input(read_input(22)?)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}
