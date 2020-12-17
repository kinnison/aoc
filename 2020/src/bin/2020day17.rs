use aoc2020::*;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x, y, z, w }
    }

    fn neighbours(self) -> impl Iterator<Item = Coord> {
        (-1..=1).flat_map(move |x| {
            (-1..=1).flat_map(move |y| {
                (-1..=1).flat_map(move |z| {
                    (-1..=1).filter_map(move |w| {
                        if x != 0 || y != 0 || z != 0 || w != 0 {
                            Some(Coord::new(self.x + x, self.y + y, self.z + z, self.w + w))
                        } else {
                            None
                        }
                    })
                })
            })
        })
    }
}

#[derive(Clone, Debug)]
struct Grid {
    threed: bool,
    active: HashSet<Coord>,
    minx: i32,
    maxx: i32,
    miny: i32,
    maxy: i32,
    minz: i32,
    maxz: i32,
    minw: i32,
    maxw: i32,
}

impl Grid {
    fn load(input: &str, threed: bool) -> Grid {
        let input = input.trim();
        let minx = 0;
        let maxx = (input.lines().next().unwrap().len() as i32) - 1;
        let miny = 0;
        let maxy = (input.lines().count() as i32) - 1;
        let minz = 0;
        let maxz = 0;
        let minw = 0;
        let maxw = 0;

        let mut active = HashSet::new();
        for (y, row) in input.lines().enumerate() {
            let y = y as i32;
            for (x, cell) in row.chars().enumerate() {
                if cell == '#' {
                    active.insert(Coord::new(x as i32, y, 0, 0));
                }
            }
        }
        Grid {
            active,
            minx,
            miny,
            minz,
            maxx,
            maxy,
            maxz,
            minw,
            maxw,
            threed,
        }
    }

    fn set(&mut self, coord: Coord) {
        self.active.insert(coord);
        self.minx = min(self.minx, coord.x);
        self.miny = min(self.miny, coord.y);
        self.minz = min(self.minz, coord.z);
        self.minw = min(self.minw, coord.w);
        self.maxx = max(self.maxx, coord.x);
        self.maxy = max(self.maxy, coord.y);
        self.maxz = max(self.maxz, coord.z);
        self.maxw = max(self.maxw, coord.w);
    }

    fn clear(&mut self, coord: Coord) {
        self.active.remove(&coord);
    }

    fn cycle(&mut self) {
        let start = std::mem::replace(&mut self.active, HashSet::new());
        let (minw, maxw) = if self.threed {
            (0, 0)
        } else {
            (self.minw - 1, self.maxw + 1)
        };
        for w in minw..=maxw {
            for z in (self.minz - 1)..=(self.maxz + 1) {
                for y in (self.miny - 1)..=(self.maxy + 1) {
                    for x in (self.minx - 1)..=(self.maxx + 1) {
                        let pos = Coord::new(x, y, z, w);
                        let is_active = start.contains(&pos);
                        let n_count = pos.neighbours().filter(|c| start.contains(c)).count();
                        match (is_active, n_count) {
                            (false, 3) => self.set(pos),            // Comes alive
                            (false, _) => {}                        // Stays dead
                            (true, 2) | (true, 3) => self.set(pos), // Stays alive
                            (true, _) => self.clear(pos),           // dies
                        }
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::load(input, true);
    for _ in 0..6 {
        grid.cycle();
    }
    grid.active.len()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::load(input, false);
    for _ in 0..6 {
        grid.cycle();
    }
    grid.active.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#".#.
..#
###"#;

    #[test]
    fn testcase1() {
        assert_eq!(part1(TEST_INPUT), 112);
    }

    #[test]
    fn testcase2() {
        assert_eq!(part2(TEST_INPUT), 848);
    }
}

fn main() -> Result<()> {
    let input: String = read_input(17)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
