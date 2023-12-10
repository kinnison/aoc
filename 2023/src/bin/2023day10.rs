use aoc2023::*;

pub fn main() -> Result<()> {
    let input = read_input(10)?;
    let input = parse_map(&input);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[derive(Debug)]
struct Map {
    grid: Vec<String>,
}

fn can_left(v: u8) -> bool {
    matches!(v, b'-' | b'J' | b'7')
}

fn can_right(v: u8) -> bool {
    matches!(v, b'-' | b'F' | b'L')
}

fn can_up(v: u8) -> bool {
    matches!(v, b'|' | b'J' | b'L')
}

fn can_down(v: u8) -> bool {
    matches!(v, b'|' | b'7' | b'F')
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn invert(self) -> Self {
        match self {
            Up => Down,
            Left => Right,
            Down => Up,
            Right => Left,
        }
    }
}

fn next_dir(v: u8, came_from: Direction) -> Direction {
    match (v, came_from) {
        (b'-', Left) => Right,
        (b'-', Right) => Left,
        (b'|', Up) => Down,
        (b'|', Down) => Up,
        (b'L', Up) => Right,
        (b'L', Right) => Up,
        (b'F', Down) => Right,
        (b'F', Right) => Down,
        (b'J', Up) => Left,
        (b'J', Left) => Up,
        (b'7', Left) => Down,
        (b'7', Down) => Left,
        _ => unreachable!(),
    }
}

impl Map {
    fn at(&self, row: i32, col: i32) -> u8 {
        self.grid[row as usize].as_bytes()[col as usize]
    }

    fn find_start(&self) -> (i32, i32) {
        for (ri, row) in self.grid.iter().enumerate() {
            if let Some(col) = row.find('S') {
                return (ri as i32, col as i32);
            }
        }
        unreachable!()
    }

    fn print(&self) {
        for row in self.grid.iter() {
            println!("{row}");
        }
    }

    fn find_path(&self) -> Vec<(i32, i32)> {
        let mut path = Vec::new();
        let (srow, scol) = self.find_start();
        println!("Finding path, starting at {srow},{scol}");

        let (mut nrow, mut ncol, mut came_from) = if can_down(self.at(srow - 1, scol)) {
            (srow - 1, scol, Down)
        } else if can_up(self.at(srow + 1, scol)) {
            (srow + 1, scol, Up)
        } else if can_right(self.at(srow, scol - 1)) {
            (srow, scol - 1, Right)
        } else if can_left(self.at(srow, scol + 1)) {
            (srow, scol + 1, Left)
        } else {
            unreachable!()
        };
        path.push((nrow, ncol));
        while path[path.len() - 1] != (srow, scol) {
            let here = self.at(nrow, ncol);
            let nextdir = next_dir(here, came_from);
            (nrow, ncol) = match nextdir {
                Up => (nrow - 1, ncol),
                Down => (nrow + 1, ncol),
                Left => (nrow, ncol - 1),
                Right => (nrow, ncol + 1),
            };
            came_from = nextdir.invert();
            path.push((nrow, ncol));
        }
        path
    }

    fn ignore_nonpath(&self) -> Self {
        let mut grid = vec![self.grid[0].to_string(); self.grid.len()];
        for (r, c) in self.find_path() {
            unsafe {
                grid[r as usize].as_bytes_mut()[c as usize] = self.at(r, c);
            }
        }
        Self { grid }
    }

    fn blow_up(&self) -> Self {
        let mut grid = Vec::new();
        for row in &self.grid {
            let mut r1 = String::new();
            let mut r2 = String::new();

            for b in row.bytes() {
                let (p1, p2) = match b {
                    b'-' => ("--", ".."),
                    b'|' => ("|.", "|."),
                    b'L' => ("L-", ".."),
                    b'J' => ("J.", ".."),
                    b'7' => ("7.", "|."),
                    b'F' => ("F-", "|."),
                    b'S' => ("SS", "S."),
                    _ => ("..", ".."),
                };
                r1.push_str(p1);
                r2.push_str(p2);
            }

            grid.push(r1);
            grid.push(r2);
        }
        Map { grid }
    }

    fn count_inside(&self) -> usize {
        let mut ret = 0;
        for (ri, row) in self.grid.iter().enumerate() {
            if (ri & 1) == 1 {
                continue;
            }
            for (ci, pos) in row.as_bytes().iter().copied().enumerate() {
                if (ci & 1) == 1 {
                    continue;
                }
                print!("{}", pos as char);
                if pos == b'.' {
                    ret += 1;
                }
            }
            println!();
        }
        ret
    }

    fn set(&mut self, row: i32, col: i32, val: u8) {
        unsafe {
            self.grid[row as usize].as_bytes_mut()[col as usize] = val;
        }
    }
    fn flood_outside(&mut self) {
        let mut look_at = vec![(0, 0)];
        while !look_at.is_empty() {
            for (row, col) in std::mem::take(&mut look_at) {
                let v = self.at(row, col);
                if v != b'.' {
                    continue;
                }
                self.set(row, col, b'O');
                if row > 0 {
                    look_at.push((row - 1, col));
                }
                if row < (self.grid.len() - 1) as i32 {
                    look_at.push((row + 1, col));
                }
                if col > 0 {
                    look_at.push((row, col - 1));
                }
                if col < (self.grid[0].len() - 1) as i32 {
                    look_at.push((row, col + 1));
                }
            }
        }
    }
}

fn parse_map(input: &str) -> Map {
    let l = input.trim().lines().next().unwrap().trim().len();
    let mut grid = Vec::new();
    grid.push(".".repeat(l + 2));
    for l in input.trim().lines() {
        let l = l.trim();
        grid.push(format!(".{l}."));
    }
    grid.push(".".repeat(l + 2));
    Map { grid }
}

fn part1(input: &Map) -> usize {
    input.print();
    let path = input.find_path();

    (path.len() + 1) >> 1
}

fn part2(input: &Map) -> usize {
    let mut map = input.ignore_nonpath().blow_up();
    map.flood_outside();
    map.print();
    map.count_inside()
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

    static TEST_INPUT2: &str = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;

    #[test]
    fn testcase1_1() {
        let input = parse_map(TEST_INPUT);
        eprintln!("{input:?}");
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn testcase2() {
        let input = parse_map(TEST_INPUT2);
        assert_eq!(part2(&input), 4);
    }
}
