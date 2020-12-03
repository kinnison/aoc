use aoc2018::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Blocked,
    Passable,
    Elf { hp: usize, flag: bool },
    Goblin { hp: usize, flag: bool },
}

impl Cell {
    fn as_char(&self) -> char {
        match self {
            Blocked => '#',
            Passable => '.',
            Elf { .. } => 'E',
            Goblin { .. } => 'G',
        }
    }

    fn is_alive(&self) -> bool {
        !matches!(self, Blocked | Passable)
    }

    fn is_elf(&self) -> bool {
        matches!(self, Elf { .. })
    }

    fn is_goblin(&self) -> bool {
        matches!(self, Goblin { .. })
    }

    fn is_passable(&self) -> bool {
        matches!(self, Passable)
    }

    fn is_cave(&self) -> bool {
        matches!(self, Blocked)
    }

    fn is_hostile(&self, is_elf: bool) -> bool {
        match self {
            Blocked | Passable => false,
            Elf { .. } => !is_elf,
            Goblin { .. } => is_elf,
        }
    }

    fn set_flag(&mut self, setflag: bool) {
        match self {
            Blocked | Passable => {}
            Elf { flag, .. } => *flag = setflag,
            Goblin { flag, .. } => *flag = setflag,
        };
    }

    fn get_flag(&mut self) -> bool {
        match self {
            Blocked | Passable => unreachable!(),
            Elf { flag, .. } => *flag,
            Goblin { flag, .. } => *flag,
        }
    }

    fn hp(&self) -> usize {
        match self {
            Blocked | Passable => 0,
            Elf { hp, .. } => *hp,
            Goblin { hp, .. } => *hp,
        }
    }

    fn hit(&mut self, damage: usize) {
        match self {
            Blocked | Passable => unreachable!(),
            Elf { hp, .. } => *hp -= damage,
            Goblin { hp, .. } => *hp -= damage,
        }
    }
}

use self::Cell::*;

#[derive(Debug, Clone)]
struct Cave {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Cave {
    fn from_str(input: &str) -> Result<Cave> {
        let height = input.lines().count();
        if height < 3 {
            return Err("Odd, the cave isn't at least 3 rows!".into());
        }
        let width = input.lines().next().expect("Broken?").len();
        if width < 3 {
            return Err("Odd, the cave isn't at least 3 columns!".into());
        }
        let mut cells = Vec::with_capacity(width * height);
        for c in input.bytes().filter(|&b| b != b'\n') {
            cells.push(match c {
                b'.' => Passable,
                b'#' => Blocked,
                b'E' => Elf {
                    hp: 200,
                    flag: false,
                },
                b'G' => Goblin {
                    hp: 200,
                    flag: false,
                },
                _ => return Err(format!("Unexpected cell '{}'", c as char).into()),
            });
        }
        Ok(Cave {
            cells,
            width,
            height,
        })
    }

    fn display(&self) {
        print!("{:3} ", 0);
        for (i, c) in self.cells.iter().enumerate() {
            print!("{}", c.as_char());
            if (i % self.width) == (self.width - 1) {
                println!();
                if i < (self.height * self.width) - 1 {
                    print!("{:3} ", i + 1);
                }
            }
        }
    }

    fn open_neighbours(&self, pos: usize) -> Vec<usize> {
        let mut ret = Vec::new();
        if !self.cells[pos].is_cave() {
            if self.cells[pos - self.width].is_passable() {
                ret.push(pos - self.width);
            }
            if self.cells[pos - 1].is_passable() {
                ret.push(pos - 1);
            }
            if self.cells[pos + 1].is_passable() {
                ret.push(pos + 1);
            }
            if self.cells[pos + self.width].is_passable() {
                ret.push(pos + self.width);
            }
        }
        ret
    }

    fn enemy_nearby(&self, pos: usize) -> bool {
        let is_elf = self.cells[pos].is_elf();
        self.enemy_nearby_(pos, is_elf)
    }
    fn enemy_nearby_(&self, pos: usize, is_elf: bool) -> bool {
        self.cells[pos - 1].is_hostile(is_elf)
            || self.cells[pos + 1].is_hostile(is_elf)
            || self.cells[pos - self.width].is_hostile(is_elf)
            || self.cells[pos + self.width].is_hostile(is_elf)
    }

    fn find_nearby_enemies(&self, pos: usize) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();
        let is_elf = self.cells[pos].is_elf();
        if self.cells[pos - self.width].is_hostile(is_elf) {
            ret.push(pos - self.width);
        }
        if self.cells[pos - 1].is_hostile(is_elf) {
            ret.push(pos - 1);
        }
        if self.cells[pos + 1].is_hostile(is_elf) {
            ret.push(pos + 1);
        }
        if self.cells[pos + self.width].is_hostile(is_elf) {
            ret.push(pos + self.width);
        }

        ret
    }

    fn creature_move(&mut self, pos: usize) -> usize {
        // First creatures identify if they're next to another creature
        // If so, they don't move
        if self.enemy_nearby(pos) {
            return pos;
        }
        // Right, so we're moving, let's do that...
        let is_elf = self.cells[pos].is_elf();
        // Now we perform a breadth first walk from our position to
        // every possible other position until we find one of the enemies.
        let mut paths = Vec::new();
        paths.push(vec![pos]);
        let mut visited = HashSet::new();
        visited.insert(pos);
        let chosenpath = 'outer: loop {
            if paths.is_empty() {
                // Enemy is unreachable, give up now
                return pos;
            }
            let mut newpaths = Vec::new();
            for basepath in paths.drain(..) {
                let open = self.open_neighbours(*basepath.iter().last().expect("Empty path?"));
                for &next in open.iter() {
                    if visited.contains(&next) {
                        continue;
                    }
                    let mut newpath = basepath.clone();
                    newpath.push(next);
                    if self.enemy_nearby_(next, is_elf) {
                        // this path reaches an enemy, and since we're breadth
                        // first we can be confident that it's the first which
                        // does.
                        break 'outer newpath[1];
                    }
                    // Otherwise we're not there *yet*
                    newpaths.push(newpath);
                    visited.insert(next);
                }
            }
            paths = newpaths;
        };
        // We've chosen chosenpath, so let's make the first move along the path
        let creature = self.cells[pos];
        self.cells[pos] = Passable;
        self.cells[chosenpath] = creature;
        // And the move is done
        chosenpath
    }

    fn creature_fight(&mut self, pos: usize, attack: usize) {
        // Creature at pos is fighting, process it...
        let mut enemies: Vec<(usize, usize)> = self
            .find_nearby_enemies(pos)
            .iter()
            .map(|&p| (self.cells[p].hp(), p))
            .collect();
        enemies.sort_unstable();
        if enemies.is_empty() {
            return;
        }
        let enemy = enemies[0].1;
        if self.cells[enemy].hp() <= attack {
            self.cells[enemy] = Passable;
        } else {
            self.cells[enemy].hit(attack);
        }
    }

    fn count_elves(&self) -> usize {
        self.cells.iter().filter(|c| c.is_elf()).count()
    }

    fn count_goblins(&self) -> usize {
        self.cells.iter().filter(|c| c.is_goblin()).count()
    }

    /// Tick the grid, returns true if the tick completed without either side
    /// winning.
    fn tick(&mut self, elfdamage: usize) -> bool {
        // Ticking involves selecting creatures in turn and running them
        for pos in 0..(self.width * self.height) {
            if self.cells[pos].is_alive() && !self.cells[pos].get_flag() {
                let pos = self.creature_move(pos);
                self.creature_fight(
                    pos,
                    if self.cells[pos].is_elf() {
                        elfdamage
                    } else {
                        3
                    },
                );
                self.cells[pos].set_flag(true);
            }
            if self.count_elves() == 0 || self.count_goblins() == 0 {
                return false;
            }
        }
        // And now clear the flags once more
        for pos in 0..(self.width * self.height) {
            self.cells[pos].set_flag(false);
        }
        true
    }

    fn tick_to_death(&mut self, elfdamage: usize) -> usize {
        // Returns complete rounds before we run out creatures/enemies
        for round in 0.. {
            //println!("Begin round {}", round);
            if !self.tick(elfdamage) {
                // We've not finished the round, but there're no enemies left
                // The outcome is rounds times hp left
                let hpsum: usize = self.cells.iter().map(|c| c.hp()).sum();
                return hpsum * round;
            }
        }
        unreachable!()
    }
}

fn part1(input: &Cave) -> usize {
    let mut cave = input.clone();
    cave.tick_to_death(3)
}

static TESTS_1: &[(usize, &str)] = &[
    (
        36334,
        r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
    ),
    (
        39514,
        r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
",
    ),
    (
        27755,
        r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
",
    ),
    (
        28944,
        r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
",
    ),
    (
        18740,
        r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
",
    ),
];

fn part2(input: &Cave) -> usize {
    // Looking for outcome with lowest elfdamage where elfcount remains stable
    let elfcount = input.count_elves();
    let mut mindamage = 1;
    let mut maxdamage = 200;
    let mut outcomes = HashMap::new();
    loop {
        let trydamage: usize = (maxdamage + mindamage) >> 1;
        let mut newcave = input.clone();
        let outcome = newcave.tick_to_death(trydamage);
        outcomes.insert(trydamage, outcome);
        if newcave.count_elves() == elfcount {
            // No elves died, update min/max
            maxdamage = trydamage;
            if mindamage == maxdamage {
                break outcome;
            }
        } else {
            // some elves died
            mindamage = trydamage;
            if mindamage == maxdamage - 1 {
                break *outcomes.get(&maxdamage).expect("Oddness!");
            }
        }
    }
}

static TESTS_2: &[(usize, &str)] = &[
    (
        4988,
        r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
",
    ),
    (
        31284,
        r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
",
    ),
];

fn main() -> Result<()> {
    for test in TESTS_1.iter() {
        let cave = Cave::from_str(test.1)?;
        assert_eq!(part1(&cave), test.0);
    }
    for test in TESTS_2.iter() {
        let cave = Cave::from_str(test.1)?;
        assert_eq!(part2(&cave), test.0);
    }
    let input = Cave::from_str(&read_input(15)?)?;
    if cfg!(debug_assertions) {
        input.display();
    }
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
