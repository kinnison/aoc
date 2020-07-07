use aoc2019::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next_direction(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn backwards(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    fn to_num(self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }

    fn moveby(self, pos: (i32, i32)) -> (i32, i32) {
        match self {
            Self::North => (pos.0, pos.1 - 1),
            Self::South => (pos.0, pos.1 + 1),
            Self::West => (pos.0 - 1, pos.1),
            Self::East => (pos.0 + 1, pos.1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellKind {
    Unknown,
    Wall,
    Empty,
    Oxygen,
}

impl std::default::Default for CellKind {
    fn default() -> Self {
        CellKind::Unknown
    }
}

impl From<i64> for CellKind {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Wall,
            1 => Self::Empty,
            2 => Self::Oxygen,
            _ => Self::Unknown,
        }
    }
}

struct RepairBot {
    brain: intcode::VM,
    map: HashMap<(i32, i32), CellKind>,
    pos: (i32, i32),
    oxygen: (i32, i32),
}

impl RepairBot {
    fn new(brain: intcode::VM) -> Self {
        let mut map = HashMap::new();
        map.insert((0, 0), CellKind::Empty);
        Self {
            brain,
            map,
            pos: (0, 0),
            oxygen: (0, 0),
        }
    }

    fn movebot(&mut self, dir: Direction) -> Result<CellKind> {
        // To move the bot, we ask the brain to move
        // and decide based on its output what we need to do
        // The sequence is:
        // 1. run with the direction as input
        // 2. It will produce output status
        // 3. We update our mirror state based on that
        //println!("Stepping robot brain with input: {}", dir.to_num());
        let kind: CellKind = match self.brain.interpreter_step(Some(dir.to_num()))? {
            intcode::VMState::GaveOutput(o) => o.into(),
            eh => return Err(format!("Unexpected brain state: {:?}", eh).into()),
        };
        let newpos = dir.moveby(self.pos);
        match kind {
            CellKind::Unknown => return Err("Unknown cell kind returned?".into()),
            CellKind::Wall => {
                // Update the map in the direction we would have moved
                self.map.insert(newpos, kind);
            }
            CellKind::Empty | CellKind::Oxygen => {
                // Update the map *and* move ourselves
                self.map.insert(newpos, kind);
                self.pos = newpos;
            }
        }
        Ok(kind)
    }

    fn explore(&mut self, find_all: bool) -> Result<usize> {
        // We will explore until we find the oxygen generator
        let mut chain = Vec::new();
        let mut curdir = Direction::North;
        loop {
            //println!("Bot at {:?} exploring {:?}", self.pos, curdir);
            let newpos = curdir.moveby(self.pos);
            if self.map.get(&newpos).is_some() {
                //println!("No point exploring {:?} already been there", newpos);
                // We've already explored that way, time to move on
                curdir = curdir.next_direction();
                if chain.is_empty() && curdir == Direction::North {
                    // We've made it back to the start and the starting direction
                    // so we're done exploring
                    break Ok(0);
                }
                if !chain.is_empty() && curdir == chain[chain.len() - 1] {
                    // We're backing up
                    self.movebot(curdir)?;
                    chain.pop();
                    curdir = curdir.backwards().next_direction();
                }
                continue;
            }
            let kind = self.movebot(curdir)?;
            //println!("Encountered: {:?}", kind);
            match kind {
                CellKind::Wall => {
                    // We hit a wall, so move on to the next direction to
                    // explore.  If the new direction is the chain tip, we're
                    // backtracking...
                    curdir = curdir.next_direction();
                    if !chain.is_empty() && curdir == chain[chain.len() - 1] {
                        // First up, we know we can do this, so do it.
                        self.movebot(curdir)?;
                        // Now pop the chain
                        chain.pop();
                        // And set direction to be the next we would have tried
                        // at the point we're now at
                        curdir = curdir.backwards().next_direction();
                    }
                }
                CellKind::Oxygen => {
                    // We've found the oxygen.  We know we *can* reach it
                    // by the route chain we have
                    self.oxygen = self.pos;
                    if !find_all {
                        break Ok(chain.len() + 1);
                    } else {
                        // Treat this as empty
                        chain.push(curdir.backwards());
                        curdir = curdir.backwards().next_direction();
                    }
                }
                CellKind::Empty => {
                    // We walked forward one block
                    // as such, record in the chain the backward of curdir
                    chain.push(curdir.backwards());
                    // And we reset exploration direction to maximise routes
                    curdir = curdir.backwards().next_direction();
                }
                _ => unimplemented!(),
            }
        }
    }

    fn spread_oxygen(&mut self) -> usize {
        let mut oxygens = vec![self.oxygen];
        let mut opens: HashSet<(i32, i32)> = self
            .map
            .iter()
            .filter_map(|(k, v)| {
                if *v == CellKind::Empty {
                    Some(*k)
                } else {
                    None
                }
            })
            .collect();
        let mut minutes = 0;
        while !opens.is_empty() {
            //println!("Minute {} with {} oxygen sources", minutes, oxygens.len());
            let mut new_oxygens: Vec<(i32, i32)> = Vec::new();
            while let Some(oxygen) = oxygens.pop() {
                for fill in &surrounds(oxygen) {
                    if opens.remove(&fill) {
                        //println!("Spreading oxygen to {:?}", fill);
                        new_oxygens.push(*fill);
                    }
                }
            }
            oxygens = new_oxygens;
            minutes += 1;
        }
        minutes
    }
}

fn part1(input: &intcode::VM) -> Result<usize> {
    let mut bot = RepairBot::new(input.clone());

    bot.explore(false)
}

fn part2(input: &intcode::VM) -> Result<usize> {
    let mut bot = RepairBot::new(input.clone());
    assert_eq!(bot.explore(true)?, 0);
    //println!("Found oxygen generator at {:?}", bot.oxygen);
    // Okay the area is fully explored
    Ok(bot.spread_oxygen())
}

fn main() -> Result<()> {
    let input = read_input(15)?;
    let input = intcode::VM::from_str(&input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);

    Ok(())
}
