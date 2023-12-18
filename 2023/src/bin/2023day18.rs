use aoc2023::*;

pub fn main() -> Result<()> {
    let input: Vec<Instruction> = read_input_as_vec(18)?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

fn part1(input: &[Instruction]) -> usize {
    let mut plan = DigPlan::from_instrs(input);
    plan.fill();
    plan.grid
        .iter()
        .map(|r| r.iter().copied().filter(|&b| b).count())
        .sum()
}

fn part2(input: &[Instruction]) -> usize {
    let input = input.iter().map(|i| i.transpose()).collect_vec();
    println!("Make plan...");
    let mut plan = DigPlan::from_instrs(&input);
    println!("Fill plan...");
    plan.fill();
    println!("Sum outcome...");
    plan.grid
        .iter()
        .map(|r| r.iter().copied().filter(|&b| b).count())
        .sum()
}

#[derive(Debug, ParseByRegex)]
#[regex = r"(?P<dir>.) (?P<dist>\d+) \(\#(?P<hex>......)\)"]
struct Instruction {
    dir: Facing,
    dist: i32,
    hex: String,
}

impl Instruction {
    fn transpose(&self) -> Self {
        let dist: String = self.hex.chars().take(5).collect();
        let dist = i32::from_str_radix(&dist, 16).unwrap();
        let dir = match self.hex.chars().nth(5).unwrap() {
            '0' => Facing::East,
            '1' => Facing::South,
            '2' => Facing::West,
            '3' => Facing::North,
            _ => unreachable!(),
        };
        Self {
            dir,
            dist,
            hex: String::new(),
        }
    }
}

struct DigPlan {
    grid: Vec<Vec<bool>>,
}

impl DigPlan {
    fn from_instrs(input: &[Instruction]) -> Self {
        let mut dug = HashSet::new();
        let mut row = 0;
        let mut col = 0;
        let mut minrow = 0;
        let mut mincol = 0;
        let mut maxrow = 0;
        let mut maxcol = 0;
        dug.insert((row, col));
        for instr in input {
            println!("Next instruction, so far we have {} dug holes", dug.len());
            let (rofs, cofs) = instr.dir.row_col_offset();
            for _ in 0..instr.dist {
                row += rofs;
                col += cofs;
                dug.insert((row, col));
                minrow = minrow.min(row);
                mincol = mincol.min(col);
                maxrow = maxrow.max(row);
                maxcol = maxcol.max(col);
            }
        }

        let height = (maxrow - minrow) as usize + 1;
        let width = (maxcol - mincol) as usize + 1;
        println!("Creating a {height}x{width} grid, please hold...");

        let mut grid = (0..height + 2)
            .map(|_| std::iter::repeat(false).take(width + 2).collect_vec())
            .collect_vec();

        println!("Grid created, transferring holes");

        let rofs = minrow - 1;
        let cofs = mincol - 1;

        for (row, col) in dug {
            let row = (row - rofs) as usize;
            let col = (col - cofs) as usize;
            grid[row][col] = true;
        }

        println!("Grid made");

        Self { grid }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.grid {
            for col in row {
                if *col {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }

    fn fill(&mut self) {
        let mut dug = vec![vec![true; self.grid[0].len()]; self.grid.len()];
        let mut queue: Vec<(usize, usize)> = vec![(0, 0)];

        let height = dug.len();
        let width = dug[0].len();

        while !queue.is_empty() {
            for (row, col) in std::mem::take(&mut queue) {
                // If out of bounds, skip
                if row >= height || col >= width {
                    continue;
                }
                // Already level, skip
                if !dug[row][col] {
                    continue;
                }
                // Cannot level because already dug by robot
                if self.grid[row][col] {
                    continue;
                }
                // Queue up/down/left/right
                queue.push((row.saturating_sub(1), col));
                queue.push((row, col.saturating_sub(1)));
                queue.push((row + 1, col));
                queue.push((row, col + 1));
                // And level this cell
                dug[row][col] = false;
            }
        }

        self.grid = dug;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)
    "#;

    #[test]
    fn testcase1() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        println!("{input:?}");
        assert_eq!(part1(&input), 62);
    }

    #[test]
    fn testcase2() {
        let input = input_as_vec(TEST_INPUT).unwrap();
        assert_eq!(part2(&input), 952_408_144_115);
    }
}
