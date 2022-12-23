use aoc2022::*;

#[derive(Clone)]
struct ElfMap {
    elves: HashSet<(i32, i32)>,
}

impl<T> From<T> for ElfMap
where
    T: AsRef<str>,
{
    fn from(input: T) -> Self {
        let mut elves = HashSet::new();

        for (row, s) in input.as_ref().trim().lines().map(str::trim).enumerate() {
            for (col, c) in s.chars().enumerate() {
                match c {
                    '#' => {
                        elves.insert((row as i32, col as i32));
                    }
                    '.' => {}
                    c => panic!("Unknown map character {c}"),
                }
            }
        }

        Self { elves }
    }
}

const DIRS: [Facing; 8] = [
    Facing::North,
    Facing::South,
    Facing::West,
    Facing::East,
    Facing::North,
    Facing::South,
    Facing::West,
    Facing::East,
];

const SURROUND: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl ElfMap {
    fn run_round(&mut self, ridx: usize) -> bool {
        let start_dir = ridx % 4;
        let mut proposed = HashMap::new();
        let mut targets = HashMap::new();

        for &(row, col) in &self.elves {
            let worried = SURROUND
                .iter()
                .any(|&(rofs, cofs)| self.elves.contains(&(row + rofs, col + cofs)));
            if !worried {
                // Elf isn't worried, it proposes to stay put
                proposed.insert((row, col), (row, col));
                *targets.entry((row, col)).or_insert(0) += 1;
                continue;
            }
            let mut paralysed = true;
            for dir in &DIRS[start_dir..start_dir + 4] {
                let blocked = match dir {
                    Facing::North => {
                        self.elves.contains(&(row - 1, col))
                            || self.elves.contains(&(row - 1, col - 1))
                            || self.elves.contains(&(row - 1, col + 1))
                    }
                    Facing::East => {
                        self.elves.contains(&(row, col + 1))
                            || self.elves.contains(&(row - 1, col + 1))
                            || self.elves.contains(&(row + 1, col + 1))
                    }
                    Facing::South => {
                        self.elves.contains(&(row + 1, col))
                            || self.elves.contains(&(row + 1, col - 1))
                            || self.elves.contains(&(row + 1, col + 1))
                    }
                    Facing::West => {
                        self.elves.contains(&(row, col - 1))
                            || self.elves.contains(&(row - 1, col - 1))
                            || self.elves.contains(&(row + 1, col - 1))
                    }
                };
                if !blocked {
                    proposed.insert((row, col), dir.do_row_col_move(row, col));
                    *targets.entry(dir.do_row_col_move(row, col)).or_insert(0) += 1;
                    paralysed = false;
                    break;
                }
            }
            if paralysed {
                // Oh no! the poor elf is paralysed and cannot choose!
                proposed.insert((row, col), (row, col));
                *targets.entry((row, col)).or_insert(0) += 1;
            }
        }

        let newelves = proposed
            .into_iter()
            .map(|(s, e)| if targets[&e] == 1 { e } else { s })
            .collect();
        let same_count = self.elves.intersection(&newelves).count();
        self.elves = newelves;
        same_count == self.elves.len()
    }

    fn bounds(&self) -> ((i32, i32), (i32, i32)) {
        let mut minrow = i32::MAX;
        let mut maxrow = i32::MIN;
        let mut mincol = i32::MAX;
        let mut maxcol = i32::MIN;
        for &(row, col) in &self.elves {
            minrow = minrow.min(row);
            mincol = mincol.min(col);
            maxrow = maxrow.max(row);
            maxcol = maxcol.max(col);
        }
        ((minrow, mincol), (maxrow, maxcol))
    }

    fn open_space(&self) -> usize {
        let ((minrow, mincol), (maxrow, maxcol)) = self.bounds();
        let width = 1 + (maxcol - mincol);
        let height = 1 + (maxrow - minrow);
        (width as usize) * (height as usize) - self.elves.len()
    }

    fn print(&self) {
        let ((minrow, mincol), (maxrow, maxcol)) = self.bounds();
        for row in minrow..=maxrow {
            for col in mincol..=maxcol {
                if self.elves.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn part1(input: &ElfMap) -> usize {
    let mut map = input.clone();
    for r in 0..10 {
        map.run_round(r);
    }
    map.open_space()
}

fn part2(input: &ElfMap) -> usize {
    let mut map = input.clone();
    for r in 0.. {
        if map.run_round(r) {
            return r + 1; // after this round, it was all the same
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;

    #[test]
    fn trivial() {
        let mut map = ElfMap::from(
            r#"
.....
..##.
..#..
.....
..##.
....."#,
        );
        map.print();
        map.run_round(0);
        map.print();
        map.run_round(1);
        map.print();
        map.run_round(2);
        map.print();
        panic!("Boo");
    }

    #[test]
    fn testcase1() {
        let input = ElfMap::from(TEST_INPUT);
        assert_eq!(part1(&input), 110);
    }

    #[test]
    fn testcase2() {
        let input = ElfMap::from(TEST_INPUT);
        assert_eq!(part2(&input), 20);
    }
}

pub fn main() -> Result<()> {
    let input = read_input(23)?;
    let input = ElfMap::from(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
