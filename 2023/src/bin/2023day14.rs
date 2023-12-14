use aoc2023::*;

pub fn main() -> Result<()> {
    let input: Vec<Platform> = read_input_as_chunks(14)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {
    grid: Vec<Vec<Cell>>,
}

impl FromStr for Platform {
    type Err = Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pgrid = s
            .trim()
            .lines()
            .map(|s| s.trim().chars().map(Cell::from_char).collect_vec())
            .collect_vec();

        let mut grid = Vec::new();
        grid.push(vec![Block; pgrid[0].len() + 2]);

        for prow in pgrid {
            grid.push(
                Some(Block)
                    .into_iter()
                    .chain(prow)
                    .chain(Some(Block))
                    .collect_vec(),
            );
        }

        grid.push(grid[0].clone());

        Ok(Self { grid })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Rounded,
    Block,
}

use Cell::*;

impl Cell {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            'O' => Self::Rounded,
            '#' => Self::Block,
            _ => unreachable!(),
        }
    }
}

impl Platform {
    fn roll_dir_once(&mut self, rofs: isize, cofs: isize) -> bool {
        let mut moved = false;
        let cols = self.grid[0].len();
        for rn in 1..(self.grid.len() - 1) {
            for cn in 1..(cols - 1) {
                let tr = (rn as isize + rofs) as usize;
                let tc = (cn as isize + cofs) as usize;
                if self.grid[tr][tc] == Empty && self.grid[rn][cn] == Rounded {
                    self.grid[tr][tc] = Rounded;
                    self.grid[rn][cn] = Empty;
                    moved = true;
                }
            }
        }
        moved
    }

    fn roll_north(&mut self) {
        while self.roll_dir_once(-1, 0) {}
    }

    fn spin_cycle(&mut self) {
        while self.roll_dir_once(-1, 0) {} // North
        while self.roll_dir_once(0, -1) {} // West
        while self.roll_dir_once(1, 0) {} // South
        while self.roll_dir_once(0, 1) {} // East
    }

    fn north_load(&self) -> u64 {
        let mut ret = 0;
        let rows = self.grid.len();
        for (rown, row) in self.grid.iter().enumerate() {
            let row_score = ((rows - rown) as u64) - 1;
            for col in row.iter().copied() {
                if col == Rounded {
                    ret += row_score;
                }
            }
        }
        ret
    }

    #[cfg(test)]
    fn print(&self) {
        for row in self.grid.iter().skip(1).take(self.grid.len() - 2) {
            for col in row.iter().skip(1).take(row.len() - 2) {
                let ch = match col {
                    Empty => '.',
                    Rounded => 'O',
                    Block => '#',
                };
                print!("{ch}");
            }
            println!();
        }
    }
}

fn part1(input: &[Platform]) -> u64 {
    let mut plaf = input[0].clone();
    plaf.roll_north();
    plaf.north_load()
}

fn part2(input: &[Platform]) -> u64 {
    let mut plaf = input[0].clone();

    // Our aim is to determine the load after cycle 1_000_000_000

    // To do this, let's spin the platform until we find a repeat
    let mut cache = HashMap::new();
    cache.insert(plaf.clone(), 0);
    let mut spins: i64 = 0;
    let found_at = loop {
        plaf.spin_cycle();
        spins += 1;
        if !cache.contains_key(&plaf) {
            cache.insert(plaf.clone(), spins);
        } else {
            break cache[&plaf];
        }
    };

    let loop_len = spins - found_at;
    //println!("After {spins} spins, we reached the same state as after {found_at} spins, so every {loop_len} we repeat");
    let mut goal: i64 = 1_000_000_000;

    goal -= found_at;
    //println!("After going {found_at} we need to go another {goal}");
    let loops = goal / loop_len;
    goal -= loops * loop_len;
    //println!("After {loops} loops, we need to go another {goal}");

    for _ in 0..goal {
        plaf.spin_cycle();
    }

    plaf.north_load()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn testcase1() {
        let input: Vec<Platform> = input_as_chunks(TEST_INPUT).unwrap();
        input[0].print();
        assert_eq!(part1(&input), 136);
    }

    #[test]
    fn testcase2() {
        let input: Vec<Platform> = input_as_chunks(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 64);
    }
}
