use aoc2022::*;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Absent,
    Open,
    Wall,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ParseByRegex)]
enum Op {
    #[regex = r"(\d+)"]
    Go(usize),
    #[regex = r"L"]
    TurnLeft,
    #[regex = r"R"]
    TurnRight,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn val(self) -> usize {
        match self {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Up => Dir::Left,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }

    fn move1(self, row: usize, col: usize) -> (usize, usize) {
        match self {
            Dir::Right => (row, col + 1),
            Dir::Down => (row + 1, col),
            Dir::Left => (row, col - 1),
            Dir::Up => (row - 1, col),
        }
    }

    fn is_horz(self) -> bool {
        matches!(self, Dir::Right | Dir::Left)
    }
}

#[derive(Clone)]
struct Lock {
    grid: Vec<Vec<Tile>>,
    row_bdr: Vec<(usize, usize)>,
    col_bdr: Vec<(usize, usize)>,
    ops: Vec<Op>,
    warps: HashMap<(usize, usize, Dir), (usize, usize, Dir)>,
}

impl<T> From<T> for Lock
where
    T: AsRef<str>,
{
    #[allow(clippy::needless_range_loop)]
    fn from(input: T) -> Self {
        let input = input.as_ref().trim_end();
        let (cgrid, ops) = input.split_once("\n\n").unwrap();
        let ops = ops
            .chars()
            .group_by(|c| c.is_numeric())
            .into_iter()
            .map(|(_, g)| {
                let s: String = g.collect();
                Op::parse_by_regex(&s).unwrap()
            })
            .collect_vec();

        // The grid is the main event, each line in `grid` represents a potentially partial row, we need to
        // know the full width of the grid, and then we can process each grid line into a vec of vecs
        let height = cgrid.lines().count();
        let width = cgrid.lines().map(str::len).max().unwrap();
        let mut grid = (0..=height + 1)
            .map(|_| (0..=width + 1).map(|_| Tile::Absent).collect_vec())
            .collect_vec();
        for (ridx, row) in cgrid.lines().enumerate() {
            for (cidx, tile) in row.chars().enumerate() {
                grid[ridx + 1][cidx + 1] = match tile {
                    ' ' => Tile::Absent,
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    c => panic!("Unknown tile: {c}"),
                }
            }
        }

        // Next, we compute the row boundaries
        let mut row_bdr = vec![(usize::MAX, usize::MIN)];
        for row in 1..=height {
            let mut first_open = usize::MAX;
            let mut last_open = usize::MIN;
            for col in 1..=width {
                if grid[row][col] != Tile::Absent {
                    first_open = first_open.min(col);
                    last_open = last_open.max(col);
                }
            }
            row_bdr.push((first_open, last_open));
        }

        let mut col_bdr = vec![(usize::MAX, usize::MIN)];
        for col in 1..=width {
            let mut first_open = usize::MAX;
            let mut last_open = usize::MIN;
            for row in 1..=height {
                if grid[row][col] != Tile::Absent {
                    first_open = first_open.min(row);
                    last_open = last_open.max(row);
                }
            }
            col_bdr.push((first_open, last_open));
        }

        let mut warps = HashMap::new();
        // Next compute the warp paths
        for row in 1..=height {
            let (first_open, last_open) = row_bdr[row];
            warps.insert(
                (row, first_open - 1, Dir::Left),
                (row, last_open, Dir::Left),
            );
            warps.insert(
                (row, last_open + 1, Dir::Right),
                (row, first_open, Dir::Right),
            );
        }
        for col in 1..=width {
            let (first_open, last_open) = col_bdr[col];
            warps.insert((first_open - 1, col, Dir::Up), (last_open, col, Dir::Up));
            warps.insert(
                (last_open + 1, col, Dir::Down),
                (first_open, col, Dir::Down),
            );
        }

        Self {
            grid,
            row_bdr,
            col_bdr,
            ops,
            warps,
        }
    }
}

impl Lock {
    fn render(&self) {
        for row in &self.grid {
            for col in row {
                match col {
                    Tile::Absent => print!(" "),
                    Tile::Open => print!("."),
                    Tile::Wall => print!("#"),
                }
            }
            println!();
        }
        for op in &self.ops {
            match op {
                Op::Go(n) => print!("{n}"),
                Op::TurnLeft => print!("L"),
                Op::TurnRight => print!("R"),
            }
        }
        println!();
        println!("Row borders: {:?}", &self.row_bdr[1..]);
        println!("Col borders: {:?}", &self.col_bdr[1..]);
    }

    fn try_walk(
        &self,
        mut row: usize,
        mut col: usize,
        mut direction: Dir,
        count: usize,
    ) -> (usize, usize, Dir) {
        // We're at row,col and we want to go in direction for count steps
        for _ in 0..count {
            // First compute the new coordinate, step one of that is to move in the direction
            let (mut newrow, mut newcol) = direction.move1(row, col);
            // Step two is to warp if we've hit a boundary
            if let Some(&(nr, nc, nd)) = self.warps.get(&(newrow, newcol, direction)) {
                newrow = nr;
                newcol = nc;
                direction = nd;
            }
            // Step three is to check if the new location is open
            match self.grid[newrow][newcol] {
                Tile::Absent => panic!("Walked into an absent tile? {newrow},{newcol}"),
                Tile::Open => {
                    row = newrow;
                    col = newcol;
                }
                Tile::Wall => {
                    break;
                }
            }
        }
        (row, col, direction)
    }

    fn follow_plan(&self) -> usize {
        let mut row = 1;
        let mut col = self.row_bdr[1].0;
        let mut facing = Dir::Right;
        println!("Starting at row={row} col={col}");
        for op in &self.ops {
            println!("At {row}, {col} facing {facing:?} about to {op:?}");
            match op {
                Op::Go(n) => {
                    (row, col, facing) = self.try_walk(row, col, facing, *n);
                }
                Op::TurnLeft => {
                    facing = facing.turn_left();
                }
                Op::TurnRight => {
                    facing = facing.turn_right();
                }
            }
        }

        // Final score is 1000 * row + r * col + facing
        (row * 1000) + (col * 4) + facing.val()
    }
}

fn part1(input: &Lock) -> usize {
    input.render();
    input.follow_plan()
}

fn part2(input: &Lock) -> usize {
    input.render();
    input.follow_plan()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#;

    #[test]
    fn testcase1() {
        let input = Lock::from(TEST_INPUT);
        assert_eq!(part1(&input), 6032);
    }

    #[test]
    fn testcase2() {
        let input = Lock::from(TEST_INPUT);
        assert_eq!(part2(&input), 5031);
    }
}

pub fn main() -> Result<()> {
    let input = read_input(22)?;
    let input = Lock::from(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}
