use aoc2018::*;

#[derive(Clone, Copy)]
enum CartDirection {
    Up,
    Down,
    Left,
    Right,
}
use self::CartDirection::*;

impl CartDirection {
    fn left(self) -> CartDirection {
        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn right(self) -> CartDirection {
        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }
}

#[derive(Clone, Copy)]
enum CartChoice {
    TurnLeft,
    StraightOn,
    TurnRight,
}

use self::CartChoice::*;

#[derive(Clone, Copy)]
struct Cart {
    x: usize,
    y: usize,
    dir: CartDirection,
    choose: CartChoice,
}

#[derive(Clone)]
struct TrackGrid {
    entries: Vec<u8>,
    width: usize,
    height: usize,
    carts: Vec<Cart>,
}

impl TrackGrid {
    fn from_str(input: &str) -> Result<TrackGrid> {
        let height = input.lines().count();
        if height == 0 {
            Err("No track?")?;
        }
        let width = input.lines().next().expect("No track?").len();
        if width == 0 {
            Err("Track is empty?")?;
        }

        let mut entries: Vec<u8> = Vec::new();
        entries.resize(width * height, b' ');
        let mut ret = TrackGrid {
            entries,
            width,
            height,
            carts: Vec::new(),
        };

        for (y, l) in input.lines().enumerate() {
            for (x, b) in l.bytes().enumerate() {
                match b {
                    b'/' | b'\\' | b'|' | b'-' | b'+' => ret.set_cell(x, y, b),
                    b' ' => {}
                    b'>' | b'<' => {
                        ret.set_cell(x, y, b'-');
                        ret.carts.push(Cart {
                            x,
                            y,
                            dir: if b == b'>' { Right } else { Left },
                            choose: TurnLeft,
                        });
                    }
                    b'^' | b'v' => {
                        ret.set_cell(x, y, b'|');
                        ret.carts.push(Cart {
                            x,
                            y,
                            dir: if b == b'v' { Down } else { Up },
                            choose: TurnLeft,
                        });
                    }
                    _ => Err(format!(
                        "Unknown/unhandlable input character: '{}'",
                        b as char
                    ))?,
                }
            }
        }

        Ok(ret)
    }

    fn set_cell(&mut self, x: usize, y: usize, value: u8) {
        self.entries[x + (y * self.width)] = value;
    }

    fn get_cell(&self, x: usize, y: usize) -> u8 {
        self.entries[x + (y * self.width)]
    }

    fn describe(&self) {
        println!(
            "Track Grid is {} rows {} columns, with {} carts",
            self.height,
            self.width,
            self.carts.len()
        );
    }

    fn tick_carts(&mut self, mode2: bool) -> Result<Option<(usize, usize)>> {
        // Step one, sort the carts into their coordinate order
        self.carts.sort_by(|a, b| {
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });
        let mut olds: HashSet<(usize, usize)> = HashSet::new();
        for c in &self.carts {
            olds.insert((c.x, c.y));
        }
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        let mut carts = std::mem::replace(&mut self.carts, Vec::new());
        loop {
            if carts.is_empty() {
                break;
            }
            let cart = carts.remove(0);
            // Remove this cart from the old set
            olds.remove(&(cart.x, cart.y));
            let (newx, newy) = match cart.dir {
                Up => (cart.x, cart.y - 1),
                Down => (cart.x, cart.y + 1),
                Left => (cart.x - 1, cart.y),
                Right => (cart.x + 1, cart.y),
            };
            let (newdir, newchoice) = match self.get_cell(newx, newy) {
                b'|' | b'-' => (cart.dir, cart.choose),
                b'/' => match cart.dir {
                    Up => (Right, cart.choose),
                    Down => (Left, cart.choose),
                    Left => (Down, cart.choose),
                    Right => (Up, cart.choose),
                },
                b'\\' => match cart.dir {
                    Up => (Left, cart.choose),
                    Down => (Right, cart.choose),
                    Left => (Up, cart.choose),
                    Right => (Down, cart.choose),
                },
                b'+' => match cart.choose {
                    StraightOn => (cart.dir, TurnRight),
                    TurnLeft => (cart.dir.left(), StraightOn),
                    TurnRight => (cart.dir.right(), TurnLeft),
                },
                _ => Err("Unknown track entry!")?,
            };
            let collided = olds.contains(&(newx, newy)) | !seen.insert((newx, newy));
            if collided {
                // Collision
                if mode2 {
                    // Remove the carts at the collision point
                    let oldlen = self.carts.len();
                    self.carts.retain(|c| (c.x, c.y) != (newx, newy));
                    let newlen = self.carts.len();
                    if (oldlen - newlen) != 1 {
                        let oldlen = carts.len();
                        carts.retain(|c| (c.x, c.y) != (newx, newy));
                        let newlen = carts.len();
                        if oldlen == newlen {
                            Err("Unable to find old cart to remove")?;
                        }
                    }
                    seen.remove(&(newx, newy));
                } else {
                    // Give up now
                    return Ok(Some((newx, newy)));
                }
            } else {
                // No collision, so save the cart
                self.carts.push(Cart {
                    x: newx,
                    y: newy,
                    dir: newdir,
                    choose: newchoice,
                });
            }
        }
        // No collisions on this tick
        Ok(None)
    }
}

static TEST_INPUT: &str = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

static TEST_INPUT2: &str = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
";

fn part1(input: &TrackGrid) -> Result<(usize, usize)> {
    // Find when we first splode
    let mut grid = input.clone();
    loop {
        if let Some(coords) = grid.tick_carts(false)? {
            break Ok(coords);
        }
    }
}

fn part2(input: &TrackGrid) -> Result<(usize, usize)> {
    // Find the last cart standing
    let mut grid = input.clone();
    loop {
        grid.tick_carts(true)?;
        if grid.carts.len() == 1 {
            break Ok((grid.carts[0].x, grid.carts[0].y));
        }
        if grid.carts.is_empty() {
            Err("No more carts?")?;
        }
    }
}

fn main() -> Result<()> {
    let test_input = TrackGrid::from_str(TEST_INPUT)?;
    print!("Loaded test: ");
    test_input.describe();
    println!("Test 1: {:?}", part1(&test_input)?);
    let test_input = TrackGrid::from_str(TEST_INPUT2)?;
    print!("Loaded test2: ");
    test_input.describe();
    println!("Test 2: {:?}", part2(&test_input)?);
    let input = TrackGrid::from_str(&read_input(13)?)?;
    print!("Loaded input: ");
    input.describe();
    println!("Part 1: {:?}", part1(&input)?);
    println!("Part 2: {:?}", part2(&input)?);
    Ok(())
}
